use clap::Parser;
use colored::*;
use std::fs::File;
//use std::io::{self, Read, Seek, SeekFrom, Stdin};
use std::fmt::Write;
use std::io::{self, Read, Seek, SeekFrom};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(help = "File to read. Defaults to STDIN.")]
    file: Option<String>,

    #[arg(short, long, default_value_t = 16, help = "Bytes per line in output")]
    line: usize,

    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "Number of bytes to read (0 for all)"
    )]
    number: usize,

    #[arg(
        short,
        long,
        default_value_t = 0,
        help = "Byte offset to begin reading"
    )]
    offset: i64,
    // #[arg(
    //     short,
    //     long,
    //     default_value_t = true,
    //     help = "enable/disable color output"
    // )]
    // colour: bool,
}

fn main() {
    let args = Args::parse();

    if args.line == 0 {
        eprintln!("{}", "Error: bytes per line must be greater than 0.".red());
        std::process::exit(1);
    }

    // Read from file or stdin
    match args.file {
        Some(path) => {
            let mut file = match File::open(&path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("{} {}", "Error opening file:".red(), e);
                    std::process::exit(1);
                }
            };

            if args.offset > 0 {
                if let Err(e) = file.seek(SeekFrom::Start(args.offset as u64)) {
                    eprintln!("{} {}", "Error seeking in file:".red(), e);
                    std::process::exit(1);
                }
            } else if args.offset < 0 {
                // Seek backwards from end
                let size = match file.seek(SeekFrom::End(0)) {
                    Ok(sz) => sz,
                    Err(e) => {
                        eprintln!("{} {}", "Error seeking in file:".red(), e);
                        std::process::exit(1);
                    }
                };

                let target = if (-args.offset) as u64 > size {
                    0
                } else {
                    size - (-args.offset) as u64
                };

                if let Err(e) = file.seek(SeekFrom::Start(target)) {
                    eprintln!("{} {}", "Error seeking in file:".red(), e);
                    std::process::exit(1);
                }
            }

            dump_file(file, args.number, args.line, args.offset.max(0) as usize);
        }
        None => {
            if args.offset != 0 {
                eprintln!(
                    "{}",
                    "Error: Cannot seek offset when reading from STDIN.".red()
                );
                std::process::exit(1);
            }
            dump_file(io::stdin(), args.number, args.line, 0);
        }
    }
}

fn dump_file<T: Read>(
    mut input: T,
    num_to_read: usize,
    bytes_per_line: usize,
    display_offset: usize,
) {
    let read_all = num_to_read == 0;
    let mut bytes_remaining = if read_all { usize::MAX } else { num_to_read };
    let mut buffer = vec![0u8; bytes_per_line];
    let mut total_read = 0usize;

    println!("{}", top_line(bytes_per_line));

    loop {
        let to_read = bytes_per_line.min(bytes_remaining);
        match input.read(&mut buffer[..to_read]) {
            Ok(0) => break, // EOF
            Ok(n) => {
                println!(
                    "{}",
                    line(&buffer, n, display_offset + total_read, bytes_per_line).bright_yellow()
                );
                total_read += n;
                bytes_remaining = bytes_remaining.saturating_sub(n);
                if bytes_remaining == 0 {
                    break;
                }
            }
            Err(e) => {
                eprintln!("{} {}", "Read error:".red(), e);
                std::process::exit(1);
            }
        }
    }

    if total_read == 0 {
        println!("{}", empty_line(display_offset, bytes_per_line));
    }

    println!("{}", bottom_line(bytes_per_line).green());
}

// (Reuse your existing top_line, bottom_line, line, empty_line, line_number functions here,
// just copy-paste from your original code, with minor fixes if necessary.)

// For brevity, here’s just an example for one of them:

fn top_line(num_per_line: usize) -> String {
    let mut line = String::from("┌──────────┬");

    for i in 0..num_per_line {
        if i > 0 && i % 8 == 0 {
            line.push_str("──");
        }
        line.push_str("───");
    }

    line.push_str("─┬─");

    for i in 0..num_per_line {
        if i > 0 && i % 8 == 0 {
            line.push_str("─");
        }
        line.push_str("─");
    }

    line.push_str("─┐");
    return line.bright_black().to_string();
}

// ... implement bottom_line, empty_line, line_number, line similarly (copy from original)

// I can help you with that if you want!

fn bottom_line(num_per_line: usize) -> String {
    let mut line = String::from("└──────────┴");

    for i in 0..num_per_line {
        if i > 0 && i % 8 == 0 {
            line.push_str("──");
        }
        line.push_str("───");
    }

    line.push_str("─┴─");

    for i in 0..num_per_line {
        if i > 0 && i % 8 == 0 {
            line.push_str("─");
        }
        line.push_str("─");
    }

    line.push_str("─┘");
    return line.bright_black().to_string();
}

fn empty_line(offset: usize, num_per_line: usize) -> String {
    let mut line = format!("│ {:width$X} │", offset, width = 8);

    for i in 0..num_per_line {
        if i > 0 && i % 8 == 0 {
            line.push_str("  ");
        }
        line.push_str("   ");
    }

    line.push_str(" │ ");

    for i in 0..num_per_line {
        if i > 0 && i % 8 == 0 {
            line.push_str(" ");
        }
        line.push_str(" ");
    }

    line.push_str(" │");
    return line.bright_black().to_string();
}

fn line_number(offset: usize) -> String {
    let number = format!("{:X}", offset);
    if number.len() > 8 {
        return number;
    }

    let mut padding = String::from(" ");
    for _ in 0..(8 - number.len()) {
        padding.push_str("·");
    }
    return format!("{}{}", padding.bright_black().to_string(), number);
}

fn line(bytes: &[u8], num_bytes: usize, offset: usize, num_per_line: usize) -> String {
    let mut line = format!("{1}{0} {1}", line_number(offset), "│".bright_black());

    for i in 0..num_per_line {
        if i > 0 && i % 8 == 0 {
            line.push_str(&" ┆".bright_black().to_string());
        }
        if i < num_bytes {
            write!(line, " {:02X}", bytes[i]).unwrap();
        } else {
            line.push_str("   ");
        }
    }

    line.push_str(&" │ ".bright_black().to_string());

    for i in 0..num_per_line {
        if i > 0 && i % 8 == 0 {
            line.push_str(&"┆".bright_black().to_string());
        }
        if i < num_bytes {
            if bytes[i] > 31 && bytes[i] < 127 {
                line.push(bytes[i] as char);
            } else {
                line.push_str(&"·".bright_black().to_string());
            }
        } else {
            line.push_str(" ");
        }
    }

    line.push_str(&" │".bright_black().to_string());
    return line;
}
