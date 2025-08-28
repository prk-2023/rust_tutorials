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
