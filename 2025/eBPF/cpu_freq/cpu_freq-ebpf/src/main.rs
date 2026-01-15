#![no_std]
#![no_main]

use aya_ebpf::{macros::tracepoint, programs::TracePointContext};
use aya_log_ebpf::info;

#[tracepoint]
pub fn cpu_freq(ctx: TracePointContext) -> u32 {
    try_cpu_freq(ctx)
    // match try_cpu_freq(ctx) {
    //     Ok(ret) => ret,
    //     Err(ret) => ret,
    // }
}

fn try_cpu_freq(ctx: TracePointContext) -> u32 {
    info!(&ctx, "tracepoint power called");

    let freq: u32 = unsafe {
        match ctx.read_at(8) {
            Ok(v) => v,
            Err(_) => return 0,
        }
    };

    let cpu: u32 = unsafe {
        match ctx.read_at(12) {
            Ok(v) => v,
            Err(_) => return 0,
        }
    };

    info!(&ctx, "CPU {} frequency changed to {} kHz", cpu, freq);

    //Ok(0)
    0
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
