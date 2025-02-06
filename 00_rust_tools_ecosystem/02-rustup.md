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

---

# Switch between different Rust installations:

Here's a step-by-step guide to installing and switching between **stable**, **beta**, and **nightly** 
Rust toolchains using `rustup` on Linux:

---

### **1. Install `rustup`**
First make sure you install rust and environment is set to use.
```bash
source "$HOME/.cargo/env"
```

---

### **2. Install Stable, Beta, and Nightly Toolchains**
Install all three toolchains:
```bash
rustup install stable
rustup install beta
rustup install nightly
```
Verify installed toolchains:
```bash
rustup show
```

---

### **3. Switch Between Toolchains**
#### **Set a Global Default Toolchain**
```bash
# Set stable as the default
rustup default stable

# Set beta as the default
rustup default beta

# Set nightly as the default
rustup default nightly
```

#### **Use a Toolchain Temporarily (Per Project)**
Override the toolchain for a specific project directory:
```bash
cd ~/my_project
rustup override set beta  # Uses beta only in this directory
```

#### **Use a Toolchain for a Single Command**
Prefix any `cargo` or `rustc` command with `+toolchain`:
```bash
cargo +nightly build  # Build with nightly
rustc +beta main.rs   # Compile with beta
```

---

### **4. Update All Toolchains**
Update all installed toolchains to their latest versions:
```bash
rustup update
```

---

### **5. Check Active Toolchain**
```bash
rustup show active-toolchain  # Shows the current default
rustc --version               # Check the active Rust version
```

---

### **6. Remove a Toolchain**
To uninstall a toolchain (e.g., nightly):
```bash
rustup uninstall nightly
```

---

### **Key Commands Summary**
| **Command**                     | **Description**                                     |
|---------------------------------|-----------------------------------------------------|
| `rustup install <toolchain>`    | Install a toolchain (stable/beta/nightly).          |
| `rustup default <toolchain>`    | Set the global default toolchain.                   |
| `rustup override set <toolchain>` | Set a per-project toolchain.                      |
| `cargo +<toolchain> <command>`  | Run a command with a specific toolchain.            |
| `rustup update`                 | Update all installed toolchains.                    |
| `rustup show`                   | List installed toolchains and components.           |

---

### **Example Workflow**
1. **Develop with Stable**:
   ```bash
   rustup default stable
   cargo build
   ```

2. **Test Beta Features**:
   ```bash
   cargo +beta test
   ```

3. **Experiment with Nightly**:
   ```bash
   rustup override set nightly
   cargo build --features unstable
   ```

---

### **Notes**
- **Nightly**: Required for experimental features (e.g., `#![feature(...)]` in code).
- **Beta**: Updated every 6 weeks as a preview of the next stable release.
- **Stable**: Recommended for production use.

By using `rustup`, you can seamlessly switch between Rust versions while keeping all toolchains updated. 🦀
