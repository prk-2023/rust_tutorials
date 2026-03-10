# `cargo-xtask`

Note: For howto write the xtask/src/main.rs : refer below and Interface `std::process::Command` for more.

## What is `cargo xtask`:

In Rust eco-system "xtask" is a convention ( note its not a official Cargo feature ) for organizing project
automation tasks inside a Rust workspace. 

In Other languages we generally use make, bash, python to perform some automation tasks as required.

Instead of using other tools for automation tasks, Rust's approach is by creating a small Rust-binary (which
we call `xtask`), this binary performs the automation tasks such as :
    - formatting
    - linting 
    - building docs 
    - code generation 
    - releasing 
    - CI helpers 
    - Running custom workflows 

You then run it like:

```bash  
cargo run -p xtask -- <task> 

# example:
cargo run -p xtask -- fmt
cargo run -p xtask -- ci
cargo run -p xtask -- codegen
```

So `cargo xtask` is a design pattern used to automate custom tasks in Rust projects. It allows you to write
your own build scripts, automation and CI logic in Rust rather then using other tools like Bash, Python,
Makefiles.

This lets you write **automation scripts in Rust itself**.

## Why `xtask` exists:

Rust projects needed a way to handle **project-specific tooling tasks**. 
Before xtask, projects commonly used:

* **Makefiles**
* **shell scripts**
* **Python scripts**
* CI YAML duplication

These approaches had drawbacks:

| Problem              | Example                         |
| -------------------- | ------------------------------- |
| Non-portable scripts | Bash scripts failing on Windows |
| Multiple languages   | Rust + Python + shell           |
| Hard to maintain     | Complex Makefiles               |
| Poor IDE support     | Hard to debug scripts           |

Rust developers wanted:

* type-checked scripts
* cross-platform automation
* reuse of Rust libraries
* easier debugging
* unified tooling language

The xtask pattern solves this.

---

## Origin and History

The **xtask pattern was popularized by**  `Alex Kladov` around **2019**.

He described the pattern in a blog post titled: **xtask — command-line tasks for Rust projects**

The idea spread quickly across the Rust ecosystem and is now used in many major projects.

Notable adopters include:

* rust-analyzer
* Bevy
* Tokio
* Deno (Rust core tooling)

Cargo itself **never officially added xtask** it emerged as a **community convention**.

---

## Why It's Called “xtask”

The name **xtask** was intentionally chosen because:

* Cargo reserves common names like `build`, `test`, etc.
* `x` is used as a **catch-all namespace for custom tasks**

So:

```
xtask = "extra tasks"
```

It avoids collisions with Cargo subcommands.

## why use it:

- *Pure Rust*: You don't need to learn other scripting language.
- *dependency management*: You can use any crate from crate.io in your build scripts. 
- *Zero extra binaries*: The only requirement to run `xtask` is rust installed system. 
- *Cross platform*: Since its rust, your scripts will work in all OS that support rust.

--- 

## Typical Project Structure ( How to Implement )

To set this up you usually organize your project as a Cargo Workspace: 

1. Project Structure:
```
my-project/
├── .cargo/
│   └── config.toml   <-- Where the magic alias lives
├── Cargo.toml        <-- Workspace manifest
├── my-crate/         <-- Your actual library or app
│   └── Cargo.toml
└── xtask/            <-- Your automation "script" crate
    ├── Cargo.toml
    └── src/
        └── main.rs
```

2. Create Alias:

In `.cargo/config.toml`, add an alias that tells Cargo to run the `xtask` crate whenever you type cargo xtask:

```toml 
[alias]
xtask = "run --package xtask --"
```

3. Write the actual `xtask` logic:

In `xtask/src/main.rs`, you write a standard Rust CLI. 
Here is a simple example using no external crates:

```rust 
use std::env;
use std::process::Command;

fn main() {
    let task = env::args().nth(1);
    match task.as_deref() {
        Some("dist") => do_dist(),
        _ => print_help(),
    }
}

fn do_dist() {
    println!("Building release distribution...");
    let status = Command::new("cargo")
        .args(&["build", "--release"])
        .status()
        .expect("Failed to run cargo build");
    
    if status.success() {
        println!("Distribution ready!");
    }
}

fn print_help() {
    println!("Usage: cargo xtask [task]");
    println!("Tasks: dist");
}
```

---
Example 2: xtask

`xtask/src/main.rs`

```rust
use std::process::Command;

fn main() {
    let task = std::env::args().nth(1).unwrap();

    match task.as_str() {
        "fmt" => {
            Command::new("cargo")
                .arg("fmt")
                .status()
                .unwrap();
        }
        "ci" => {
            Command::new("cargo")
                .args(["clippy", "--all-targets"])
                .status()
                .unwrap();
        }
        _ => panic!("unknown task"),
    }
}
```

Run:

```
cargo run -p xtask -- fmt
```

---

### Easier complex automation

Example tasks:

* generating bindings
* building WASM
* packaging releases
* verifying documentation
* running integration test matrices

---

