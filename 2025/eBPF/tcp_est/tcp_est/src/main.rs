use anyhow::{Context, Result};
use anyhow::Context as _;
use aya::programs::{SockOps, links::CgroupAttachMode};
use clap::Parser;
// #[rustfmt::skip]
use log::{debug, warn};
use tokio::signal;
use aya_log::EbpfLogger;

#[derive(Debug, Parser)]
struct Opt {
    #[clap(short, long, default_value = "/sys/fs/cgroup")]
    cgroup_path: std::path::PathBuf,
}

#[tokio::main]
//async fn main() -> anyhow::Result<()> {
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();

    env_logger::init();

    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/tcp_est"
    )))?;

    if let Err(e) = aya_log::EbpfLogger::init(&mut ebpf) {
        eprintln!("fail to init eBPF logger: {}", e);
    }
    // match EbpfLogger::init(&mut ebpf) {
    //     Err(e) => {
    //         // This can happen if you remove all log statements from your eBPF program.
    //         warn!("failed to initialize eBPF logger: {e}");
    //     }
    //     Ok(logger) => {
    //         let mut logger =
    //             tokio::io::unix::AsyncFd::with_interest(logger, tokio::io::Interest::READABLE)?;
    //         tokio::task::spawn(async move {
    //             loop {
    //                 let mut guard = logger.readable_mut().await.unwrap();
    //                 guard.get_inner_mut().flush();
    //                 guard.clear_ready();
    //             }
    //         });
    //     }
    // }
    let Opt { cgroup_path } = opt;
    let cgroup =
        std::fs::File::open(&cgroup_path).with_context(|| format!("{}", cgroup_path.display()))?;
    let program: &mut SockOps = ebpf.program_mut("tcp_est").unwrap().try_into()?;
    program.load()?;
    program.attach(cgroup, CgroupAttachMode::default())?;

    let ctrl_c = signal::ctrl_c();
    println!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    println!("Exiting...");

    Ok(())
}
