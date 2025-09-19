# Tracefs  Lister: tool to list *Subsystems* and *Subsystem:Events* 

A small Rust CLI tool to **list kernel tracing events** from `/sys/kernel/debug/tracing/events`. 

Built using [`clap`](https://crates.io/crates/clap) and [`colored`](https://crates.io/crates/colored), 
this tool supports listing **all tracing subsystems** or **events within a specific subsystem**.

> ⚠️ **Requires `sudo` privileges** to access the tracefs filesystem.

---

## ✨ Features

- List all available tracing subsystems
- List events for a specific subsystem
- Colored, formatted terminal output for easy readability
- Built with safe and modern Rust

---

## Dependencies

This tool uses the following crates:

- [`clap`](https://docs.rs/clap) – For command-line argument parsing
- [`colored`](https://docs.rs/colored) – For colored terminal output

Add them to your `Cargo.toml`:

```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
colored = "2"
````

---

## Usage

### Build

```bash
cargo build --release
```

### Run

> Requires `sudo` since it accesses the debugfs (`/sys/kernel/debug/tracing`)

```bash
sudo ./target/release/tracefs-lister
```

#### List All Subsystems

```bash
sudo ./tracefs-lister --list
```

#### List Events in a Specific Subsystem (e.g., `sched`)

```bash
sudo ./tracefs-lister --list sched
sudo ./tracefs-lister --list net
```

---

## How It Works

### CLI Argument Handling

The tool uses `clap::Parser` to define a single optional argument `--list`:

```rust
#[derive(Parser, Debug)]
struct Args {
    #[arg(long, num_args(0..=1), default_missing_value = "all")]
    list: Option<String>,
}
```

* `--list` with no value: defaults to `"all"`.
* `--list <subsystem>`: lists events for that subsystem.

### Event Listing

Events are fetched from the tracefs directory:

* **All subsystems:** `/sys/kernel/debug/tracing/events/`
* **Specific subsystem:** `/sys/kernel/debug/tracing/events/<subsystem>/`

It then reads all subdirectories and prints them, with colored formatting based on the mode (all vs specific).

---

## 📂 Example Output

### All Subsystems

```bash
$ sudo ./tracefs-lister --list

System supported subsystems for tracing: (select the subsystem of interest)
  block
  irq
  sched
  syscalls
  ...

```

### Specific Subsystem (e.g., `sched`)

```bash
$ sudo ./tracefs-lister --list sched

Listed tracing events for subsystem: sched

  sched_switch
  sched_wakeup
  sched_migrate_task
  ...
```

---

## Permissions

This tool reads from `/sys/kernel/debug/tracing`, which is typically only accessible by root or users with specific capabilities.

Use `sudo` when running this tool, or configure appropriate permissions if integrating into other tooling.

---
## Cross Build:

- Install rust nightly and cross toolchain 
- Add the target 
    `rustup target add aarch64-unknown-linux-gnu`

Fedora: 
- Default cross toolchain for aarch64 on Fedora lacks sysrootfs.
- use 
    `dnf copr enable lantw44/aarch64-linux-gnu-toolchain`
    `dnf install aarch64-linux-gnu-{binutils,gcc,glibc}`

```bash 
cargo build --release --target aarch64-unknown-linux-gnu --config=target.aarch64-unknown-linux-gnu.linker=\"aarch64-linux-gnu-gcc\"
```

Or use the below way to use cargo build for cross building :

- **include the linker configuration in `Cargo.toml`** and optionally use a `.cargo/config.toml` 
  file to specify the target and linker.

---

### Recommended Setup

Instead of putting the linker config directly into `Cargo.toml`, which doesn’t support build-specific 
settings like linker paths, Rust projects use a `.cargo/config.toml` file.

---

1. Project Directory Structure

```
your-project/
├── Cargo.toml
├── src/
│   └── main.rs
└── .cargo/
    └── config.toml   <-- Add this file
```

---

2. `.cargo/config.toml` — Configure Cross-Compilation

Create a file at `.cargo/config.toml` with the following content:

```toml
[build]
target = "aarch64-unknown-linux-gnu"

[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

---

3. Build with Cross-Compilation (Now Just)

```bash
cargo build --release
```

This will automatically build for `aarch64-unknown-linux-gnu` using `aarch64-linux-gnu-gcc`.

---

Notes

* Make sure you have the target installed:

```bash
rustup target add aarch64-unknown-linux-gnu
```

* Ensure the cross linker (`aarch64-linux-gnu-gcc`) is installed:

```bash
sudo apt install gcc-aarch64-linux-gnu
```
---

Optional: Cargo.toml Metadata

You **can** add a section to `Cargo.toml` for documentation purposes, though it doesn't affect behavior:

```toml
[package.metadata.cross]
default-target = "aarch64-unknown-linux-gnu"
```
But actual behavior is controlled via `.cargo/config.toml`.
