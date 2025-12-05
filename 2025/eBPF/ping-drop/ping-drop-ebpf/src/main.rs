#![no_std]
#![no_main]
#![allow(nonstandard_style, dead_code)]

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::HashMap,
    programs::XdpContext,
};
use aya_log_ebpf::info;

use core::mem;
use network_types::{
    eth::{EthHdr, EtherType},
    ip::{IpProto, Ipv4Hdr},
};
// Define a threshold for what constitutes a "large" ping packet in bytes (e.g., 512 bytes)
const LARGE_PACKET_THRESHOLD: u16 = 512;

//use network_types::ip::{IpProto, Ipv4Hdr}; // Import IpProto
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[map] // (1)
static BLOCKLIST: HashMap<u32, u32> = HashMap::<u32, u32>::with_max_entries(1024, 0);

#[xdp]
pub fn ping_drop(ctx: XdpContext) -> u32 {
    match try_ping_drop(ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_ABORTED,
    }
}

#[inline(always)]
unsafe fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let (start, end) = (ctx.data(), ctx.data_end());
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    let ptr = (start + offset) as *const T;
    Ok(unsafe { &*ptr })
}

// (2)
fn block_ip(address: u32) -> bool {
    unsafe { BLOCKLIST.get(&address).is_some() }
}

// fn try_xdp_firewall(ctx: XdpContext) -> Result<u32, ()> {
//     let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };
//     match unsafe { (*ethhdr).ether_type() } {
//         Ok(EtherType::Ipv4) => {}
//         _ => return Ok(xdp_action::XDP_PASS),
//     }
//
//     let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
//     let source = u32::from_be_bytes(unsafe { (*ipv4hdr).src_addr });
//
//     // (3)
//     let action = if block_ip(source) {
//         xdp_action::XDP_DROP
//     } else {
//         xdp_action::XDP_PASS
//     };
//     info!(&ctx, "SRC: {:i}, ACTION: {}", source, action);
//
//     Ok(action)
// }

// ... (Rest of the file remains the same) ...

fn try_ping_drop(ctx: XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(&ctx, 0)? };
    match unsafe { (*ethhdr).ether_type() } {
        Ok(EtherType::Ipv4) => {}
        _ => return Ok(xdp_action::XDP_PASS),
    }

    let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(&ctx, EthHdr::LEN)? };
    let source = u32::from_be_bytes(unsafe { (*ipv4hdr).src_addr });
    // (1)
    // NEW: Get the protocol type from the IPv4 header
    let protocol = unsafe { (*ipv4hdr).proto };

    //(2) Get the total length (must convert from network byte order)
    //Use u16::from_be_bytes() instead of u16::from_be(), since you are converting from a 2-byte array:
    let total_len = u16::from_be_bytes(unsafe { (*ipv4hdr).tot_len });

    // (3) Updated Logic: Check the blocklist AND the protocol
    let action = if block_ip(source) {
        // IP is blocked. Now, only drop if the traffic is ICMP.

        // IpProto::Icmp is the constant for protocol value 1
        if protocol == IpProto::Icmp {
            // Drop ICMP packets from blocked IPs
            xdp_action::XDP_DROP
        } else {
            // Allow all other traffic (TCP, UDP, etc.) from blocked IPs
            xdp_action::XDP_PASS
        }
    } else {
        xdp_action::XDP_PASS
        // IP is not blocked, so pass
        // if total_len > LARGE_PACKET_THRESHOLD {
        //     // Drop large ICMP packets from ANY source
        //     xdp_action::XDP_DROP
        // } else {
        //     xdp_action::XDP_PASS
        // }
    };

    // aya-log-ebpf macro is primarily designed to handle basic numeric types like u64 and network addr
    // (like {:i} for IP addresses).
    // Since protocol is a simple integer value (u8 or u16 depending on the network-types definition,
    // but small enough to fit in a u64), you must explicitly cast it to a u64 before passing it to the macro.
    info!(
        &ctx,
        "SRC: {:i}, PROTO: {}, LEN: {}, ACTION: {}",
        source,
        protocol as u64,
        total_len as u64,
        action // "SRC: {:i}, ACTION: {}",
               // source,
               // action
    );

    Ok(action)
}
