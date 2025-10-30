use minigrep::{search, search_insensitive};
use std::env;
use std::error::Error;
use std::fs;
use std::process;

// command line args struct
pub struct Config {
    pub query: String,     // argument that holds the search string
    pub file_path: String, // file to search
    pub ignore_case: bool,
}

#[allow(dead_code)]
impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!(" !QQ! Not enough Arguments!");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Config {
            query,
            file_path,
            ignore_case,
        }
    }

    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err(" !QQ! Not enough Arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();
        let ignore_case = env::var("IGNORE_CASE").is_ok();
        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}
//--------------------------------
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    //println!("With text:\n{}", contents);

    let results = if config.ignore_case {
        search_insensitive(&config.query, &contents)
    } else {
        search(&config.query, &contents)
    };
    for line in results {
        println!("{}", line);
    }
    Ok(())
}
//--------------------------------
fn main() {
    // read the command arguments into a vector
    let args: Vec<String> = env::args().collect();

    // Separate and validate command line arguments
    let config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments : {err}");
        process::exit(1);
    });

    // file open, and read file and search the pattern ( search implemented in lib.rs)
    if let Err(e) = run(config) {
        eprintln!("Application Error!!! : {e}");
        process::exit(1);
    }
}
