#![no_std]
#![no_main]

use aya_ebpf::{
    helpers::{bpf_get_current_comm, bpf_get_current_pid_tgid},
    macros::{kprobe, map},
    maps::RingBuf,
    programs::ProbeContext,
};
use aya_log_ebpf::info;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Event {
    pub pid: u32,
    pub tgid: u32,
    pub comm: [u8; 16],
}

#[map]
static MYEVENTS: RingBuf = RingBuf::with_byte_size(4096 * 64, 0);

#[kprobe]
pub fn ringbuffer_map(ctx: ProbeContext) -> u32 {
    match try_ringbuffer_map(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_ringbuffer_map(ctx: ProbeContext) -> Result<u32, u32> {
    // info!(&ctx, "kprobe called");
    let pid_tgid = bpf_get_current_pid_tgid();

    // Optional : reduce log only monitor if the pid is over 1000
    if ((pid_tgid >> 32) as u32) < 9000 {
        return Ok(0);
    }

    // Create the event on the stack
    let mut event = Event {
        pid: (pid_tgid >> 32) as u32,
        tgid: pid_tgid as u32,
        comm: [0u8; 16],
    };

    // Fill the comm field
    if let Ok(comm) = bpf_get_current_comm() {
        event.comm = comm;
    }

    // Output to RingBuf.
    // .reserve() + .submit() is more efficient, but .output() is simpler for examples.
    // let _ = EVENTS.output(&event, 0);
    let _ = MYEVENTS.output::<Event>(&event, 0);

    info!(&ctx, "KPROBE TRIGGERED: PID {}", event.pid);
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
