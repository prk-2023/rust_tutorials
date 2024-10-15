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
