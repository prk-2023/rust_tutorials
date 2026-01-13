#![no_std]
#![no_main]

use aya_ebpf::bindings::task_struct;
use aya_ebpf::helpers::bpf_printk;
use aya_ebpf::helpers::{bpf_get_current_comm, bpf_get_current_pid_tgid, bpf_get_current_task};
use aya_ebpf::{macros::kprobe, programs::ProbeContext};
use aya_log_ebpf::info;

#[kprobe]
pub fn task_struct(ctx: ProbeContext) -> u32 {
    match try_task_struct(ctx) {
        Ok(ret) => ret,
        Err(ret) => ret,
    }
}

fn try_task_struct(ctx: ProbeContext) -> Result<u32, u32> {
    // info!(&ctx, "kprobe called");
    let task = unsafe { bpf_get_current_task() as *const task_struct };
    if task.is_null() {
        return Err(1);
    }
    // Read pid and tgid
    let tgid = (bpf_get_current_pid_tgid() >> 32) as u32;
    let pid = bpf_get_current_pid_tgid() as u32;
    // Read comm (task name)
    //let mut comm = [0u8; 16];
    let comm = bpf_get_current_comm();
    info!(&ctx, "pid={} tgid={}", pid, tgid);
    unsafe { bpf_printk!(b"comm: %s\n\0", &comm as *const _ as *const u8) };
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
