use read_file::search;
use std::env;
use std::error::Error;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect(); /* collect() method on iterator turn in to relevant type*/

    println!("-> {} <-", &args[0]);
    // let config = Config::new(&args);
    // or
    let config = Config::build(&args).unwrap_or_else(|err| {
        println!("Problem parsing arguments : {err}");
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In file : {}", config.file_path);

    // let file_contents =
    //     fs::read_to_string(config.file_path).expect("should have been able to read the file!");
    // println!("With text:\n{}", file_contents);
    if let Err(e) = run(config) {
        println!("Application Error!!! : {e}");
        process::exit(1);
    }
}
//------------------------------------------
fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    //fs::read_to_string(config.file_path).expect("Should have been able to read the file!");

    println!("With text:\n{}", contents);

    for line in search(&config.query, &contents) {
        println!("{}", line);
    }
    Ok(())
}
//------------------------------------------
struct Config {
    query: String,
    file_path: String,
}
// If we with to use reference then oweners we have to redefine the struct with string slice and
// add lifetimes are reference are involved to let the compiler inform how long the referenced are
// valid.
// struct Config<'a> {
//  //both fields will now hold references with lifetime 'a
//  query: &'a str,
//  file_path: &'a str,
//
// }
#[allow(dead_code)]
impl Config {
    fn new(args: &[String]) -> Config {
        if args.len() < 3 {
            panic!(" !QQ! Not enough Arguments!");
        }
        // if we wish to replace clone(), then we have to use other commented Config struct with
        // lifetimes as reference are involved in which lifetime notation case changes are required
        let query = args[1].clone();
        let file_path = args[2].clone();
        Config { query, file_path }
    }
    //or use the below method if you prefer a Return result
    fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            //panic!(" !QQ! Not enough Arguments!");
            return Err(" !QQ! Not enough Arguments!");
        }
        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}
// fn parse_config(cmd_args: &[String]) -> Config {
//     let query = cmd_args[1].clone();
//     let file_path = cmd_args[2].clone();
//
//     // (query, file_path)
// }

// fn main() {
//     let args: Vec<String> = env::args().collect(); /* collect() method on iterator turn in to relevant type*/
//
//     println!("-> {} <-", &args[0]);
//
//     //println!("-> {} <-", &args[1]);
//     //let tmpargs = args.clone(); // clone args to prevent from moving
//     //dbg!(tmpargs);
//
//     // Save the arguments to variables
//     //let query = &args[1];
//     //let file_path = &args[2];
//     //let (query, file_path) = parse_config(&args);
//     let config = parse_config(&args);
//
//     //println!("Searching for {query}");
//     //println!("In file : {file_path}");
//     println!("Searching for {}", config.query);
//     println!("In file : {}", config.file_path);
//
//     let file_contents =
//         fs::read_to_string(config.file_path).expect("should have been able to read the file!");
//     println!("With text:\n{}", file_contents);
// }
//
// struct Config {
//     query: String,
//     file_path: String,
// }
//
// //fn parse_config(cmd_args: &[String]) -> (&str, &str) {
// fn parse_config(cmd_args: &[String]) -> Config {
//     // let query = &cmd_args[1];
//     // let file_path = &cmd_args[2];
//     // clone can take some additional time, but
//     let query = cmd_args[1].clone();
//     let file_path = cmd_args[2].clone();
//
//     // (query, file_path)
//     Config { query, file_path }
// }
