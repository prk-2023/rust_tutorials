use anyhow::Context as _;
use aya::programs::{Xdp, XdpFlags};
use clap::Parser;
#[rustfmt::skip]
//use log::{debug, warn};
use log::warn;
use tokio::signal;
use aya::maps::PerCpuArray;
use std::time::Duration;
// use tokio::time;

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long, default_value = "eth0")]
    iface: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let opt = Opt::parse();

    env_logger::init();

    // Bump the memlock rlimit. This is needed for older kernels that don't use the
    // new memcg based accounting, see https://lwn.net/Articles/837122/
    // let rlim = libc::rlimit {
    //     rlim_cur: libc::RLIM_INFINITY,
    //     rlim_max: libc::RLIM_INFINITY,
    // };
    // let ret = unsafe { libc::setrlimit(libc::RLIMIT_MEMLOCK, &rlim) };
    // if ret != 0 {
    //     debug!("remove limit on locked memory failed, ret is: {ret}");
    // }

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/my-pps-counter"
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
    let Opt { iface } = opt;
    let program: &mut Xdp = ebpf.program_mut("my_pps_counter").unwrap().try_into()?;
    program.load()?;
    program.attach(&iface, XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    let  counter: PerCpuArray<_, u64> = PerCpuArray::try_from(ebpf.map_mut("COUNTER").unwrap())?;
    let mut last_total = 0u64;

    loop { 
        // time::sleep(Duration::from_secs(1)).await;
        // // Sum values from all CPU cores
        // let cpu_values = counter.get(&0, 0)?; 
        // let current_total: u64 = cpu_values.iter().sum();
        //
        // let pps = current_total.saturating_sub(last_total);
        // println!("PPS: {}", pps);
        // last_total = current_total;
        tokio::select! {
        // Option 1: The timer hits 1 second
        _ = tokio::time::sleep(Duration::from_secs(1)) => {
            let cpu_values = counter.get(&0, 0)?; 
            let current_total: u64 = cpu_values.iter().sum();
            let pps = current_total.saturating_sub(last_total);
            
            println!("PPS: {}", pps);
            last_total = current_total;
        }
        // Option 2: The user presses Ctrl+C
        _ = signal::ctrl_c() => {
            println!("Exiting gracefully...");
            break; // This breaks the loop and allows the program to finish
        }
    }
    }


    Ok(())
}
