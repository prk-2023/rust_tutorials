use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "file-check", version = "1.0")]
struct Cli {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(long, default_value = "fast")]
    mode: Mode,
}

#[derive(Copy, Clone, Debug, ValueEnum)]
enum Mode {
    Fast,
    Slow,
}

fn main() {
    let args = Cli::parse();
    println!("Mode: {:?}", args.mode);

    if args.file.exists() {
        println!("Found");
    } else {
        println!("Not Found");
    }
}
