# Rust Clap Tutorial: Building Command-Line Interfaces

## Table of Contents

1. [Introduction to Clap](#introduction-to-clap)
2. [Installation and Setup](#installation-and-setup)
3. [Method 1: Derive Macro](#method-1-derive-macro)
4. [Method 2: Builder Pattern](#method-2-builder-pattern)
5. [Method 3: YAML Configuration](#method-3-yaml-configuration)
6. [Common Argument Types](#common-argument-types)
7. [Subcommands](#subcommands)
8. [Practical Example: File Manager](#practical-example-file-manager)
9. [Best Practices](#best-practices)
10. [Exercises](#exercises)

---

## Introduction to Clap

Clap (Command Line Argument Parser) is Rust's most popular crate for building command-line interfaces. 

It provides three main approaches:

- **Derive Macro** (Recommended): Uses Rust attributes for declarative CLI definition
- **Builder Pattern**: Programmatic API for maximum flexibility
- **YAML Configuration**: External YAML files for complex CLIs

### Key Features
- Automatic help generation
- Validation and parsing
- Subcommand support
- Shell completion generation
- Colorized output

---

## Installation and Setup

Add to your `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4.0", features = ["derive"] }
```

For the builder pattern only, you can use:
```toml
[dependencies]
clap = "4.0"
```
You can also run from command line:

`cargo add clap --features derive`

---

## Method 1: Derive Macro

The derive macro is the most modern and recommended approach. 
It's type-safe, readable, and easy to maintain.

### Basic Structure

```rust
use clap::Parser;

/// Brief description of your application
#[derive(Parser)]
#[command(name = "appname")]
#[command(about = "Detailed description", version = "1.0")]
struct Cli {
    // Your arguments here
}
```

NOTE:
-------------------------------------------------------------------------------------------------
In the above example we have *use clap::Parser;*
In rust **use abc::xyz;** 
Here **abc** is a crate
and **xyz** can be enum, struct, trait, function,trait ...

In Clap crate , *Parser*, *Subcommand* are traits defined in clap.

Clarification on use clap::{Parser, Subcommand};
In this specific case: use clap::{Parser, Subcommand};
1. clap is the Crate (Library)
clap is the external dependency (the library) you added to your project's Cargo.toml.

2. Parser and Subcommand are Traits (Not just "Features")

Trait: A trait defines a set of methods that a type must implement. 
It's similar to an "interface" in other languages.

In code we imported the Parser trait: use clap::Parser;

We then used the #[derive(Parser)] attribute on the Args struct.

When you use #[derive(Parser)], you are asking the Rust compiler (via the clap macro) to automatically 
implement the logic of the Parser trait for your Args struct.

The most important method provided by the Parser trait is the static method parse(), which is why you can 
call:

  `let args = Args::parse(); // The `parse` method comes from the `Parser` trait`

==> The **Parser and Subcommand**  traits are implemented by the declarative attribute (#[derive(Parser)])
to automatically generate the trait implementation. 
The entire purpose of this system is to let you write the declarative definition of your data structure 
(what the commands and arguments are), while the macro automatically generates the complex, repetitive 
implementation (how to actually parse and process them).

-------------------------------------------------------------------------------------------------

### Complete Example

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

**Usage:**
```bash
cargo run -- input.txt
cargo run -- input.txt -o output.txt -t 4 -v
cargo run -- --help
cargo run -- --version
```

### Advanced Derive Features

```rust
use clap::{Parser, ValueEnum};

#[derive(Debug, ValueEnum, Clone)]
enum LogLevel {
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

/// Advanced configuration example
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Files to process
    #[arg(required = true, num_args = 1..)]
    files: Vec<String>,
    
    /// Log level
    #[arg(short, long, value_enum, default_value_t = LogLevel::Info)]
    log_level: LogLevel,
    
    /// Configuration file path
    #[arg(short, long, default_value = "config.toml")]
    config: String,
    
    /// Dry run - don't make changes
    #[arg(long)]
    dry_run: bool,
}

fn main() {
    let args = Cli::parse();
    
    println!("Files to process: {:?}", args.files);
    println!("Log level: {:?}", args.log_level);
    println!("Config file: {}", args.config);
    println!("Dry run: {}", args.dry_run);
}
```

---

## Method 2: Builder Pattern

The builder pattern offers maximum flexibility and is useful for dynamic CLI generation.

### Basic Builder Example

```rust
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("myapp")
        .version("1.0")
        .author("Your Name <email@example.com>")
        .about("Does awesome things")
        .arg(
            Arg::new("input")
                .help("The input file to use")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("output")
                .help("The output file to use")
                .short('o')
                .long("output")
                .required(false),
        )
        .arg(
            Arg::new("verbose")
                .help("Enable verbose mode")
                .short('v')
                .long("verbose")
                .action(clap::ArgAction::SetTrue),
        )
        .get_matches();

    // Extract values
    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output");
    let verbose = matches.get_flag("verbose");

    println!("Input: {}", input);
    if let Some(output) = output {
        println!("Output: {}", output);
    }
    println!("Verbose: {}", verbose);
}
```

### Advanced Builder with Validation

```rust
use clap::{Arg, Command};

fn main() {
    let matches = Command::new("advanced")
        .about("Advanced builder example")
        .version("1.0")
        .arg(
            Arg::new("port")
                .short('p')
                .long("port")
                .value_parser(clap::value_parser!(u16).range(1..65535))
                .default_value("8080")
                .help("Port number"),
        )
        .arg(
            Arg::new("count")
                .short('c')
                .long("count")
                .value_parser(clap::value_parser!(usize).range(1..100))
                .default_value("1")
                .help("Number of iterations"),
        )
        .arg(
            Arg::new("name")
                .short('n')
                .long("name")
                .required(true)
                .help("Your name"),
        )
        .get_matches();

    let port = matches.get_one::<u16>("port").unwrap();
    let count = matches.get_one::<usize>("count").unwrap();
    let name = matches.get_one::<String>("name").unwrap();

    println!("Hello {}, running on port {} for {} iterations", name, port, count);
}
```

---

## Method 3: YAML Configuration

For very complex CLIs or when you want to separate configuration from code.

### YAML File (`cli.yaml`)

```yaml
name: myapp
version: "1.0"
about: Does awesome things
author: Your Name <email@example.com>
args:
  - input:
      help: Input file to process
      required: true
      index: 1
  - output:
      help: Output file
      short: o
      long: output
  - verbose:
      help: Enable verbose mode
      short: v
      long: verbose
      action: SetTrue
  - port:
      help: Port number
      short: p
      long: port
      value_parser: 
        range: 
          start: 1
          end: 65535
      default_value: "8080"
```

### Rust Code

```rust
use clap::load_yaml;

fn main() {
    let yaml = load_yaml!("cli.yaml");
    let matches = clap::Command::from(yaml).get_matches();
    
    let input = matches.get_one::<String>("input").unwrap();
    let output = matches.get_one::<String>("output");
    let verbose = matches.get_flag("verbose");
    let port = matches.get_one::<String>("port").unwrap();
    
    println!("Input: {}", input);
    if let Some(output) = output {
        println!("Output: {}", output);
    }
    println!("Verbose: {}", verbose);
    println!("Port: {}", port);
}
```

**Note:** Place the YAML file in your project root and ensure it's included in the build by adding this to your `Cargo.toml`:

```toml
[package]
# ... other package config
build = "build.rs"

[[bin]]
name = "your_binary_name"
path = "src/main.rs"
```

Create `build.rs`:
```rust
fn main() {
    println!("cargo:rerun-if-changed=cli.yaml");
}
```

---

## Common Argument Types

### 1. Flags (Boolean Arguments)

```rust
#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    verbose: bool,
    
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    force: bool,
    
    #[arg(short, long, action = clap::ArgAction::SetFalse)]
    no_cache: bool,
}
```

### 2. Options (Optional Values)

```rust
#[derive(Parser)]
struct Cli {
    #[arg(short, long)]
    output: Option<String>,
    
    #[arg(short, long, default_value = "default.txt")]
    config: String,
    
    #[arg(short, long, default_value_t = 42)]
    answer: u32,
}
```

### 3. Positional Arguments

```rust
#[derive(Parser)]
struct Cli {
    // Single positional argument
    input: String,
    
    // Multiple positional arguments
    #[arg(required = true, num_args = 1..)]
    files: Vec<String>,
    
    // Optional positional with default
    #[arg(default_value = ".")]
    directory: String,
}
```

### 4. Enumerated Values

```rust
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum Format {
    Json,
    Yaml,
    Toml,
    Xml,
}

#[derive(ValueEnum, Clone, Debug)]
enum Color {
    #[value(name = "red", alias = "r")]
    Red,
    #[value(name = "green", alias = "g")]
    Green,
    #[value(name = "blue", alias = "b")]
    Blue,
}

#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_enum)]
    format: Format,
    
    #[arg(short, long, value_enum)]
    color: Option<Color>,
}
```

### 5. Validation and Parsing

```rust
#[derive(Parser)]
struct Cli {
    #[arg(short, long, value_parser = clap::value_parser!(u16).range(1..65535))]
    port: u16,
    
    #[arg(short, long, value_parser = clap::value_parser!(f64).range(0.0..=1.0))]
    probability: f64,
    
    #[arg(short, long, value_parser = parse_custom_type)]
    custom: CustomType,
}

fn parse_custom_type(s: &str) -> Result<CustomType, String> {
    // Custom parsing logic
    Ok(CustomType::from(s))
}
```

---

## Subcommands

Subcommands are essential for Git-style interfaces where you have multiple related commands.

### Basic Subcommands

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new item
    Add {
        /// Item name
        name: String,
        
        /// Force addition
        #[arg(short, long)]
        force: bool,
    },
    
    /// Remove an item
    Remove {
        /// Item name
        name: String,
        
        /// Recursive removal
        #[arg(short, long)]
        recursive: bool,
    },
    
    /// List items
    List {
        /// Show all items
        #[arg(short, long)]
        all: bool,
    },
}

fn main() {
    let args = Cli::parse();
    
    match args.command {
        Commands::Add { name, force } => {
            println!("Adding item: {} (force: {})", name, force);
        },
        Commands::Remove { name, recursive } => {
            println!("Removing item: {} (recursive: {})", name, recursive);
        },
        Commands::List { all } => {
            println!("Listing items (all: {})", all);
        },
    }
}
```

### Nested Subcommands

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: MainCommands,
}

#[derive(Subcommand)]
enum MainCommands {
    /// Database operations
    Db {
        #[command(subcommand)]
        operation: DbCommands,
    },
    
    /// File operations
    File {
        #[command(subcommand)]
        operation: FileCommands,
    },
}

#[derive(Subcommand)]
enum DbCommands {
    /// Initialize database
    Init {
        #[arg(short, long)]
        reset: bool,
    },
    
    /// Backup database
    Backup {
        /// Backup file path
        path: String,
    },
}

#[derive(Subcommand)]
enum FileCommands {
    /// Upload file
    Upload {
        /// File path
        path: String,
    },
    
    /// Download file
    Download {
        /// File identifier
        id: String,
    },
}

fn main() {
    let args = Cli::parse();
    
    match args.command {
        MainCommands::Db { operation } => match operation {
            DbCommands::Init { reset } => {
                println!("Initializing database (reset: {})", reset);
            },
            DbCommands::Backup { path } => {
                println!("Backing up database to: {}", path);
            },
        },
        MainCommands::File { operation } => match operation {
            FileCommands::Upload { path } => {
                println!("Uploading file: {}", path);
            },
            FileCommands::Download { id } => {
                println!("Downloading file with ID: {}", id);
            },
        },
    }
}
```

---

## Practical Example: File Manager

Let's build a complete file manager application using everything we've learned.

```rust
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Debug, ValueEnum, Clone)]
enum OutputFormat {
    Text,
    Json,
    Csv,
    Yaml,
}

#[derive(Debug, ValueEnum, Clone)]
enum SortBy {
    Name,
    Size,
    Modified,
    Created,
}

/// A powerful file management tool
#[derive(Parser)]
#[command(name = "fileman")]
#[command(about = "Manage files efficiently", version = "1.0", author)]
struct Cli {
    /// Global verbose flag
    #[arg(short, long, global = true)]
    verbose: bool,
    
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List directory contents
    List {
        /// Directory to list
        path: Option<PathBuf>,
        
        /// Show hidden files
        #[arg(short = 'a', long)]
        all: bool,
        
        /// Long format with details
        #[arg(short, long)]
        long: bool,
        
        /// Sort by field
        #[arg(short, long, value_enum, default_value_t = SortBy::Name)]
        sort: SortBy,
        
        /// Output format
        #[arg(short, long, value_enum, default_value_t = OutputFormat::Text)]
        format: OutputFormat,
        
        /// Reverse sort order
        #[arg(short, long)]
        reverse: bool,
    },
    
    /// Copy files or directories
    Copy {
        /// Source path
        source: PathBuf,
        
        /// Destination path
        destination: PathBuf,
        
        /// Force overwrite
        #[arg(short, long)]
        force: bool,
        
        /// Recursive copy
        #[arg(short, long)]
        recursive: bool,
        
        /// Preserve file attributes
        #[arg(short, long)]
        preserve: bool,
    },
    
    /// Search for files
    Search {
        /// Search pattern (supports wildcards)
        pattern: String,
        
        /// Search directory
        #[arg(short, long, default_value = ".")]
        directory: PathBuf,
        
        /// Case insensitive search
        #[arg(short = 'i', long)]
        insensitive: bool,
        
        /// Search recursively
        #[arg(short, long)]
        recursive: bool,
        
        /// Maximum depth for recursive search
        #[arg(long)]
        max_depth: Option<usize>,
    },
    
    /// File information
    Info {
        /// File or directory path
        path: PathBuf,
        
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },
}

fn main() {
    let args = Cli::parse();
    
    if args.verbose {
        println!("Verbose mode enabled");
    }
    
    match args.command {
        Commands::List { path, all, long, sort, format, reverse } => {
            let dir = path.unwrap_or_else(|| PathBuf::from("."));
            println!("Listing directory: {:?}", dir);
            println!("All files: {}, Long format: {}, Sort by: {:?}", all, long, sort);
            println!("Output format: {:?}, Reverse: {}", format, reverse);
            
            // Implementation would go here
            list_directory(&dir, all, long, sort, format, reverse);
        },
        Commands::Copy { source, destination, force, recursive, preserve } => {
            println!("Copying {:?} to {:?}", source, destination);
            println!("Force: {}, Recursive: {}, Preserve: {}", force, recursive, preserve);
            
            // Implementation would go here
            copy_file_or_dir(&source, &destination, force, recursive, preserve);
        },
        Commands::Search { pattern, directory, insensitive, recursive, max_depth } => {
            println!("Searching for '{}' in {:?}", pattern, directory);
            println!("Case insensitive: {}, Recursive: {}", insensitive, recursive);
            if let Some(depth) = max_depth {
                println!("Max depth: {}", depth);
            }
            
            // Implementation would go here
            search_files(&pattern, &directory, insensitive, recursive, max_depth);
        },
        Commands::Info { path, detailed } => {
            println!("Getting info for: {:?}", path);
            println!("Detailed: {}", detailed);
            
            // Implementation would go here
            show_file_info(&path, detailed);
        },
    }
}

// Stub implementations - these would contain the actual logic
fn list_directory(
    dir: &PathBuf, 
    all: bool, 
    long: bool, 
    sort: SortBy, 
    format: OutputFormat, 
    reverse: bool
) {
    println!("Implement directory listing here");
}

fn copy_file_or_dir(
    source: &PathBuf, 
    destination: &PathBuf, 
    force: bool, 
    recursive: bool, 
    preserve: bool
) {
    println!("Implement copy operation here");
}

fn search_files(
    pattern: &str, 
    directory: &PathBuf, 
    insensitive: bool, 
    recursive: bool, 
    max_depth: Option<usize>
) {
    println!("Implement file search here");
}

fn show_file_info(path: &PathBuf, detailed: bool) {
    println!("Implement file info here");
}
```

**Usage Examples:**
```bash
# List commands
cargo run -- list
cargo run -- list /home/user -a -l --sort size --reverse
cargo run -- list --format json

# Copy commands
cargo run -- copy source.txt dest.txt
cargo run -- copy dir1 dir2 --recursive --force

# Search commands
cargo run -- search "*.rs" --recursive --max-depth 3
cargo run -- search "TODO" --insensitive

# Info commands
cargo run -- info important_file.txt --detailed

# Global verbose flag
cargo run -- --verbose list -a -l
```

---

## Best Practices

### 1. Use Derive Macro for New Projects
```rust
// ✅ Recommended
#[derive(Parser)]
struct Cli { /* ... */ }

// ❌ Avoid unless you need dynamic features
let cmd = Command::new("app").arg(Arg::new("input"));
```

### 2. Provide Good Documentation
```rust
#[derive(Parser)]
#[command(about, long_about = "A detailed description of what this tool does")]
struct Cli {
    /// The input file to process. Supports various formats.
    #[arg(short, long)]
    input: String,
    
    /// Number of worker threads. More threads may improve performance
    /// but will use more memory.
    #[arg(short, long, default_value_t = 4)]
    threads: usize,
}
```

### 3. Use Appropriate Types
```rust
#[derive(Parser)]
struct Cli {
    // ✅ Use proper types with validation
    #[arg(value_parser = clap::value_parser!(u16).range(1..=65535))]
    port: u16,
    
    // ❌ Avoid string types when numbers are expected
    // #[arg(short, long)]
    // port: String,
}
```

### 4. Group Related Arguments
```rust
#[derive(Parser)]
struct Cli {
    /// Database configuration
    #[command(flatten)]
    db: DatabaseArgs,
    
    /// Logging configuration  
    #[command(flatten)]
    logging: LoggingArgs,
}

#[derive(clap::Args)]
struct DatabaseArgs {
    #[arg(long, default_value = "localhost")]
    db_host: String,
    
    #[arg(long, default_value = "5432")]
    db_port: u16,
}

#[derive(clap::Args)]
struct LoggingArgs {
    #[arg(long, default_value = "info")]
    log_level: String,
    
    #[arg(long)]
    log_file: Option<String>,
}
```

### 5. Handle Errors Gracefully
```rust
use clap::Parser;
use std::process;

#[derive(Parser)]
struct Cli {
    input: String,
}

fn main() {
    let args = match Cli::try_parse() {
        Ok(args) => args,
        Err(e) => {
            eprintln!("Error: {}", e);
            process::exit(1);
        }
    };
    
    // Rest of your application
}
```

---

## Exercises

### Exercise 1: Basic Calculator
Create a CLI calculator that supports:
- Four operations: add, subtract, multiply, divide
- Two numbers as positional arguments
- Operation as a required flag (`--add`, `--subtract`, etc.)
- Optional `--verbose` flag

### Exercise 2: Contact Manager
Build a contact manager with subcommands:
- `add`: Add a new contact (name, email, phone)
- `list`: List all contacts (with optional filtering)
- `remove`: Remove a contact by email
- `search`: Search contacts by name or email

### Exercise 3: Configuration Validator
Create a tool that:
- Takes a config file path (positional)
- Has `--format` option for output (json, yaml, toml)
- Has `--strict` flag for strict validation
- Has `--schema` option for custom schema file

### Exercise 4: Advanced File Organizer
Build a file organizer with:
- Subcommands for different organization strategies (by date, by type, by size)
- Global options for source and destination directories
- Dry-run mode to preview changes
- Configuration file support

---

## Additional Resources

- [Clap Documentation](https://docs.rs/clap/latest/clap/)
- [Clap GitHub Repository](https://github.com/clap-rs/clap)
- [Rust CLI Working Group](https://rust-cli.github.io/)

This tutorial covers the essential aspects of using Clap for building command-line interfaces in Rust. 
Start with the derive macro approach for most projects, and use the builder pattern when you need dynamic
CLI generation or more complex validation logic.
