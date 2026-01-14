use aya::programs::KProbe;
use aya::{
    //include_bytes_aligned,
    maps::RingBuf,
    // Ebpf,
};
#[rustfmt::skip]
use log::{debug, info,warn};
use tokio::signal;
use tokio::io::unix::AsyncFd;
use std::ptr;

#[repr(C)]
#[derive(Clone, Copy)]
struct Event {
    pub pid: u32,
    pub gtid: u32,
    pub comm: [u8; 16],
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Bump the memlock rlimit. This is needed for older kernels that don't use the
    // new memcg based accounting, see https://lwn.net/Articles/837122/
    let rlim = libc::rlimit {
        rlim_cur: libc::RLIM_INFINITY,
        rlim_max: libc::RLIM_INFINITY,
    };
    let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    if ret != 0 {
        debug!("remove limit on locked memory failed, ret is: {ret}");
    }

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    //
    // 1. Load eBPF bytecode
    let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/ringbuffer_map"
    )))?;

    // 2. Initialize aya-log
    match aya_log::EbpfLogger::init(&mut ebpf) {
        Err(e) => {
            // This can happen if you remove all log statements from your eBPF program.
            warn!("failed to initialize eBPF logger: {e}");
        }
        Ok(logger) => {
            let mut logger =
                tokio::io::unix::AsyncFd::with_interest(logger, tokio::io::Interest::READABLE)?;
            tokio::task::spawn(async move {
                loop {
                    let mut guard = logger.readable_mut().await.unwrap();
                    guard.get_inner_mut().flush();
                    guard.clear_ready();
                }
            });
        }
    }

    // 3. Attach KProbe
    let program: &mut KProbe = ebpf.program_mut("ringbuffer_map").unwrap().try_into()?;
    program.load()?;
    program.attach("do_sys_openat2", 0)?;

    // 4. Handle Ring BUffer: take ownership to make it 'static 
    let ring_buf_map = ebpf.take_map("MYEVENTS").expect("map MYEVENTS not found QQ ");
    // let ring_buf = RingBuf::try_from(ringbuffer_map_common)?;
    let ring_buf = RingBuf::try_from(ring_buf_map)?;
    let mut async_rb = AsyncFd::new(ring_buf)?;

    //5 Spawn background processor 
    tokio::task::spawn( 
        async move {
            info!("Waiting for events... (Try running 'ls' in another terminal)");
            loop {
                // Wait for data
                let mut guard = match async_rb.readable_mut().await {
                    Ok(g) => g,
                    Err(e) => {
                        warn!("Ring buffer error: {e}");
                        break;
                    }
                };
    
                let rb = guard.get_inner_mut();
                
                // Drain all available items
                while let Some(item) = rb.next() {
                    // Cast raw bytes to Event struct
                    let event = unsafe { ptr::read_unaligned(item.as_ptr() as *const Event) };
                    
                    let comm = String::from_utf8_lossy(&event.comm)
                        .trim_end_matches('\0')
                        .to_string();
    
                    println!("PID: {:<8} | COMM: {}", event.pid, comm);
                }
    
                // Reset readiness for next epoll trigger
                guard.clear_ready();
            }
        }
    );

    let ctrl_c = signal::ctrl_c();
    println!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    println!("Exiting...");

    Ok(())
}
