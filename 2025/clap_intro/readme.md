# Crate *clap*

ref: https://www.shuttle.dev/blog/2023/12/08/clap-rust


A *complete beginner-to-advanced tutorial on the `clap` crate* in Rust.

---

## Rust `clap` Crate Tutorial

`clap`: *Command Line Argument Parsing Made Easy*

---

## What is `clap`?

`clap` (Command Line Argument Parser) is the most widely used crate for building 
*user-friendly command-line interfaces (CLI)* in Rust.

It helps you:

* Define arguments and subcommands
* Automatically generate help/usage messages
* Validate types (e.g., `--age <u32>`)
* Create complex CLI tools easily

---

## Installation
Add `clap` to your `Cargo.toml`:

`cargo add clap -F derive `

or  add it manually as below
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

* The `"derive"` feature enables using the `#[derive(Parser)]` macro for easy setup.
[ref:1] More on derive features:
---

## Getting Started (Basic Example)

```rust
use clap::Parser;

/// Simple CLI app
#[derive(Parser, Debug)]
#[command(name = "hello")]
#[command(about = "Greets the user")]
struct Args {
    /// Name of the person
    #[arg(short, long)]
    name: String,

    /// Age (optional)
    #[arg(short, long)]
    age: Option<u32>,
}

fn main() {
    let args = Args::parse();
    println!("Hello, {}!", args.name);
    if let Some(age) = args.age {
        println!("You are {} years old.", age);
    }
}
```

### Example Usage:

```bash
cargo run -- --name Alice --age 30
```

---

## Explanation of Attributes

| Attribute             | Description                           |
| --------------------- | ------------------------------------- |
| `#[derive(Parser)]`   | Enables parsing via struct derivation |
| `#[arg(short, long)]` | Adds `-n` and `--name` flags          |
| `#[command(...)]`     | Metadata for help/version info        |

---

## Adding Subcommands

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "Tool", version = "1.0", about = "CLI with subcommands")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Greet someone
    Greet {
        #[arg(short, long)]
        name: String,
    },

    /// Say goodbye
    Farewell {
        #[arg(short, long)]
        name: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Greet { name } => {
            println!("Hello, {}!", name);
        }
        Commands::Farewell { name } => {
            println!("Goodbye, {}!", name);
        }
    }
}
```

### Example:

```bash
cargo run -- greet --name Bob
cargo run -- farewell --name Alice
```

---

## Built-in Features of `clap`

### 1. Auto Help and Version Flags

```bash
cargo run -- --help
cargo run -- --version
```

### 2. Required vs Optional Arguments

```rust
#[arg(required = true)] // Makes an argument required (default for non-Option types)
```

### 3. Default Values

```rust
#[arg(default_value = "guest")]
name: String,
```

### 4. Value Validation

```rust
#[arg(value_parser = clap::value_parser!(u32).range(18..))]
age: u32, // Only accepts ages 18+
```

---

## Advanced Features

### Environment Variable Support

```rust
#[arg(env = "USERNAME")]
username: String,
```

### Argument Groups (Mutually Exclusive)

```rust
use clap::{ArgGroup, Parser};

#[derive(Parser)]
#[command(group(
    ArgGroup::new("mode")
        .required(true)
        .args(&["verbose", "quiet"])
))]
struct Args {
    #[arg(long)]
    verbose: bool,

    #[arg(long)]
    quiet: bool,
}
```

### Subcommand Nesting

You can nest subcommands (like `git remote add`) using enums recursively.

---

## Testing CLI Tools

Use unit tests with `clap::Command::try_get_matches_from()` to test different CLI inputs.

```rust
#[cfg(test)]
mod tests {
    use clap::Parser;
    use super::Args;

