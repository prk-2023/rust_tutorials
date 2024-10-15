# Rustup

- ref: https://rust-lang.github.io/rustup/index.html
- 'rustup': Rust toolchain installer.

- Rustup installs The Rust Programming Language from the official release channels, enabling you to easily
  switch between stable, beta, and nightly compilers and keep them updated. It makes cross-compiling simpler
  with binary builds of the standard library for common platforms.

- primary role is to manage multiple Rust toolchains on a single system, making it easier to work work with
  different versions of Rust and its dependencies.

- Features:
    - toolchain management: ( install, update, manage multiple Rust toolchains including stable, beta, alpha
      and nightly channels )
    - version management: switch between different version of Rust
    - Dependencies management: manages dependencies for each toolchain, ensuring that the correct version of
      dependencies are used for each project.
    - Override management: temporarily override the default toolchain for specific project or directory.

- Usage: rustup [OPTIONS] [+toolchain] [COMMAND]

    Commands:
      show         Show the active and installed toolchains or profiles
      update       Update Rust toolchains and rustup
      check        Check for updates to Rust toolchains and rustup
      default      Set the default toolchain
      toolchain    Modify or query the installed toolchains
      target       Modify a toolchain's supported targets
      component    Modify a toolchain's installed components
      override     Modify toolchain overrides for directories
      run          Run a command with an environment configured for a given toolchain
      which        Display which binary will be run for a given command
      doc          Open the documentation for the current toolchain
      man          View the man page for a given command
      self         Modify the rustup installation
      set          Alter rustup settings
      completions  Generate tab-completion scripts for your shell
      help         Print this message or the help of the given subcommand(s)

- Installation:
    $ rustup install stable
    $ rustup install beta
    $ rustup install nightly

- Set default toolchain:
    $ rustup default stable
    $ rustup default beta
    $ rustup default nightly

- list installed toolchains:
    $ rustup toolchains list

- update rustup and toolchains:
    $ rustup self update
    $ rustup update

- Override Default toolchain for a project:
    $ rustup override set nightly

- Usecase:
    - development
    - project management
    - CI/CD pipelines: install and manage specific version for CI/CD pipelines.
    - Testing and Debugging.


