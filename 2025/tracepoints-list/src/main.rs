use clap::Parser;
use colored::*;
use std::{fs, io};
// use std::{
//     fs::{self, File, OpenOptions},
//     io::{self, BufRead, BufReader, Write},
//     path::PathBuf,
//     sync::{
//         atomic::{AtomicBool, Ordering},
//         Arc,
//     },
// };
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]

struct Args {
    // // Name of the person to greet
    // #[arg(short, long)]
    // name: String,
    //
    // //Number of time to greet
    // #[arg(short, long, default_value_t = 1)]
    // count: u8,
    /// List available events in a subsystem and exit
    #[arg(long)]
    list: Option<String>,
}

//fn main() -> io::Result<()> {
fn main() {
    let args = Args::parse();

    // --list mode
    if let Some(subsystem) = args.list.as_deref() {
        //return list_events(subsystem);
        let _x = list_events(subsystem);
    } else {
        // If no subsystem argument is passed  list all subsystem
        let _x = list_events("");
    }
}

const TRACEFS_BASE: &str = "/sys/kernel/debug/tracing";

#[allow(unused_assignments)]
// fn list_events(subsystem: &str) -> io::Result<()> {
//     let path = if subsystem.is_empty() {
//         format!("{}/events", TRACEFS_BASE);
//     } else {
//         format!("{}/events/{}", TRACEFS_BASE, subsystem);
//     };
//     //let entries = fs::read_dir(&path)?;
//     println!(
//         "Available events in {}:",
//         if subsystem.is_empty() {
//             "all subsystems"
//         } else {
//             subsystem
//         }
//     );
//     let entries = fs::read_dir(&path)?;
//     for entry in entries {
//         let entry = entry?;
//         if entry.file_type()?.is_dir() {
//             if let Some(name) = entry.file_name().to_str() {
//                 println!("  {}", name.red());
//             }
//         }
//     }
//     Ok(())
// }
fn list_events(subsystem: &str) -> io::Result<()> {
    // The `if/else` expression returns a String value
    let path = if subsystem.is_empty() {
        format!("{}/events", TRACEFS_BASE)
    } else if subsystem == "subsystems" {
        format!("{}/events/{}", TRACEFS_BASE, subsystem)
    } else {
        format!("{}/events/{}", TRACEFS_BASE, subsystem)
    };

    // `path` is guaranteed to be a String, which implements AsRef<Path>
    println!(
        "Available events in {}:",
        if subsystem.is_empty() {
            "all subsystems"
        } else {
            subsystem
        }
    );
    let entries = fs::read_dir(&path)?;
    for entry in entries {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            if let Some(name) = entry.file_name().to_str() {
                println!("  {}", name.red());
            }
        }
    }
    Ok(())
}
