#![no_std]
#![no_main]

use aya_ebpf::{macros::uprobe, programs::ProbeContext, EbpfContext};
use aya_log_ebpf::info;

#[uprobe]
pub fn uprobe_malloc(ctx: ProbeContext) -> u32 {
    match try_uprobe_malloc(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_uprobe_malloc(ctx: ProbeContext) -> Result<u32, u32> {
    // info!(&ctx, "function malloc called by libc");
    // In x86_64, the 1st argument (size) is in %rdi
    // Aya's  .arg(0) helper handles this register mapping
    let size: usize = ctx.arg(0).ok_or(0u32)?;

    // We can also get the PID of the process calling malloc
    let pid = ctx.pid();

    info!(&ctx, "PID {}: alloc called for {} bytes ", pid, size);
    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[unsafe(link_section = "license")]
#[unsafe(no_mangle)]
static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
