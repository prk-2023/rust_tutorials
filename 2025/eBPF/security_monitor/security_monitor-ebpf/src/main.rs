#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, tracepoint, xdp},
    maps::HashMap,
    programs::{TracePointContext, XdpContext},
};

use aya_log_ebpf::info;
use core::mem;
use network_types::eth::{EthHdr, EtherType};
use network_types::ip::{IpProto, Ipv4Hdr};
use network_types::{tcp::TcpHdr, udp::UdpHdr};

// ADDING: This makes .pid() and other context helpers work
use aya_ebpf::EbpfContext;
use core::ptr::addr_of_mut;

#[repr(C)]
#[derive(Clone, Copy)]

pub struct EventData {
    pub event_type: u32, // 1: XDP, 2: Socket, 3: Exec
    pub data_one: u32,   // IP or PID for IP we strore the raw Ipv4 bits and handle at userspace
    pub data_two: u32,   // Port or 0
}
// Manually implement Pod for the eBPF side too
// unsafe impl aya_ebpf::Pod for EventData {}

#[map]
static mut EVENTS: HashMap<u32, EventData> = HashMap::with_max_entries(1024, 0);
// #[xdp]
// pub fn security_monitor(ctx: XdpContext) -> u32 {
//     match try_security_monitor(ctx) {
//         Ok(ret) => ret,
//         Err(_) => xdp_action::XDP_ABORTED,
//     }
// }
//
// fn try_security_monitor(ctx: XdpContext) -> Result<u32, u32> {
//     info!(&ctx, "received a packet");
//     Ok(xdp_action::XDP_PASS)
// }
// --- 1. XDP Firewall Example ---

#[inline(always)]
unsafe fn ptr_at<T>(ctx: &XdpContext, offset: usize) -> Result<*const T, ()> {
    let start = ctx.data();
    let end = ctx.data_end();
    let len = mem::size_of::<T>();

    if start + offset + len > end {
        return Err(());
    }

    Ok((start + offset) as *const T)
}
// --- 1. Robust XDP firewall
#[xdp]
pub fn xdp_firewall(ctx: XdpContext) -> u32 {
    match try_xdp_firewall(&ctx) {
        Ok(ret) => ret,
        Err(_) => xdp_action::XDP_PASS,
    }
}
fn try_xdp_firewall(ctx: &XdpContext) -> Result<u32, ()> {
    let ethhdr: *const EthHdr = unsafe { ptr_at(ctx, 0)? };

    match unsafe { *ethhdr }.ether_type() {
        Ok(EtherType::Ipv4) => {
            let ipv4hdr: *const Ipv4Hdr = unsafe { ptr_at(ctx, EthHdr::LEN)? };
            // let source_addr = unsafe { (*ipv4hdr).src_addr() };
            // FIX::
            // "no_std" most reliable way to turn those 4 bytes into a u32 for your map is to use
            // u32::from_be_bytes() or u32::from_ne_bytes().
            let source_addr = u32::from_be_bytes(unsafe { (*ipv4hdr).src_addr });

            let source_port = match unsafe { (*ipv4hdr).proto } {
                IpProto::Tcp => {
                    let tcphdr: *const TcpHdr = unsafe { ptr_at(ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    u16::from_be_bytes(unsafe { (*tcphdr).source })
                }
                IpProto::Udp => {
                    let udphdr: *const UdpHdr = unsafe { ptr_at(ctx, EthHdr::LEN + Ipv4Hdr::LEN)? };
                    unsafe { (*udphdr).src_port() }
                }
                _ => 0,
            };

            info!(
                ctx,
                "XDP: IPv4 SRC: {:i}, PORT: {}", source_addr, source_port
            );
            unsafe {
                let _ = (*addr_of_mut!(EVENTS)).insert(
                    &1,
                    &EventData {
                        event_type: 1,
                        data_one: source_addr,
                        data_two: source_port as u32,
                    },
                    0,
                );
            }
        }
        _ => {}
    }
    Ok(xdp_action::XDP_PASS)
}

// --- 2. Tracepoint (Socket Open) ---
// Hooking into the 'sys_enter_connect' tracepoint
//#[tracepoint(name = "socket_connect")]
#[tracepoint]
pub fn socket_connect(ctx: TracePointContext) {
    let pid = ctx.pid();
    unsafe {
        let _ = (*addr_of_mut!(EVENTS)).insert(
            &2,
            &EventData {
                event_type: 2,
                data_one: pid,
                data_two: 0,
            },
            0,
        );
    }
}
// --- 3. Second trace point
// Use the stable tracepoint for execve
#[tracepoint]
pub fn handle_execve(ctx: TracePointContext) {
    let pid = ctx.pid();
    unsafe {
        let _ = (*addr_of_mut!(EVENTS)).insert(
            &3,
            &EventData {
                event_type: 3,
                data_one: pid,
                data_two: 0,
            },
            0,
        );
    }
}
//----------------------------------

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
