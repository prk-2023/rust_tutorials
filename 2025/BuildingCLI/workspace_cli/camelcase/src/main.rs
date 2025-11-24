use clap::Parser;

/// A simple tool that converts a string to CamelCase
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The input string to convert
    input: String,
}

fn to_camel_case(input: &str) -> String {
    input
        .split_whitespace()
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<String>()
}

fn main() {
    let cli = Cli::parse();

    let camel = to_camel_case(&cli.input);
    println!("{}", camel);
}
