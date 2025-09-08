use clap::Parser;

#[derive(Parser, Debug)]
#[command(author,version, about, long_about = None)]

struct cli_arguments {
    name: String,
    age: u32,
    count: u8, // Number of time to greet
}
fn main() {
    let args = cli_arguments::parse();
    println!("Hello: {}, {} is your age group", args.name, args.age);
    // or
    for _ in 0..args.count {
        println!("hello {}!", args.name);
        println!("your age group {}!", args.age);
    }
}

/* This example is the `clap` crate to parse command line arguments.
 * The code defines a simple CLI that takes a single argument, name
 * and then print that name.
 *
 * `clap` crate is a out of the box for getting a polished CLI experience
 * which includes common argument behaviour, help generation, suggested fixed from users, colored
 * output shell ***completion***.
 * Reasonable parse performance,
 *
 * // Code analysis:
 * 1.
 *  use clap::Parser;
 *  This imports the necessary traits and macros from the `clap` crate.
 *  The `Parser` trait allows your struct to parse command line arguments and Subcommand ( this
 *  required use clap::Subcommand) Subcommand is not used in the project
 *
 * 2. struct Args {}
 * `struct Args {}` this is the core of the argument parsing, It's standard Rust struct that
 * defines the shape of the data you want to receive from the command line
 *
 * - #[derive(Parser)] : This is the key part for automatic code generation.
 *   `derive` is the Rust macro that lets us automatically implement traits for a struct or enum.
 *   when we add `#[derive(Parser)}` to Args, `clap` crate automatically generates code needed to:
 *   - Parse the command line arguments
 *   - Handle help messages ( -h or --help )
 *   - Handle version information ( -V or --version )
 *   - Validate the arguments that are provided
 *
 * 3.  #[command(author,version, about, long_about = None)]  : this attribute adds meta data to
 *     your CLI. `Clap` macro uses this information to generate a professional looking help message
 *     for the program. ( this automatically includes author, version and brief description (about) )
 *     All this means we do not have to write the code manually.
 *
 * 4. `name.String` This field defines a required command-line argument named `name`.
 *      `Clap` crate automatically generated the code to :
 *      - read the value provided by the user ( ex: cargo -- Mr.X)
 *      - Store the value in the `name` field of the Args struct
 *
 * For more on automatically code generation read the second section of:
 *      * ../../../00_rust_tools_ecosystem/16-macros.md
 *
 */
