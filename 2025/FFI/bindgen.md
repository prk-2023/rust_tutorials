 # `bindgen` : FFI tool for rust:


`bidgen` is an essential tool for FFI (Foreign Function Interface). It automatically generates Rust FFI
bindings to `C` ( and some `C++`)libraries, This tool offload the tedious and error prone task of
manually translating C headers into Rust structs and function declarations. 

Essentially `bindgen` reads C header files (`.h`) and produce a Rust file ( `.rs`) containing the
equivalent `extern "C"` blocks.

## How `bindgen` works:

`bindgen` uses **libclang** to parse C/C++ headers. Because it uses a real compiler front-end, it
understands complex C constructs, macros, and types perfectly.

**The WorkFlow** 

1. *Input*: You provide a C header file ( ex: `libfoo.h` )
2. *Processing*: `bindgen` parses the header using `libclang`.
3. *Output*: It generates a Rust module containing `repr(C)` structs, enums and `extern "C"` function
   wrappers.


## How to use `bindgen`:

The most common ( and recommended ) way to use `bindgen` is via a build script (`build.rs`). This ensures
that bindings are automatically updated whenever your C headers change. 

### 1. Project setup:

- Add `bindgen` to Cargo.toml under `[build-dependencies]`.

```toml 
[package]
name = "my-library-sys"
version = "0.1.0"
edition = "2021"

[dependencies]
# Your dependencies here

[build-dependencies]
bindgen = "0.70" # Use the latest version
```

### 2. Create a Wrapper Header 
Create a file named wrapper.h in your project root. 
This file should include all the C headers you want to generate bindings for.

```c
// wrapper.h
#include <curl/curl.h>
#include "my_local_lib.h"
```

### 3. Create the Build Script

Create a `build.rs` file in your project root. 
This script tells `Cargo` to run `bindgen` before compiling your Rust code.

```rust 
use std::env;
use std::path::PathBuf;

fn main() {
    // 1. Tell cargo to look for shared libraries in a specific directory
    println!("cargo:rustc-link-search=/path/to/lib");

    // 2. Tell cargo to tell rustc to link the system library
    println!("cargo:rustc-link-lib=foo");

    // 3. Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // 4. Generate the bindings
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        // Only generate bindings for functions starting with "foo_"
        .allowlist_function("foo_.*") 
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    // 5. Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
```

### 4. Include the Bindings in Rust

In your `src/lib.rs` or `src/main.rs`, use the include! macro to pull in the generated code.

```rust 
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Now you can call C functions!
fn main() {
    unsafe {
        let handle = foo_init();
        // ...
    }
}
```

Best Practices & Tips:

*The unsafe Block* : 
    All generated functions from `bindgen` are inherently `unsafe`. It is best practice to wrap these in a "safe" Rust abstraction layer rather than using the raw bindings throughout your app.

*Allowlisting* : 
    By default, `bindgen` will try to generate bindings for everything it finds (including standard system headers). Use `.allowlist_function`, `.allowlist_type`, and `.allowlist_var` to keep your bindings file clean.

*Handling Opaque Types* : 
    If a C struct is private or too complex, you can use `.opaque_type("my_struct")` to treat it as a blob of bytes in Rust, preventing `bindgen` from trying to map its fields.


