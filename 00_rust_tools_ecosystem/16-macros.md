# Macros in Rust:
---

### 🔍 **“Macros in Rust are substantially different from macros in C.”**

Rust's macros are **not just simple text substitution** like C macros. In C:

```c
#define SQUARE(x) x * x
```

This is pure **text replacement**, which can lead to issues:

```c
int result = SQUARE(1 + 2); // expands to 1 + 2 * 1 + 2 = 1 + 2 + 2 = 5 (wrong!)
```

---

### 🌳 **“They work on the syntax tree and are hence not prone to substitution issues.”**

Rust macros operate on the **abstract syntax tree (AST)**. This means:

* The Rust compiler understands the **structure** of the code inside a macro.
* There is no raw text substitution—Rust macros generate **valid, structured Rust code**, avoiding 
  problems like unintended precedence or naming conflicts.

So Rust's macro system is **safer and more powerful**.

---

### 🛠️ **“Can be used for static code generation, as they integrate nicely with the strict type system.”**

Because Rust macros understand the code structure and integrate with the type system, you can:

* Generate repetitive or boilerplate code at compile time.
* Define complex logic once and reuse it with different types or parameters.
* Still catch **type errors** at compile time, unlike C where macro expansion might produce invalid code 
  that fails silently or at runtime.

For example:

```rust
macro_rules! create_function {
    ($name:ident) => {
        fn $name() {
            println!("You called {:?}()", stringify!($name));
        }
    };
}

create_function!(foo);
create_function!(bar);

fn main() {
    foo();
    bar();
}
```

This generates two functions at compile time.

---

### ✍️ **“In addition to the built-in macros, the programmer can easily define his/her own macros.”**

Rust has built-in macros like `println!`, `vec!`, `format!`, etc.

But you can also define:

* **Declarative macros** using `macro_rules!` (simpler).
* **Procedural macros** for more advanced use, like deriving traits automatically (`#[derive(Debug)]`), attribute-like macros, or function-like macros.

---

### ✅ Summary

| Feature                     | Rust Macros                               | C Macros                         |
| --------------------------- | ----------------------------------------- | -------------------------------- |
| Based on syntax             | ✅ Yes (AST-based)                         | ❌ No (text substitution)         |
| Type-checked                | ✅ Yes                                     | ❌ No                             |
| Custom macro support        | ✅ Yes (`macro_rules!`, procedural macros) | ✅ Yes (`#define`, but less safe) |
| Compile-time generation     | ✅ Yes                                     | ✅ Yes (but unsafe and limited)   |
| Safe from substitution bugs | ✅ Yes                                     | ❌ No                             |

---

Rust macros are powerful tools for **safe, reusable, and type-aware code generation**, offering a major 
advantage over C-style macros.

# More on macros 

In Rust we generally come across automatic code generation via macros in 3 main stages:
1. Beginers: Usage of built-in macros:

    - println!() : this macros takesin input and generated the necessary rust code to format and print the
      string, which is sometime a regular function can not do it because it needs to accept a variable
      number of arguments and types.
    - vec!() : Generated code to create a `Vec` from a list of automatic code generation. 
    - #[derinve(Debug)]: this is a powerful form of automatic code generation. By adding this attributes to
      a struct or enum, the compiler automatically generated the code to implement `Debug` tait ( allowing
      us to use {:?} formatting for printing. or `Clone` trair.

    These save you from manually writing the boilerplate code for these common functionalities. 

2. Second state: where we see automatic code generation: While using popular crates:
    
    - Serialization/Deserialization (ex: `sarde`): `sarde` it uses 
    #[derive(Serialize, Deserialize)] to automatically generate the code for converting a struct into a
    format like JSON and vice-versa. ( this is time saver )
    - `clap`: Command line parser ( check ../2025/example-cli/ and other clap examples)
    `clap` uses :
        #[derive(Parser)] to generate all the argument parsing, help messages and version from a simple
        structure definition.
    - Asynchronous Programming ( ex: `async-trait`): in async world, macros are ofen used to bridge gaps in
      the language. The `async-trait` macro for instance generates the necessary boilerplate to allow `async`
      methods to be used in traits, which are natively supported in Rust versions.

3. Advance Stage: when writing your own macros: This is for advanced use cases where you need to reduce
   boilerplate in your own project or build a DSL ( Domain specific language )
   For example we might create a macro to:
   - Define a set of similar functions: If you have a dozen functions that all follow a similar pattern, you
     can write a macro to generate all of them from a simple list.

   - Validate data at compile-time: A macro could inspect a struct's fields and generate code to enforce
     certain constraints, providing an error before the program even runs.

   - Integrate with an external system: For instance, a macro could parse a database schema and generate
     Rust structs that correspond to the database tables, saving you from a lot of manual work. 


## How to see the generated code before compilation:

1. The automatic code generation can be viewed using `cargo-expand` tool.

    `cargo install cargo-expand`

   This downloads, compiles, and installs the tool into your $CARGO_HOME/bin directory.

2. run cargo expand inside your project directory

```bash 

$ cargo new hello_world
    Creating binary (application) `hello_world` package
note: see more `Cargo.toml` keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

$ cd hello_world
$ cargo expand
    Checking hello_world v0.1.0 (/tmp/hello_world)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.14s

#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2024::*;
#[macro_use]
extern crate std;
fn main() {
    {
        ::std::io::_print(format_args!("Hello, world!\n"));
    };
}
```

The expand command will:

- Temporarily change your toolchain to the nightly channel, as cargo-expand uses an unstable compiler feature.
- Expand all the macros in your crate.
- Format the resulting code using rustfmt (if you have it installed).
- Print the expanded code to your terminal.

