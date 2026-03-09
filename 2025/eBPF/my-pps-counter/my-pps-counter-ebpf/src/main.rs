#![no_std]
#![no_main]

use aya_ebpf::{
    bindings::xdp_action,
    macros::{map, xdp},
    maps::PerCpuArray,
    programs::XdpContext,
};
// use aya_log_ebpf::info;

#[map]
static COUNTER: PerCpuArray<u64> = PerCpuArray::with_max_entries(1, 0);

#[xdp]
pub fn my_pps_counter(_ctx: XdpContext) -> u32 {
    // Look up the first index in our Per-CPU array
    if let Some(val_ptr) = COUNTER.get_ptr_mut(0) {
        unsafe {
            // Increment the value directly at the pointer
            *val_ptr += 1;
        }
    }

    xdp_action::XDP_PASS
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
