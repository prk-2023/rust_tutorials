use clap::Parser;
use colored::*;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[arg(long, num_args(0..=1), default_missing_value ="all")]
    list: Option<String>,

    #[arg(long, num_args(0..=1), help = "event name: dump trace event data format")]
    trace_event: Option<String>,
}

fn main() {
    println!(
        "{}{}",
        "NOTE: Requires ".white().bright_white(),
        "!!!sudo!!!".red().bright_red().blink()
    );
    let args = Args::parse();

    let subsystem = args.list.unwrap_or_else(|| "all".to_string());

    if let Some(fmt_x) = args.trace_event {
        let _x = list_fmt(&subsystem, &fmt_x);
    } else {
        let _x = list_events(&subsystem);
    }
}

const TRACEFS_BASE: &str = "/sys/kernel/debug/tracing";

#[allow(unused_assignments)]
fn list_events(subsystem: &str) -> io::Result<()> {
    // The `if/else` expression returns a String value
    let path = if subsystem.is_empty() {
        format!("{}/events", TRACEFS_BASE)
    } else if subsystem == "all" {
        format!("{}/events", TRACEFS_BASE)
    } else {
        format!("{}/events/{}", TRACEFS_BASE, subsystem)
    };

    // `path` is guaranteed to be a String, which implements AsRef<Path>
    if subsystem == "all" {
        println!(
            "{}",
            "System supported subsystems for tracing: (select the subsystem of interest)".yellow()
        );
    } else {
        println!(
            "{} {} \n",
            "Listed tracing events for subsystem:".yellow().underline(),
            subsystem.bright_white().bold().underline()
        );
    }
    let entries = fs::read_dir(&path)?;
    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                if subsystem == "all" {
                    println!("  {}", name.green().bold());
                } else {
                    println!("  {}", name.bright_cyan().bold().italic());
                }
            }
        }
    }
    Ok(())
}

#[allow(unused_assignments)]
fn list_fmt(subsystem: &str, trace_item: &str) -> io::Result<()> {
    let file_path = format!(
        "{}/events/{}/{}/format",
        TRACEFS_BASE, subsystem, trace_item
    );
    println!("DBG:: {file_path}");

    if let Err(e) = read_file_line_by_line(&file_path) {
        eprintln!("Error reading file: {}", e);
    }
    Ok(())
}

fn read_file_line_by_line(file_path: &str) -> io::Result<()> {
    // Open the file
    let file = File::open(file_path)?;

    // Create a buffered reader for the file
    let reader = BufReader::new(file);

    // Iterate over each line in the file
    for line in reader.lines() {
        let line = line?; // Unwrap the Result from the iterator
        println!("{}", line); // Print each line
    }

    Ok(())
}
