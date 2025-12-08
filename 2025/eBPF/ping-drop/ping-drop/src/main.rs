use anyhow::{Context, Result};
use aya::{
    maps::HashMap,
    programs::{Xdp, XdpFlags}
};
use clap::Parser;
//#[rustfmt::skip]
use log::{debug, warn, info};
use tokio::signal;
use aya_log::EbpfLogger;
// use std::net::Ipv4Addr;
use std::{fs::File, io::{BufRead, BufReader},net::Ipv4Addr};

// crates: IP_address list as  Command argument 

#[derive(Debug, Parser)]
struct Opt {
    // Network Interface default eth0
    #[clap(short, long, default_value = "eth0")]
    iface: String,

    //Comma separated list of IPV4 address to block 
    #[clap(long)]
    block: Option<String>,

    //File containing  List of blocked ip address on each line
    #[clap(long)]
    ip_file: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let opt = Opt::parse();
    env_logger::init();

    // Load eBPF-Object :
    // This will include your eBPF object file as raw bytes at compile-time and load it at
    // runtime. This approach is recommended for most real-world use cases. If you would
    // like to specify the eBPF program at runtime rather than at compile-time, you can
    // reach for `Bpf::load_file` instead.
    let mut ebpf = aya::Ebpf::load(aya::include_bytes_aligned!(concat!(
        env!("OUT_DIR"),
        "/ping-drop"
    )))?;

    match EbpfLogger::init(&mut ebpf) {
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

    // Attach XDP Program:
    //let Opt { iface } = opt;
    let program: &mut Xdp = ebpf.program_mut("ping_drop").unwrap().try_into()?;
    program.load()?;
    program.attach(&opt.iface, XdpFlags::default())
        .context("failed to attach the XDP program with default flags - try changing XdpFlags::default() to XdpFlags::SKB_MODE")?;

    // // (1)
    // let mut blocklist: HashMap<_, u32, u32> =
    //     HashMap::try_from(ebpf.map_mut("BLOCKLIST").unwrap())?;
    // // (2)
    // let block_addr: u32 = Ipv4Addr::new(10, 10, 10, 27).into();
    // // (3)
    // blocklist.insert(block_addr, 0, 0)?;
    // // (1)
    // let mut blocklist1: HashMap<_, u32, u32> =
    //     HashMap::try_from(ebpf.map_mut("BLOCKLIST").unwrap())?;
    // // (2)
    // let block_addr1: u32 = Ipv4Addr::new(172, 21, 182, 108).into();
    // // (3)
    // blocklist1.insert(block_addr1, 0, 0)?;
    //Open BLOCKLIST map:
    let mut blocklist: HashMap<_,u32,u32> =
        HashMap::try_from(ebpf.map_mut("BLOCKLIST").unwrap())?;

    // -----------------------------------------------------
    // Load IP Address List  from --block <ips>
    // -----------------------------------------------------
    if let Some(list) = opt.block {
        for ip_str in list.split(',') {
            let trimmed = ip_str.trim();
            if trimmed.is_empty() {
                continue;
            }
            match trimmed.parse::<Ipv4Addr>() {
                Ok(ip) => { 
                    let key: u32 = ip.into();
                    blocklist.insert(key,1,0)?;
                    info!("Adding Blocked IP (CLI): {}", ip);
                }
                Err(_) => warn!("Invalid IP in --block {}", trimmed),
            }
        }
    }
    // -----------------------------------------------------
    // Load IP Address List  from --ip_file <file_path>
    // -----------------------------------i------------------
    if let Some(path) = opt.ip_file {
        let f = File::open(&path).with_context(|| format!("Failed to open file!!: {}", path))?;
        let reader = BufReader::new(f);
        
        for line in reader.lines() {
            let line = line?;
            let trimmed = line.trim();

            //Allow comments and blank lines in file
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue; 
            }
            match trimmed.parse::<Ipv4Addr>() {
                Ok(ip) => {
                    let key: u32 = ip.into();
                    blocklist.insert(key, 1, 0)?;
                    info!("Added File with Block IP_addrs: {}",ip);
                }
                Err(_) => warn!("Invalid IP in file {}: {}", path, trimmed),
            }
        }
    }
    let ctrl_c = signal::ctrl_c();
    println!("Waiting for Ctrl-C...");
    ctrl_c.await?;
    println!("Exiting...");

    Ok(())
}