    #[test]
    fn test_name_argument() {
        let args = Args::parse_from(&["app", "--name", "TestUser"]);
        assert_eq!(args.name, "TestUser");
    }
}
```

---

## Real-World Example: `mycli`

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "mycli", version = "1.0", about = "A real CLI example")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a user
    Add {
        #[arg(long)]
        name: String,
        #[arg(long)]
        email: String,
    },
    /// Delete a user
    Delete {
        #[arg(long)]
        id: u32,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Commands::Add { name, email } => {
            println!("Added user: {} with email: {}", name, email);
        }
        Commands::Delete { id } => {
            println!("Deleted user with ID: {}", id);
        }
    }
}
```

---

## Resources

* [crates.io/crates/clap](https://crates.io/crates/clap)
* [clap GitHub](https://github.com/clap-rs/clap)
* [Official clap documentation](https://docs.rs/clap)

---

## Summary

| Feature                 | Supported? |
| ----------------------- | ---------- |
| Positional arguments    | ✅          |
| Named flags/options     | ✅          |
| Subcommands             | ✅          |
| Auto help/version       | ✅          |
| Env var integration     | ✅          |
| Mutually exclusive args | ✅          |

---

Would you like this as a downloadable `.md` file or to scaffold a sample `clap` project with `cargo new` and a filled-out `main.rs`?


-----------------------------------------------------------------
[ref:1]: derive feature:
-----------------------------------------------------------------
In Rust features: are optional components or capabilitied of a crage. 
This is similar to addon that enabled extra features. 

1. 
By default when we add a crate :
ex: `cargo add crate-name` 
By default you will only get its core functionality. 
2. Many crates provide features that you can optionally enable using -F or --features option. 

What is `derived` feature?
ex: 

cargo add clap -F derive 

- we are adding clap crate (command line argument parser) and enabling its `derive` feature.

```rust
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    name: String,
}
```

- `#[derive(Parser)]` this is a macro
- This generates code that handles parsing command-line args.
- this macro only works when the `derive` feature is enabled in the `clap` crate.

Reason for this design:

Crate authors often split features to:
- keep dependencies minimal ( macros may require extra crates like `syc` or `pro-macro2`.. )
- Allow uses to choose only what is required for them, which makes compilation faster and produce smaller
  binary sizes.


[dependencies]
clap = { version = "x.y", features = ["derive"] }

This is some thing similar to C where we add addtional macros as 

```c 
#include "config.h"

#ifdef USE_ARG_MACROS
#include "arg_macros.h"  // Header that defines macro magic for parsing
#endif

int main(int argc, char **argv) {
    #ifdef USE_ARG_MACROS
    PARSE_ARGS(argc, argv);  // fancy macro
    #else
    // manual parsing
    #endif
}
```
- arg_macros.h is like enabling the derive feature in clap.
- Without defining USE_ARG_MACROS, those macros aren’t included at all.

Allowing to control this behaviour at compilation time.
-----------------------------------------------------------------
# Intro to Clap :

Ref : https://dev.to/moseeh_52/getting-started-with-clap-a-beginners-guide-to-rust-cli-apps-1n3f

## Intro

"clap": stands for command line argument parser.
If a popular Rust crate that helps to easily define a parse command line arguments in a safe, efficient and
expressive way.

When we are building command line apps in Rust list ripgrep, cargo, or bat... `clap` is often the way to go
solution for handling inputs like:

```
    myapp --file config.txt --verbose
```
Along with `clap` Rust's ecosystem supports other mature and high-quality crates like:
- clap              : cli parser 
- anyhow/thiserror  : error handling 
- serde             : Data serialization ( JSON, TOML, YAML )
- tokio /async-std  : Async runtime 
- indicatif         : Progress bad and spinners 

## What is `clap`?

Command line Argument Parser its a popular Rust crate for building CLI apps. It carters:
- Parsing arguments ( --file config.toml )
- Validating input 
- Showing help and usage messages 
- Handling subcommands 
- Support environment variables, default values and more...
- It has clean and declarative API 
- rich features like --help --version , subcommands and env support.
- built in validation, error reporting, and enum support. 
- Active and on par with modern ergonomics.

We can define your CLI either by describing a structure ( derive-style )
or by building it manually ( Builder-style ).

Note: Apart from `clap` there are other ways to parse CLI arguments in Rust using
1. std::env::args() : lowlevel access to command-line args.
2. getopts: Old crate inspired by c-ctyle option parsing 
3. structopt: predecessor to clap now merged into clap.

Reload rust topics:

### Traits: The Foundation 

Traits are like interfaces in other languages. It defines behaviour that a type can implement.

Example:
```rust 
traig Speak {
    fn say_hello(&self);
}

struct Person;

impl Speak for Person {
    fn say_hello(&self) {
        println!("hello!");
    }
}
fn main() {
    let person = Person;
    person.say_hello(); // calls the trair method:
}
```

in `clap` traits like  *`Parser`* and *`ValueEnum`* let your struct automatically become a CLI parser or
enum handler. 

#### *`Parser`* Trait: Derive your CLI:

```rust 

use clap::Parser;

#[derive(Parser)]
#[command(name = "myapp")]

struct Args {
    #[arg(short, long)]
    file: String,
}

fn main() {
    let args = Args::parse(); // This comes from parser trait
    println!("File:{}". args.file);
}
```

The `Parser` trait gives:
    - Args::parse() - Parse CLI args from the command line 
    - Args::try_parse() - Parse and catch errors instead of exiting. 
    - Args::parse_from(...) - Parse from a custom source ( like in tests )

#[command(...)]: App Metadata 
This attribute comes from the `Parser` trait and lets us define:
    - name — App name
    - version — App version
    - about — Short description
    - author — Developer name/email
`clap` uses this above info to generate "--help" and "--version"

#[arg(...)]: Fine-tune Your CLI
Each filed in the struct represents a 'flag', 'argument' or 'option'

```rust 
struct Args {
    #[arg(short, long, help = "Path to the config file", default_value = "config.toml")]
    file: String,

    #[arg(long, env = "RUN_MODE")]
    mode: Option<String>,
}
```
- short, long: adds -f and --file  ( -h or --help )
- help: adds help text
- default_value: gives a fallback
- env: reads from an env var if missing

#### ValueEnum: Enums with Argument Values
With `ValueEnum`, you can let the user pick from fixed enum values like --mode fast or --mode slow.
example:
```rust 
    enum Mode {
        Fast,
        Slow.
    }

    #[derive(Parser)]
    struct Args {
        #[arg(long)]
        mode:   
    }

```

==> Clap:
    - Automatically convert strings to enum values 
    - Show allowed values in the --help message 
    - Reject anything with a friendly error.

#### Common Derive Traits You Might Use:

|Trait      |What it enables                        | Comes from |
|-----------|---------------------------------------|------------|
|Parser     |::parse() to turn CLI into a structure | clap crate |
|ValueEnum  |Enum parsing from strings              | clap crate |
|Debug      |println!("{?}",val) for debugging      | std lib    |
|Clone      |.clone() support for struct/enum       | std lib    |

These are added using #[derive(...)] and give your types functionality automatically.

## Builder vs Derive derive-style

Clap supports both:

- Derive style (what we've been using)
```rust 
#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    name: String,
}
```
- Builder style (manual but flexible)
```rust 
use clap::{Command, Arg};

let matches = Command::new("myapp")
    .arg(Arg::new("name").short('n').long("name").required(true))
    .get_matches();
```

Example: file-checker

```rust 
use clap::Parser;
use clap::ValueEnum;

#[derive(Parser)]
#[command(name = "file-check", version = "0.1.0")]

struct Cli {
    #[arg(short, long)]
    file: PathBuf,

    #[arg(long, default_value = "fast")]
    mode: Mode,
}
fn main() {
    let args = Cli::parse();
    println!("Mode: {:?}", args.mode);
    
    if args.file.exists() {
        println!("Found");
    } else {
        println!("Found");

    }
}
```

`cargo run -- --file ./config.toml --mode slow`
