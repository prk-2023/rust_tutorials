use clap::Parser;
use colored::*;
use std::{fs, io};
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    #[arg(long, num_args(0..=1), default_missing_value ="all")]
    list: Option<String>,
}

fn main() {
    println!(
        "{}{}",
        "NOTE: Requires ".white().bright_white(),
        "!!!sudo!!!".red().bright_red().blink()
    );
    let args = Args::parse();

    // --list mode
    if let Some(subsystem) = args.list.as_deref() {
        //return list_events(subsystem);
        let _x = list_events(subsystem);
    } else {
        // If no subsystem argument is passed  list all subsystem
        let _x = list_events("all");
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