## How xtask Compares to Alternatives

| Tool           | Language | Pros                | Cons                     |
| -------------- | -------- | ------------------- | ------------------------ |
| Makefile       | Make     | simple              | bad Windows support      |
| Bash scripts   | Bash     | flexible            | Linux-centric            |
| Python scripts | Python   | readable            | extra runtime dependency |
| **xtask**      | Rust     | type-safe, portable | compile step             |

---

# Example Projects

---

Imagine you are developing on an x86_64 machine, but you want to build and send a "Hello World" binary to an
ARM64 device (like a Raspberry Pi or an AWS Graviton instance).

Doing this manually requires:

- Installing the `aarch64` target via `rustup`.

- Finding and pointing to an `aarch64` GCC linker.

- Running a long, messy command like:

`CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER=aarch64-linux-gnu-gcc cargo build --target aarch64-unknown-linux-gnu`

We can handle this with xtask: By simply building it as `cargo xtask build` as below:

### Phase 1: Preparation

Before we code, you need the cross-compilation tools on your machine.

1. **Add the target:** 
    `rustup target add aarch64-unknown-linux-gnu`

2. **Install the linker:** 
    You need a C linker that understands ARM. 
    On Ubuntu/Debian, run: `sudo apt install gcc-aarch64-linux-gnu`.

---

### Phase 2: Project Layout

Create a directory structure that looks like this:

1. Create a new folder `my-project`.
2. Inside, create two sub-folders: `hello` (your app) and `xtask` (your automation).

---

### Phase 3: The Workspace Configuration

In the root directory (`my-project/`), create a `Cargo.toml` to link everything together:

```toml
[workspace]
members = ["hello", "xtask"]
resolver = "2"

```

---

### Phase 4: Create the "Hello World" App

1. Inside `my-project/hello/`, run `cargo init`.
2. Your `hello/src/main.rs` should look like this:

```rust
fn main() {
    println!("Hello from ARM64!");
}

```

---

### Phase 5: Create the `xtask` (The Automator)

1. Inside `my-project/xtask/`, create a `Cargo.toml`:

```toml
[package]
name = "xtask"
version = "0.1.0"
edition = "2021"

```

2. Inside `my-project/xtask/src/main.rs`, write the logic to invoke the cross-compiler:

```rust
use std::process::Command;

fn main() {
    println!(">>> Running cross-compile task...");

    // This is the equivalent of running a complex shell command
    let status = Command::new("cargo")
        .args([
            "build",
            "--package", "hello",
            "--target", "aarch64-unknown-linux-gnu",
            "--release"
        ])
        // We tell cargo which linker to use for this target
        .env("CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER", "aarch64-linux-gnu-gcc")
        .status()
        .expect("Failed to build");

    if status.success() {
        println!("Build complete! Find your binary in target/aarch64-unknown-linux-gnu/release/hello");
    }
}

```

---

### Phase 6: The "Cargo Alias"

To make it feel like a built-in command, create a file at `my-project/.cargo/config.toml`:

```toml
[alias]
xtask = "run --package xtask --"

```

---

### Phase 7: Execute!

Now, from the root of your project, simply run:

```bash
cargo xtask

```

**What just happened?**

1. `cargo xtask` looked at your alias and ran `cargo run --package xtask --`.
2. Cargo compiled your `xtask` program.
3. The `xtask` program executed your Rust logic, which called `cargo build --target aarch64...`.
4. Your "Hello World" binary was cross-compiled successfully.

---

**You have now successfully built a tool that manages your project's build process.** 

## Expand from here:

Expand xtask to automatically copy the binary to your ARM device via scp after the build finishes?
Target device ip is 10.10.10.27 and root user and no password

Once your `xtask` handles both compilation and deployment, you move from "a script that compiles code" to a 
full **CI/CD pipeline right on your local machine.**

### 1. The Strategy

To automate this, we will add a `deploy` function to your `xtask/src/main.rs`. 
We will use `std::process::Command` to invoke `scp`.

**Note:** For "no password" `scp` to work, you must have your local SSH public key 
(`~/.ssh/id_rsa.pub`) copied to your ARM device's `~/.ssh/authorized_keys` file. 
If you haven't done this, run `ssh-copy-id root@10.10.10.27` once before trying the automation.

### 2. Updating your `xtask/src/main.rs`

You can now create a "chained" command that builds the binary, then automatically copies it.

```rust
use std::process::Command;

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.get(0).map(|s| s.as_str()) {
        Some("deploy") => {
            build_arm(); // 1. Build
            deploy_to_device(); // 2. Copy
        }
        _ => println!("Usage: cargo xtask [deploy]"),
    }
}

fn build_arm() {
    // (Your existing build logic here...)
    let status = Command::new("cargo")
        .args(["build", "--target", "aarch64-unknown-linux-gnu", "--release"])
        .env("CARGO_TARGET_AARCH64_UNKNOWN_LINUX_GNU_LINKER", "aarch64-linux-gnu-gcc")
        .status()
        .expect("Failed to build");
    
    assert!(status.success(), "Build failed!");
}

fn deploy_to_device() {
    println!(">>> Copying binary to ARM device...");

    let status = Command::new("scp")
        .args([
            "target/aarch64-unknown-linux-gnu/release/hello", // Source
            "root@10.10.10.27:/usr/local/bin/hello",           // Destination
        ])
        .status()
        .expect("Failed to run scp");

    if status.success() {
        println!("Deployment successful!");
    } else {
        eprintln!("Deployment failed!");
    }
}

```

