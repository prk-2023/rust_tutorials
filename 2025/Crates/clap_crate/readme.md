# Clap : Command line Argument parser: 

`clap` is the most popular crate for building command line interfaces.

## Introduction to Clap:

It provides 3 main approaches: (i.e you can use `clap` in 3 main ways)

- Derive Macro( Recommended ): Uses Rust attributes for declarative CLI definition.

  [Derived attributes and code generation => 12-attributes.md](12-attributes.md)

- Builder pattern: Programmatic API for maximum flexibility  ( Builder API )

- YAML Configuration: External YAML files for complex CLIs ( less common )

### Key Features:

- Automatic help generation ( the generated code can be checked using 'cargo expand; )

  [Derived attributes and code generation => 12-attributes.md](12-attributes.md)
    
- Validation and parsing 
- subcommand support
- shell completion generation 
- colorized output 

### Installation and Setup

Add to your Cargo.toml 

[dependencies]
clap = {version = "4.5.48", features = ["derive"]}

#### Method 1: Derive Macro:
This is the most modern and recommended approach, its type-safe readable and easy to maintain.

##### Basic Structure:

```rust 
    use calp::Parser;
    // Brief description of your application 
    #[derive(Parser)]
    #[command(name = "appname")]
    #[command(about = "Detailed description", version = "1.0")]
    struct Cli {
        // The Program arguments go here
    }
```

##### Complex Example:

```rust  
use clap::Parser;

/// A simple file processor

#[derive(Parser)]
#[command(name = "fileproc")]
#[command(about = "Process files with various options", version = "1.0")]
struct Cli {
    /// Input file path
    input: String,
    
    /// Output file path
    #[arg(short, long)]
    output: Option<String>,
    
    /// Number of processing threads
    #[arg(short, long, default_value_t = 1)]
    threads: u8,
    
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();
    
    println!("Processing file: {}", args.input);
    
    if let Some(output) = args.output {
        println!("Output file: {}", output);
    }
    
    println!("Threads: {}", args.threads);
    
    if args.verbose {
        println!("Verbose mode enabled");
        // Add detailed logging here
    }
}
```

----------------------------------------------

Great! The [`clap`](https://crates.io/crates/clap) crate (short for **Command Line Argument Parser**) is the
standard way to build CLI applications in Rust. It's powerful, flexible, and supports:

* Positional and optional arguments
* Flags and switches
* Subcommands
* Argument parsing and validation
* Auto-generated help and version messages

---

## 🧰 How to Use `clap`

You can use `clap` in 3 main ways:

1. **Builder API**
2. **Derive Macros (most popular & idiomatic)**
3. **YAML Configuration (less common)**

We’ll focus on the **Derive Macro API**, which is clean and idiomatic for most Rust projects.

---

## ✅ Add Clap to Your Project

In your `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }  # Adjust version if needed
```

---

## ✅ Full Example: A CLI Tool with `clap`

Let’s build a realistic CLI tool called `fileman` (short for “file manager”) that:

* Has subcommands (`copy`, `move`, `delete`)
* Accepts positional and optional arguments
* Uses flags
* Prints version and help automatically

### 📁 File: `src/main.rs`

```rust
use clap::{Parser, Subcommand, Args};

/// FileMan: A simple CLI file manager
#[derive(Parser)]
#[command(name = "fileman")]
#[command(version = "1.0")]
#[command(author = "Your Name <you@example.com>")]
#[command(about = "A simple CLI tool to manage files", long_about = None)]
struct Cli {
    /// Enable verbose mode (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Subcommands for file operations
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Copy a file from source to destination
    Copy(CopyArgs),

    /// Move a file
    Move(MoveArgs),

    /// Delete a file
    Delete(DeleteArgs),
}

#[derive(Args)]
struct CopyArgs {
    /// Source file path
    #[arg()]
    source: String,

    /// Destination file path
    #[arg()]
    destination: String,

    /// Overwrite if the destination file exists
    #[arg(short, long)]
    force: bool,
}

#[derive(Args)]
struct MoveArgs {
    #[arg()]
    source: String,

    #[arg()]
    destination: String,
}

#[derive(Args)]
struct DeleteArgs {
    #[arg()]
    target: String,

    /// Skip confirmation prompt
    #[arg(short, long)]
    yes: bool,
}

fn main() {
    let cli = Cli::parse();

    // Global verbosity setting
    match cli.verbose {
        0 => println!("Running in normal mode."),
        1 => println!("Running in verbose mode."),
        2 => println!("Running in extra verbose mode."),
        _ => println!("Running in insane verbosity..."),
    }

    // Match subcommands
    match &cli.command {
        Commands::Copy(args) => {
            println!("Copying from {} to {}", args.source, args.destination);
            if args.force {
                println!("Overwriting is enabled.");
            }
        }
        Commands::Move(args) => {
            println!("Moving from {} to {}", args.source, args.destination);
        }
        Commands::Delete(args) => {
            if !args.yes {
                println!("Are you sure you want to delete {}? (confirmation needed)", args.target);
            } else {
                println!("Deleting {} without confirmation.", args.target);
            }
        }
    }
}
```

---

## 🧪 Run and Test

Once compiled:

```bash
# Show help
$ cargo run -- --help

# Copy with verbose and force
$ cargo run -- -vv copy ./a.txt ./b.txt --force

# Move
$ cargo run -- move ./a.txt ./b.txt

# Delete with confirmation
$ cargo run -- delete ./b.txt

# Delete without confirmation
$ cargo run -- delete ./b.txt --yes
```

---

## 💡 Features Covered in This Example

| Feature                                        | Demonstrated |
| ---------------------------------------------- | ------------ |
| `#[derive(Parser)]`                            | ✔️           |
| Subcommands with `#[derive(Subcommand)]`       | ✔️           |
| Flags (`--force`, `--yes`)                     | ✔️           |
| Positional arguments (`source`, `destination`) | ✔️           |
| Verbosity with count (`-v`, `-vv`)             | ✔️           |
| Auto-generated help and version                | ✔️           |
| Grouping args with `#[derive(Args)]`           | ✔️           |

---

## ✅ Tips

* You can use `value_parser` to add validation (e.g. `value_parser = clap::value_parser!(u32)`).
* You can derive `Clone`, `Debug` on argument structs for debugging.
* `ArgAction::Count` gives you nice verbosity flags (`-v`, `-vv`, etc.).

---

Would you like to:

* Add file handling logic to make it functional?
* Use environment variables or config files?
* Use colored output or logging?

Let me know how deep you'd like to go!


----------------------------------------------------------


Great! The [`clap`](https://crates.io/crates/clap) crate (short for **Command Line Argument Parser**) is the standard way to build CLI applications in Rust. It's powerful, flexible, and supports:

* Positional and optional arguments
* Flags and switches
* Subcommands
* Argument parsing and validation
* Auto-generated help and version messages

---

## 🧰 How to Use `clap`

You can use `clap` in 3 main ways:

1. **Builder API**
2. **Derive Macros (most popular & idiomatic)**
3. **YAML Configuration (less common)**

We’ll focus on the **Derive Macro API**, which is clean and idiomatic for most Rust projects.

---

## ✅ Add Clap to Your Project

In your `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4.4", features = ["derive"] }  # Adjust version if needed
```

---

## ✅ Full Example: A CLI Tool with `clap`

Let’s build a realistic CLI tool called `fileman` (short for “file manager”) that:

* Has subcommands (`copy`, `move`, `delete`)
* Accepts positional and optional arguments
* Uses flags
* Prints version and help automatically

### 📁 File: `src/main.rs`

```rust
use clap::{Parser, Subcommand, Args};

/// FileMan: A simple CLI file manager
#[derive(Parser)]
#[command(name = "fileman")]
#[command(version = "1.0")]
#[command(author = "Your Name <you@example.com>")]
#[command(about = "A simple CLI tool to manage files", long_about = None)]
struct Cli {
    /// Enable verbose mode (-v, -vv, -vvv)
    #[arg(short, long, action = clap::ArgAction::Count)]
    verbose: u8,

    /// Subcommands for file operations
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Copy a file from source to destination
    Copy(CopyArgs),

    /// Move a file
    Move(MoveArgs),

    /// Delete a file
    Delete(DeleteArgs),
}

#[derive(Args)]
struct CopyArgs {
    /// Source file path
    #[arg()]
    source: String,

    /// Destination file path
    #[arg()]
    destination: String,

    /// Overwrite if the destination file exists
    #[arg(short, long)]
    force: bool,
}

#[derive(Args)]
struct MoveArgs {
    #[arg()]
    source: String,

    #[arg()]
    destination: String,
}

#[derive(Args)]
struct DeleteArgs {
    #[arg()]
    target: String,

    /// Skip confirmation prompt
    #[arg(short, long)]
    yes: bool,
}

fn main() {
    let cli = Cli::parse();

    // Global verbosity setting
    match cli.verbose {
        0 => println!("Running in normal mode."),
        1 => println!("Running in verbose mode."),
        2 => println!("Running in extra verbose mode."),
        _ => println!("Running in insane verbosity..."),
    }

    // Match subcommands
    match &cli.command {
        Commands::Copy(args) => {
            println!("Copying from {} to {}", args.source, args.destination);
            if args.force {
                println!("Overwriting is enabled.");
            }
        }
        Commands::Move(args) => {
            println!("Moving from {} to {}", args.source, args.destination);
        }
        Commands::Delete(args) => {
            if !args.yes {
                println!("Are you sure you want to delete {}? (confirmation needed)", args.target);
            } else {
                println!("Deleting {} without confirmation.", args.target);
            }
        }
    }
}
```

---

## 🧪 Run and Test

Once compiled:

```bash
# Show help
$ cargo run -- --help

# Copy with verbose and force
$ cargo run -- -vv copy ./a.txt ./b.txt --force

# Move
$ cargo run -- move ./a.txt ./b.txt

# Delete with confirmation
$ cargo run -- delete ./b.txt

# Delete without confirmation
$ cargo run -- delete ./b.txt --yes
```

---

## 💡 Features Covered in This Example

| Feature                                        | Demonstrated |
| ---------------------------------------------- | ------------ |
| `#[derive(Parser)]`                            | ✔️           |
| Subcommands with `#[derive(Subcommand)]`       | ✔️           |
| Flags (`--force`, `--yes`)                     | ✔️           |
| Positional arguments (`source`, `destination`) | ✔️           |
| Verbosity with count (`-v`, `-vv`)             | ✔️           |
| Auto-generated help and version                | ✔️           |
| Grouping args with `#[derive(Args)]`           | ✔️           |

---

## ✅ Tips

* You can use `value_parser` to add validation (e.g. `value_parser = clap::value_parser!(u32)`).
* You can derive `Clone`, `Debug` on argument structs for debugging.
* `ArgAction::Count` gives you nice verbosity flags (`-v`, `-vv`, etc.).

---

Would you like to:

* Add file handling logic to make it functional?
* Use environment variables or config files?
* Use colored output or logging?

Let me know how deep you'd like to go!
