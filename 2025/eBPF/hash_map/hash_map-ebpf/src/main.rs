#![no_std]
#![no_main]

use aya_ebpf::{
    helpers::bpf_probe_read_user_str_bytes,
    macros::{map, tracepoint},
    maps::{HashMap, PerCpuArray},
    programs::TracePointContext,
};
use aya_log_ebpf::info;
use core::str::from_utf8_unchecked;
use hash_map_common::MAX_PATH_LEN;

const FILENAME_OFFSET: usize = 16;
const ZEROED_ARRAY: [u8; MAX_PATH_LEN] = [0u8; MAX_PATH_LEN];

// Use a PerCpuArray as a scratch buffer to stay under the 512-byte stack limit.
#[map]
static MY_BUF: PerCpuArray<[u8; MAX_PATH_LEN]> = PerCpuArray::with_max_entries(1, 0);

#[map]
static EXCLUDED_CMDS: HashMap<[u8; 512], u8> = HashMap::with_max_entries(10, 0);

#[tracepoint]
pub fn hash_map(ctx: TracePointContext) -> u32 {
    match try_hash_map(ctx) {
        Ok(ret) => ret,
        Err(_) => 1,
    }
}

fn try_hash_map(ctx: TracePointContext) -> Result<u32, i64> {
    let buf = MY_BUF.get_ptr_mut(0).ok_or(0)?;

    let filename = unsafe {
        *buf = ZEROED_ARRAY;
        let filename_src_addr = ctx.read_at::<*const u8>(FILENAME_OFFSET)?;
        let filename_bytes = bpf_probe_read_user_str_bytes(filename_src_addr, &mut *buf)?;
        if EXCLUDED_CMDS.get(&*buf).is_some() {
            info!(&ctx, "No log for this binary!!");
            return Ok(0);
        }
        from_utf8_unchecked(filename_bytes)
    };
    info!(&ctx, "Tracepoint sys_enter_execve: {}", filename);

    Ok(0)
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe { core::hint::unreachable_unchecked() }
}