### 3. Why this is powerful

By keeping the build and deploy steps in a single `xtask` command, you ensure consistency:

* **Atomic operations:** 
    You never accidentally deploy a stale binary because the `deploy` task *always* triggers a fresh build 
    first.

* **Version Control:** 
    The deployment path (e.g., `/usr/local/bin/hello`) is now hardcoded in your repo. 
    New team members don't have to guess where the binary should go; they just run `cargo xtask deploy`.

### 4. Taking it further (The Pro Way)

If you find yourself copying many files or if the deployment is slow, swap `scp` for **`rsync`**.
`rsync` is a "smarter" tool that only copies the *changes* (delta) between files, making it much faster for large binaries.

You would simply replace the `Command::new("scp")` with:

```rust
Command::new("rsync")
    .args(["-avz", "target/aarch64-unknown-linux-gnu/release/hello", "root@10.10.10.27:/usr/local/bin/hello"])
    .status()?;

```


---------------

# `std::process:Command` master for xtask tasks:


To master `xtask`, **`std::process::Command` is your primary interface.**

With `xtask`, you are essentially writing a "wrapper program" that orchestrates the operating system.
`Command` is the bridge between your safe, static Rust code and the messy, dynamic world of shell commands, 
environment variables, and system tools.

### 1. The `Command` Toolbox

When you are building your `xtask` tools, you don't need to learn the whole standard library, 
but you **do** need to master these four components of `Command`:

| Component | Purpose | Use Case in `xtask` |
| --- | --- | --- |
| **`new(program)`** | Defines the base command. | `Command::new("cargo")` or `Command::new("docker")`. |
| **`args(&[&str])`** | Passes flags and options. | Adding `--release`, `--target`, or `--features`. |
| **`env(key, val)`** | Sets environment variables. | **Crucial:** Setting cross-compilation linkers (`CC`, `LD`) or `RUSTFLAGS`. |
| **`status()` / `output()**` | Runs the command and waits for results. | Checking if your build succeeded or capturing the output text. |

---

### 2. The "Mastery" Pattern: Handling Results

A common beginner mistake is ignoring the result of `Command`. 
A true `xtask` master treats external commands as *fallible*.

Below is the robust way to execute a command:

```rust
use std::process::Command;

fn run_command(cmd: &str, args: &[&str]) -> anyhow::Result<()> {
    let status = Command::new(cmd)
        .args(args)
        .env("MY_VAR", "value") // Injecting config
        .status()?; // Spawns the process and waits

    if status.success() {
        Ok(())
    } else {
        // Return a meaningful error if the build fails
        Err(anyhow::anyhow!("Command '{}' failed with status {}", cmd, status))
    }
}

```

### 3. Why `status()` vs `output()`?

This distinction is vital for your automation tools:

* **`status()`**: 
    - Use this when you just want to run a build and let the user see the progress in their terminal. 
    - It pipes `stdout` and `stderr` directly to the console. **(Best for standard builds)**.

* **`output()`**: 
    - Use this when you need to *capture* the result. 
    - For example, if you want to run `git rev-parse HEAD` to get the current commit hash so you can use it
      in your code. 
    - You capture the output as a `Vec<u8>` or `String`. **(Best for gathering info)**.

---

### 4. Important 

* **Pathing:**  
    - Remember that `Command` is executed from the working directory of the `xtask` process (usually the
      root of your workspace). If you need to run a command inside a specific sub-crate: 
      - use `.current_dir("path/to/crate")`.

* **Streaming:** 
    - `status()` shows everything, but sometimes you want to hide the noise or log it to a file. 
    - To do this, you have to use `stdout(Stdio::piped())` and handle the buffers yourself (this is "Pro"
      level `xtask`).

* **OS Differences:** 
    - While `Command` abstracts a lot, remember that your `xtask` code might run on Windows (PowerShell/CMD) or Linux (Bash/Sh). 
    - `Command::new("sh").arg("-c").arg("...")` is often used to run complex shell pipelines, but keep in
      mind this hurts cross-platform compatibility.

### Summary for your Learning Path

If you want to move from "beginner" to "adept" with `xtask`:

1. **Stop writing raw `Command` calls.** Start creating small helper functions (like the `run_command`
   example above) so your code doesn't get cluttered.
2. **Use `anyhow` or `eyre` crates.** These make error handling in your `xtask` much cleaner.
3. **Learn `std::process::Stdio`.** This allows you to redirect input/output—essential if you ever want your
   `xtask` to "silently" run a build or capture logs to a file.
