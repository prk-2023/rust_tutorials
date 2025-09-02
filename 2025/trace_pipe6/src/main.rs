use clap::Parser;
use colored::*;
use ctrlc;
use std::{
    fs::{self, File, OpenOptions},
    io::{self, BufRead, BufReader, Write},
    path::PathBuf,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};

#[derive(Parser, Debug)]
#[command(author, version, about = "Tracepoint monitor using tracefs")]
struct Args {
    /// Tracepoints to monitor (format: subsystem:event)
    #[arg()]
    tracepoints: Vec<String>,

    /// List available events in a subsystem and exit
    #[arg(long)]
    list: Option<String>,

    /// Filter by PID (only show events from this PID)
    #[arg(long)]
    pid: Option<u32>,

    /// Write output to this file (in addition to stdout)
    #[arg(long)]
    out: Option<PathBuf>,
}

const TRACEFS_BASE: &str = "/sys/kernel/debug/tracing";

fn list_events(subsystem: &str) -> io::Result<()> {
    let path = format!("{TRACEFS_BASE}/events/{subsystem}");
    let entries = fs::read_dir(&path)?;

    println!("Available events in subsystem '{}':", subsystem);
    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                println!("  {}", name);
            }
        }
    }

    Ok(())
}

fn enable_tracepoint(subsystem: &str, event: &str) -> io::Result<()> {
    let path = format!("{TRACEFS_BASE}/events/{subsystem}/{event}/enable");
    fs::write(path, "1")?;
    Ok(())
}

fn disable_tracepoint(subsystem: &str, event: &str) -> io::Result<()> {
    let path = format!("{TRACEFS_BASE}/events/{subsystem}/{event}/enable");
    fs::write(path, "0")?;
    Ok(())
}

fn apply_pid_filter(pid: u32) -> io::Result<()> {
    let path = format!("{TRACEFS_BASE}/set_ftrace_pid");
    let mut file = File::create(path)?;
    writeln!(file, "{}", pid)?;
    println!("{} {}", "Applied PID filter for:".blue(), pid);
    Ok(())
}

fn parse_tracepoints(raw: &[String]) -> Vec<(String, String)> {
    let mut result = Vec::new();
    for tp in raw {
        match tp.split_once(':') {
            Some((sub, ev)) => result.push((sub.to_string(), ev.to_string())),
            None => {
                eprintln!(
                    "{} '{}'. {}",
                    "Invalid tracepoint format".red(),
                    tp,
                    "Expected format: subsystem:event".yellow()
                );
                std::process::exit(1);
            }
        }
    }
    result
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    // --list mode
    if let Some(subsystem) = args.list.as_deref() {
        return list_events(subsystem);
    }

    if args.tracepoints.is_empty() {
        eprintln!(
            "{}",
            "No tracepoints provided. Example: ./trace_pipe syscalls:sys_enter_open sched:sched_switch".red()
        );
        std::process::exit(1);
    }

    if !PathBuf::from(TRACEFS_BASE).exists() {
        eprintln!(
            "{}",
            "tracefs is not mounted or accessible. Is debugfs mounted?".red()
        );
        std::process::exit(1);
    }

    // Parse tracepoints into (subsystem, event) pairs
    let tracepoints = parse_tracepoints(&args.tracepoints);

    // Optional PID filter
    if let Some(pid) = args.pid {
        apply_pid_filter(pid)?;
    }

    // Open output file if requested
    let mut output_file = if let Some(path) = &args.out {
        let file = OpenOptions::new().create(true).append(true).open(path)?;
        println!("{} {:?}", "Logging trace output to:".blue(), path);
        Some(file)
    } else {
        None
    };

    // Enable all tracepoints
    for (sub, ev) in &tracepoints {
        enable_tracepoint(sub, ev).map_err(|e| {
            eprintln!("{} {}:{}", "Failed to enable tracepoint:".red(), sub, ev);
            e
        })?;
        println!("{} {}:{}", "Enabled tracepoint:".green(), sub, ev);
    }

    let running = Arc::new(AtomicBool::new(true));
    let tracepoints_arc = Arc::new(tracepoints.clone());

    // Setup Ctrl+C cleanup
    let r = running.clone();
    let tp_clone = tracepoints_arc.clone();
    ctrlc::set_handler(move || {
        eprintln!("{}", "\nCaught signal, cleaning up...".yellow());
        for (sub, ev) in tp_clone.iter() {
            let _ = disable_tracepoint(sub, ev);
        }
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    // Read from trace_pipe
    let trace_pipe = File::open(format!("{TRACEFS_BASE}/trace_pipe"))?;
    let mut reader = BufReader::new(trace_pipe);

    println!("{}", "Listening to trace_pipe...".blue());

    loop {
        if !running.load(Ordering::SeqCst) {
            break;
        }

        let mut line = String::new();
        match reader.read_line(&mut line) {
            Ok(0) => break,
            Ok(_) => {
                print!("{}", line.cyan());

                if let Some(file) = output_file.as_mut() {
                    let _ = file.write_all(line.as_bytes());
                }
            }
            Err(e) => {
                if e.kind() == io::ErrorKind::Interrupted {
                    continue;
                } else {
                    return Err(e);
                }
            }
        }
    }

    println!("{}", "Program terminated gracefully.".green());

    Ok(())
}
