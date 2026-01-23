use anyhow::Context as _;
use aya::programs::{Xdp, XdpFlags, TracePoint};
use aya::maps::HashMap;
// use aya::{include_bytes_aligned, Ebpf};
use std::net::Ipv4Addr;
use clap::Parser;
#[rustfmt::skip]
use log::{debug, warn, info};
use tokio::signal;
use tokio::time::{self, Duration};


// use aya::Pod; // You might need this import

//Shared memory layout:
#[repr(C)]
#[derive(Clone, Copy)]
struct EventData {
    event_type: u32,
    data_one: u32,
    data_two: u32,
}
// Manually implement Pod
unsafe impl aya::Pod for EventData {}

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long, default_value = "enp3s0")]
    iface: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

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
    let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/security_monitor"
    )))?;
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
    // let Opt { iface } = opt;
    // let program: &mut Xdp = ebpf.program_mut("security_monitor").unwrap().try_into()?;
    // program.load()?;
    // program.attach(&iface, XdpFlags::default())
    //     .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;
    //
    // let ctrl_c = signal::ctrl_c();
    // println!("Waiting for Ctrl-C...");
    // ctrl_c.await?;
    // println!("Exiting...");
    //
    // Ok(())
    let Opt { iface } = opt;

    // --- Program 1: XDP ---
    let xdp_prog: &mut Xdp = ebpf.program_mut("xdp_firewall").context("xdp not found")?.try_into()?;
    xdp_prog.load()?;
    xdp_prog.attach(&iface, XdpFlags::default())
        .context("failed to attach XDP")?;

    // --- Program 2: Tracepoint ---
    let tp_prog: &mut TracePoint = ebpf.program_mut("socket_connect").context("tp not found")?.try_into()?;
    tp_prog.load()?;
    tp_prog.attach("syscalls", "sys_enter_connect")
        .context("failed to attach Tracepoint")?;

    // --- Program 3: Second trace point 
    let prog: &mut TracePoint = ebpf.program_mut("handle_execve")
        .context("tracepoint handle_execve not found")?
        .try_into()?;
    prog.load()?;
    // Attach to the stable syscalls:sys_enter_execve tracepoint
    prog.attach("syscalls", "sys_enter_execve")
        .context("failed to attach execve tracepoint")?; 
    info!("Monitoring process execution via Tracepoint");

    // 4. Access the Shared Map
    //let events: HashMap<_, u32, EventData> = HashMap::try_from(ebpf.map("EVENTS").context("map not found")?)?;
    let events: HashMap<_, u32, EventData> = HashMap::try_from(ebpf.map("EVENTS").unwrap())?;

    println!("Monitoring active on {}. Press Ctrl-C to exit.", iface);

    // 5. Polling Interval Logic
    let mut interval = time::interval(Duration::from_millis(1000));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                // Poll each specific key we use as an ID (1, 2, 3)
                for id in 1..=3 {
                    if let Ok(event) = events.get(&id, 0) {
                        match event.event_type {
                            1 => {
                                let ip = Ipv4Addr::from(event.data_one);
                                println!("[NET] Ingress from IP: {}, Port: {}", ip, event.data_two);
                            }
                            2 => println!("[SOCK] Connection attempt by PID: {}", event.data_one),
                            3 => println!("[EXEC] New process created by PID: {}", event.data_one),
                            _ => {}
                        }
                    }
                }
            }
            _ = signal::ctrl_c() => {
                println!("Exiting...");
                break;
            }
        }
    }

    Ok(())
}
