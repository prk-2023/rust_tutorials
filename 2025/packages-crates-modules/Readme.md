# Packages, Crates, Modules:

## Understand the Module system:

Rust's Module system helps you organize code as it grows:

1. Packages: A Cargo feature that lets you build, test and share crates.

2. Crates: A tree of modules that produces a library or executable 

3. Modules and use: let you control the organization, scope and privacy of paths.

4. Paths: A way of naming an item, such as a struct, function, or module.

### Why Modules matter:

- Organize large codebase.

- Control privacy: *public* vs *private*.

- Prevent naming conflict. 

- Make code more maintainable.


### Packages and Crates:

**Key Concepts** :
- Crate: Smallest amount of code that Rust compiler considers at at time.

    - Binary Crate: Executable program 

    - Library Crate: Code intended to be used by other programs

- Package: One or more crates that provide a set of functionality 

    - Contains `Cargo.toml` file describing how to build those crates.


### Creating a package: 

```bash 
// create a package
$ cargo new my-project 
$ cd my-project 
$ tree 
my-project/
├── Cargo.toml
└── src/
    └── main.rs
```

**Cargo.toml**:
```toml
[package]
name = "my-project"
version = "0.1.0"
edition = "2021"

[dependencies]

```

**src/main.rs** 
```rust 
fn main() {
    println!("Hello, world!");
}
```

### Creating a Library crate
