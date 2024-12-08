# Cargo:

- Cargo is rust build system, package manager and more, it handles many tasks such as building code,
compiling, downloading the libs our code depends on and building libraries which are called depedencies.

- Cargo comes installed with rust and the version can be checked using --version cmd arg.

    $ cargo --version
    cargo 1.78.0 (54d8815d0 2024-03-26)

- Creating project: cargo is also used to create a project. 

```
    $ cargo new HelloWorld
    Creating binary (application) `HelloWorld` package
    warning: the name `HelloWorld` is not snake_case or kebab-case which is recommended for package names,
    consider `helloworld` note: see more `Cargo.toml` keys and their definitions at
    https://doc.rust-lang.org/cargo/reference/manifest.html

    $ cd HelloWorld; tree
    .
    ├── Cargo.toml
    └── src
        └── main.rs

    2 directories, 2 files
```
- Build/run project:
```
    $ cargo run 
    $ cargo build
```
- cargo List all commands:
```
    cargo --list
    Installed Commands:
        add                  Add dependencies to a Cargo.toml manifest file
        b                    alias: build
        bench                Execute all benchmarks of a local package
        build                Compile a local package and all of its dependencies
        c                    alias: check
        check                Check a local package and all of its dependencies for errors
        clean                Remove artifacts that cargo has generated in the past
        clippy               Checks a package to catch common mistakes and improve your Rust code.
        config               Inspect configuration values
        d                    alias: doc
        doc                  Build a package's documentation
        fetch                Fetch dependencies of a package from the network
        fix                  Automatically fix lint warnings reported by rustc
        fmt                  Formats all bin and lib files of the current crate using rustfmt.
        generate-lockfile    Generate the lockfile for a package
        git-checkout         This command has been removed
        help                 Displays help for a cargo subcommand
        init                 Create a new cargo package in an existing directory
        install              Install a Rust binary
        libbpf
        locate-project       Print a JSON representation of a Cargo.toml file's location
        login                Log in to a registry.
        logout               Remove an API token from the registry locally
        metadata             Output the resolved dependencies of a package, the concrete used versions
                             including overrides, in machine-readable format
        miri
        new                  Create a new cargo package at <path>
        owner                Manage the owners of a crate on the registry
        package              Assemble the local package into a distributable tarball
        pkgid                Print a fully qualified package specification
        publish              Upload a package to the registry
        r                    alias: run
        read-manifest        Print a JSON representation of a Cargo.toml manifest.
        remove               Remove dependencies from a Cargo.toml manifest file
        report               Generate and display various kinds of reports
        rm                   alias: remove
        run                  Run a binary or example of the local package
        rustc                Compile a package, and pass extra options to the compiler
        rustdoc              Build a package's documentation, using specified custom flags.
        search               Search packages in the registry. Default registry is crates.io
        t                    alias: test
        test                 Execute all unit and integration tests and build examples of a local package
        tree                 Display a tree visualization of a dependency graph
        uninstall            Remove a Rust binary
        update               Update dependencies as recorded in the local lock file
        vendor               Vendor all dependencies for a project locally
        verify-project       Check correctness of crate manifest
        version              Show version information
        yank                 Remove a pushed crate from the index

```

---

## rustc compiler takes only a single file argument for compilation:

There is a design philosophy behind Rust's decision to only take one input file for compilation. 
This philosophy is rooted in the way Rust approaches compilation and the goals of the language.

In C and C++, the compiler can take multiple input files because the compilation process is designed to be 
incremental. 
Each file is compiled separately, and the resulting object files are linked together to create the final 
executable. This approach allows for faster compilation times, as only the files that have changed need to 
be recompiled.

However, this approach also has some drawbacks. 
For example, it can lead to issues with header files, where changes to a header file can cause multiple 
source files to be recompiled unnecessarily. 
Additionally, the incremental compilation approach can make it more difficult to optimize the code, as the 
compiler only has visibility into a single file at a time.

Rust, on the other hand, takes a more holistic approach to compilation. When you compile a Rust program, the
compiler takes the entire program as input and compiles it as a single unit. 
This approach allows the compiler to perform more aggressive optimizations, as it has visibility into the 
entire program.

There are several reasons why Rust's designers chose to take this approach:

1. **Whole-program optimization**: 
    By compiling the entire program at once, Rust's compiler can perform more aggressive optimizations, such
    as dead code elimination, inlining, and constant folding.
2. **Better error reporting**: 
    When the compiler has visibility into the entire program, it can provide more accurate and helpful error
    messages. For example, if a function is used incorrectly, the compiler can point to the exact location 
    where the function is defined.
3. **Simplified dependency management**: By compiling the entire program at once, Rust's compiler can 
    automatically manage dependencies between files. This eliminates the need for manual dependency 
    management, which can be error-prone and time-consuming.
4. **Improved code analysis**: 
    By analyzing the entire program at once, Rust's compiler can perform more advanced code analysis, such 
    as detecting unused code, unreachable code, and other issues.

To achieve this, Rust uses a concept called a "crate" (short for "container"), which is a single unit of 
compilation that can contain multiple files. When you compile a Rust program, you specify the crate root 
(usually `lib.rs` or `main.rs`), and the compiler compiles the entire crate as a single unit.

While this approach may seem restrictive at first, it has several benefits that make it well-suited for 
Rust's goals of safety, performance, and productivity.
