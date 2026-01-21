# Build Script:

Ref: - https://doc.rust-lang.org/cargo/reference/build-scripts.html

Some packages need to compile 3rd-party non-Rust code, ex: C libraries. Other packages need to link to C
libraries which can either be located on the system or possibly need to be build from source. Others still
need facilities for functionality such as code generation before building.

Cargo does not replace these well-optimized tools for that tasks but it does not integrate with them with
custom build scripts. 

Using a `build.rs` in root of a package will cause Cargo to compile the script and execute or just before
the package:

Example:

```rust 
// Example custom build script.
fn main() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=src/hello.c");
    // Use the `cc` crate to build a C file and statically link it.
    cc::Build::new()
        .file("src/hello.c")
        .compile("hello");
}
```


## `build.rs`: build script

In Rust, `build.rs` is a build script. It is a regular Rust file that Cargo compiles and executes before it 
compiles the rest of your package.

It is primarily used when your project needs to perform tasks that the standard Cargo configuration 
(`Cargo.toml`) cannot handle on its own.

---

### Common Use Cases

You will typically see a `build.rs` file used for the following scenarios:

* Compiling C/C++ Dependencies:
  If your Rust project wraps a C library, the build script can use the `cc` crate to compile the C code and 
  link it to your Rust binary.

* Generating Code at Runtime:
  If you use Protocol Buffers (protobuf) or need to generate Rust code from a database schema or a specific 
  specification, the build script generates those `.rs` files before the main compilation starts.

* Linking Native Libraries:
  It tells Cargo where to find system libraries (like OpenSSL or libssl) so the linker can find them.

* Environment Discovery:
  It can detect details about the host operating system or compiler version and pass "features" or 
  configuration flags to your code.


### How it Works

1. Placement: 
   You place `build.rs` in the root directory of your project (next to `Cargo.toml`).

2. Execution: 
   When you run `cargo build`, Cargo checks for this file. If it exists, it compiles it into an executable 
   and runs it.
   
3. Communication: 
   The script communicates with Cargo by printing specially formatted lines to `stdout`.

For example, to tell Cargo to link a library named "z", your `build.rs` would look like this:

```rust
fn main() {
    // Informs Cargo to link the system library 'z'
    println!("cargo:rustc-link-lib=z");
}

```

### When NOT to use it

You should avoid using a build script if your needs can be met by `Cargo.toml`. Build scripts increase 
compile times because they must be compiled before anything else. 
If you are just trying to manage Rust dependencies, stick to the standard manifest.

---

**Would you like me to help you write a specific `build.rs` script for a C library or a code generation task?**
