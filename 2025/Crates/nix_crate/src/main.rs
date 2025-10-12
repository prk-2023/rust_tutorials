use clap::Parser;
use nix::dir::Dir;
use nix::fcntl::{open, OFlag};
use nix::sys::stat::Mode;
use std::os::unix::io::OwnedFd;
use std::path::PathBuf;

/// List files in a directory using nix
#[derive(Parser, Debug)]
struct Args {
    /// Path to the directory
    #[arg(short, long)]
    path: PathBuf,
}

fn main() {
    let args = Args::parse();

    let dir_path = args.path;

    if !dir_path.is_dir() {
        eprintln!("Error: '{}' is not a directory", dir_path.display());
        std::process::exit(1);
    }

    let fd: OwnedFd = match open(
        &dir_path,
        OFlag::O_DIRECTORY | OFlag::O_RDONLY,
        Mode::empty(),
    ) {
        Ok(fd) => fd,
        Err(e) => {
            eprintln!("Failed to open directory: {}", e);
            std::process::exit(1);
        }
    };
}
