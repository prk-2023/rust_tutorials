#![no_std]
#![no_main]

use aya_ebpf::{macros::sock_ops, programs::SockOpsContext};
use aya_log_ebpf::info;

#[sock_ops]
pub fn tcp_est(ctx: SockOpsContext) -> u32 {
    match try_tcp_est(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_tcp_est(ctx: SockOpsContext) -> Result<u32, u32> {
    info!(&ctx, "received TCP connection");
    // Which TCP event triggered this callback?
    let op = ctx.op();

    // We want something similar to BPF_SOCK_OPS_ACTIVE_ESTABLISHED_CB
    // which means a TCP client-side connection has just been established.
    const ACTIVE_ESTABLISHED: u32 = 4;

    if op == ACTIVE_ESTABLISHED {
        let local = ctx.local_port();
        let remote = ctx.remote_port();

        info!(
            &ctx,
            "TCP connection established: local_port={} remote_port={}", local, remote
        );
    }

    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

// #[unsafe(link_section = "license")]
// #[unsafe(no_mangle)]
// static LICENSE: [u8; 13] = *b"Dual MIT/GPL\0";
