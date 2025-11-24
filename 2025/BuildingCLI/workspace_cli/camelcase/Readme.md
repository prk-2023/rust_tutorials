# clap 

## Program:
Below program converts a input string into CamelCase string:

```rust 
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
```

## Clap Breakdown:

- Adding clap to the project:

```bash
$ cargo add clap --features=derive
```
- Import the derive macro into the program:

```rust 
use clap::Parser
```
This bring the clap's procedural macro , which will allow you to **turn a structure into a CLI parser**

- Define structure that represents the command line arguments:

```rust 
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The input string to convert
    input: String,
}
```
The next step lies in mapping the command arguments that you want for the program to be mapped to the
Structure.
    1. `#[derive(Parser, Debug)]` tells clap to generate argument-parsing code for this struct

    2. The `#command(author, version, about, long_about = None)` attribute fills the meta-data:
        - author: uses Cargo package author 
        - version: uses cargo package version 
        - about: a short description 

       Each field in this struct becomes a command line argument.

       How these fields works:
       - `input: String` 
        * This is a **positional argument** (no flags are needed)
        * what ever the user types after the program becomes `input`
        * the comment above "/// The input string to convert " is shown with --help 

- Parsing the arguments at run time :

The statement in `main()`  `let cli = Cli::parser`) actual reads the CLI arguments.
It fills the `Cli` struct with the parsed values.
Handles errors, help text, and validation automatically.

## Specifying Different Argument Types with **clap** 

Clap supports a variety of argument types, and you can use annotations to specify the type of each argument.

Overview of how to handle different argument types, including flags, options, and positional arguments, 
using the `Cli` struct.

---

### 1. Specifying Argument Types

The type of each field in your struct determines the expected type of the argument. The most common types are:

* **String**: For strings (single words, sentences, etc.)
* **i32, u32, i64, u64, etc.**: For integers
* **bool**: For flags (true/false values)
* **Vec<T>**: For multiple values (e.g., a list of strings or integers)
* **Option<T>**: For optional values (arguments that are not required)

#### **Example: Argument Types**

Here’s a breakdown of common argument types and how to annotate them:

```rust
use clap::{Parser, ArgAction};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Positional string argument
    input: String,

    /// Integer argument
    #[arg(short, long)]
    number: i32,

    /// Optional flag (bool)
    #[arg(short, long)]
    verbose: bool,

    /// Optional integer argument with a default value
    #[arg(short, long, default_value_t = 10)]
    count: u32,

    /// Multiple values (Vec<T>)
    #[arg(short, long, action = ArgAction::Append)]
    files: Vec<String>,

    /// Optional argument with Option<T>
    #[arg(short, long, default_value_t = None)]
    language: Option<String>,
}
```

#### **Explanation of Each Field**:

1. **Positional Argument (`input: String`)**:

   * This is a regular positional argument. The user provides it directly after the program name.
   * It accepts a single string value.

2. **Integer Argument (`number: i32`)**:

   * The `number` field expects an integer (`i32`).
   * We use `#[arg(short, long)]` to allow the argument to be provided both as a flag (`-n` or `--number`).

3. **Boolean Flag (`verbose: bool`)**:

   * A `bool` is used for flags. If the flag is provided, it’s `true`; otherwise, it’s `false`.
   * Example: `--verbose` or `-v`.

4. **Default Value for Integer Argument (`count: u32`)**:

   * This argument has a default value of `10`. If the user doesn’t provide it, `count` will automatically be set to `10`.

5. **Multiple Values (`files: Vec<String>`)**:

   * This allows the user to provide multiple values (e.g., filenames).
   * `#[arg(action = ArgAction::Append)]` means the argument can be provided multiple times, and Clap will collect them into a `Vec<String>`.

6. **Optional Argument (`language: Option<String>`)**:

   * This is an optional argument. If the user doesn't provide a value, the `language` will be `None`.

---

### **2. More Argument Attributes and Features**

Clap provides several other features that you can use to customize arguments:

#### **Common Attributes**:

* **`#[arg(short = 's', long)]`**: You can specify both short and long flags (e.g., `-s` or `--size`).
* **`#[arg(default_value = "value")]`**: Sets a default value for an argument.
* **`#[arg(required = true)]`**: Makes an argument required (default is optional).
* **`#[arg(value_name = "NAME")]`**: Specifies a custom name for the argument in help text.

#### **Action Types**:

* **`ArgAction::SetTrue` / `SetFalse`**: Use this for boolean flags (default).
* **`ArgAction::Append`**: Allows an argument to be provided multiple times, collecting the values into a vector.
* **`ArgAction::Set`**: Assigns a value to the argument.

#### **Enum and Custom Types**:

You can also specify **enums** or **custom types** for arguments by using the `ValueEnum` trait. Here's an example:

```rust
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
enum Mode {
    Fast,
    Slow,
}

#[derive(Parser)]
struct Cli {
    /// Mode of operation
    #[arg(value_enum)]
    mode: Mode,
}
```

Here, `Mode` is an enum with `Fast` and `Slow` values, and Clap will automatically handle it as an argument.

---

### 3. Comprehensive List of Argument Types and Attributes

* **Positional arguments**: Regular arguments that appear in a specific order (e.g., `input: String`).
* **Flags**: Boolean arguments like `--verbose`.
* **Options**: Optional arguments with a value (e.g., `--count 10`).
* **Multiple Values**: Use `Vec<T>` for collecting multiple instances of an argument (e.g., filenames).
* **Enums**: Use `ValueEnum` to map arguments to a set of predefined values.
* **Default values**: Use `#[arg(default_value = "10")]` to provide default values.
* **Optionals**: Use `Option<T>` to make an argument optional.

### 4. Further Resources

For an exhaustive guide on **Clap** and its features, refer to the official documentation:

* **Clap Documentation**: [Clap Docs](https://docs.rs/clap/latest/clap/)
* **Clap Examples**: [Clap Examples on GitHub](https://github.com/clap-rs/clap/tree/master/examples)

This should give you everything you need to build more complex command-line parsers in Rust with `Clap`.

---


## Example program with string, i32 and bool as input arguments and print them:

```rust 
use clap::Parser;

/// A simple program to take a string, integer, and boolean argument.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// The input string
    #[arg(short = 'i', long, default_value = "Missing String argument")]
    myinput: String,

    /// The number value
    #[arg(short = 'n', long, default_value_t = 0)]
    mynumber: i32,

    /* boolean flags in Clap, you don't need to specify true or false as values. 
     * If the flag is present, the value will be true; if the flag is absent, it will be false.
    */
    /// The verbose flag (boolean)
    #[arg(short, long, default_value_t = false)]
    verbose: bool,
}
fn main() {
    // Parse the command-line arguments
    let cli = Cli::parse();

    // Print the parsed values
    println!("String input: {}", cli.myinput);
    println!("Integer input: {}", cli.mynumber);
    println!("Verbose flag: {}", cli.verbose);
}
```
```bash 
$ ./target/debug/clap_test
String input: Missing String argument
Integer input: 0
Verbose flag: false

$  ./target/debug/clap_test -i hello  -n 100 -v
String input: hello
Integer input: 100
Verbose flag: true 

$ ./target/debug/clap_test -i hello  -n 100
String input: hello
Integer input: 100
Verbose flag: false

...
```


