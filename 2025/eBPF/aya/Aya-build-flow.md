# `aya-build`

To set up the **"Modern Aya"** workflow (v0.13.1), you’ll use `aya-build` to automate the compilation of 
your eBPF code. 

This ensures that when you run your user-space Rust app, the eBPF bytecode is automatically recompiled if 
you made changes.

Here is how to implement the `build.rs` approach:

---

## 1. Add Dependencies

First, ensure your user-space `Cargo.toml` has `aya-build` in its `[build-dependencies]`.

```toml
[build-dependencies]
aya-build = "0.1.0" # Or the version matching your Aya install
anyhow = "1.0"

```

## 2. Create the `build.rs`

In the **root of your user-space crate** (the same folder as your `src` and `Cargo.toml`), create a file 
named `build.rs`.

```rust
use aya_build::project::Project;
use std::path::PathBuf;

fn main() {
    // 1. Point to your eBPF crate directory
    let mut bpf_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    bpf_path.pop(); // Go up to workspace root
    bpf_path.push("myapp-ebpf");

    // 2. Tell aya-build to compile it
    // This will run 'cargo build' inside the eBPF crate
    let project = Project::new(&bpf_path);
    project.build().expect("Failed to build eBPF program");

    // 3. Optional: Tell Cargo to re-run if eBPF files change
    println!("cargo:rerun-if-changed=../myapp-ebpf/src");
}

```

---

## 3. The Resulting Workflow

Now that you have this set up, your development cycle looks like this:

1. **Modify BPF Code:** Change something in `myapp-ebpf/src/main.rs`.
2. **Run App:** Simply run `cargo run --package myapp`.
3. **Automatic Build:**
* `build.rs` triggers `aya-build`.
* `aya-build` compiles your Rust eBPF into a `.elf` file (usually targeting `bpfel-unknown-none`).
* The user-space app then picks up the new `.elf` and loads it into the kernel.



---

## Why keep Codegen separate?

You *could* technically put the `aya-tool` code-generation logic inside this same `build.rs`, but I 
strongly advise against it for two reasons:

1. **Permissions:** 
`aya-tool` needs to read `/sys/kernel/btf/vmlinux`, which often requires **root/sudo**. 
If you put it in `build.rs`, every time you hit "Save" and your IDE (Rust-Analyzer) tries to check your code,
it might fail because it doesn't have permissions to read the kernel BTF.
2. **Frequency:** 
You only need to generate bindings when you want to use a *new* kernel struct. 
You compile your BPF code *hundreds* of times.

### Pro-Tip: The "Task" alias

If you don't want a full `xtask` crate but want a shortcut, add this to your `.cargo/config.toml`:

```toml
[alias]
codegen = "run --package my-codegen-tool" # If you made a small helper
# OR simply use a shell alias

```

---

In Aya **0.13.x**, the "magic" that makes your eBPF code compile when you run your user-space app is 
actually a combination of several tools working in the background. 

Understanding these "finer details" will help you troubleshoot when the compiler inevitably complains 
about the BPF verifier or missing types.

Here is the breakdown of the build pipeline:

---

### 1. The Toolchain: Why `nightly`?

Even in 2026, compiling Rust to BPF requires **Rust Nightly**. 
This is because eBPF is a "no-std" environment that relies on several unstable compiler features:

* **`build-std`**: 
    The compiler must recompile the `core` library specifically for the `bpfel-unknown-none` target.
* **Inline Assembly**: 
    BPF-specific assembly instructions are often used for optimization.

### 2. The Linker: `bpf-linker`

The most important "hidden" tool is **`bpf-linker`**.

When you run a build, the Rust compiler (`rustc`) doesn't actually know how to create a valid BPF ELF file 
on its own. 

It generates `LLVM` bitcode, which it then hands off to `bpf-linker`.

**What `bpf-linker` does for you:**

* Dead Code Elimination: 
    It aggressively removes any Rust code that the BPF verifier would hate (like stack-heavy functions you 
    aren't using).

* BTF Generation: 
    It embeds the "Debug Info" (BTF) into the binary, allowing for CO-RE (Compile Once – Run Everywhere).

* Section Naming: 
    It ensures your functions are placed in the correct ELF sections (e.g., `xdp`, `kprobe/`) so the kernel 
    knows where to attach them.

---

### 3. `aya-build` vs. Manual Compilation

In your `build.rs`, when you call `project.build()`, it is essentially automating this complex terminal 
command for you:

```bash
cargo +nightly build \
    -Z build-std=core \
    --target=bpfel-unknown-none \
    --release \
    --package myapp-ebpf

```

### 4. The `common` Crate: The "Secret Sauce"

You’ll notice the template creates a `myapp-common` crate. 
This is vital for **Type Safety**.

* Shared Structs:

If you define a struct for a BPF Map in `common`, both your eBPF code (kernel-space) and your Rust app 
(user-space) will use the exact same memory layout.

* No-Std Compatibility: 
The `common` crate is marked `![no_std]`, making it legal to import into the restricted eBPF environment.

---

### Summary of the Lifecycle

| Step | Component | Action |
| --- | --- | --- |
| 1. Trigger | `cargo run` | Triggers user-space `build.rs`. |
| 2. Compile | `aya-build` | Invokes `rustc` nightly for the eBPF crate. |
| 3. Link | `bpf-linker` | Packages bitcode into a BPF-compliant ELF object. |
| 4. Load | `aya` (Library) | Reads the ELF, performs relocations (via BTF), and calls `bpf()`. |
| 5. Verify | Linux Kernel | The **Verifier** checks the code for loops/safety and JITs it. |

---

### Pro-Tip for Troubleshooting

If your build fails with a "Verifier Error" during `cargo run`, you can see the kernel's full reasoning 
by increasing the log level in your user-space loader:

```rust
let mut bpf = EbpfLoader::new()
    .extension("my_prog")
    .load_file("path/to/bpf.o")?;

```

## example eBPF  using your generated `task_struct` bindings 

This example brings everything together: your generated bindings, aya-log for debugging, and a Tracepoint 
to hook into the kernel.

We will use the `sys_enter_execve` tracepoint. 
This fires every time a process tries to execute a new program.

1. The eBPF Code (myapp-ebpf/src/main.rs)
   `bpf_get_current_task()`  to get a pointer to the current `task_struct`, 
   then uses your bindings to read the parent's PID.

```rust 
#![no_std]
#![no_main]

mod bindings; // Your generated task_struct lives here
use bindings::task_struct;

use aya_ebpf::{
    helpers::{bpf_get_current_task, bpf_probe_read_kernel},
    macros::tracepoint,
    programs::TracePointContext,
};
use aya_log_ebpf::info;

#[tracepoint]
pub fn track_exec(ctx: TracePointContext) -> u32 {
    match try_track_exec(ctx) {
        Ok(ret) => ret,
        Err(_) => 1,
    }
}

fn try_track_exec(ctx: TracePointContext) -> Result<u32, i64> {
    // 1. Get the pointer to the current task_struct from the kernel
    let task_ptr: *const task_struct = unsafe { bpf_get_current_task() as *const task_struct };

    // 2. Read the parent's TGID (PID) from the task_struct
    // We use bpf_probe_read_kernel because task_ptr is a kernel pointer
    let parent_tgid = unsafe {
        let parent_ptr = bpf_probe_read_kernel(&(*task_ptr).real_parent)?;
        bpf_probe_read_kernel(&(*parent_ptr).tgid)?
    };

    // 3. Get the process name (comm) of the current process
    let mut comm = [0u8; 16];
    let _ = ctx.command(&mut comm);
    let comm_str = core::str::from_utf8(&comm).unwrap_or("unknown");

    info!(&ctx, "EXEC: process '{}' (parent PID: {})", comm_str, parent_tgid);

    Ok(0)
}
```

2. The User-Space Loader (myapp/src/main.rs)

   Loads the code into the kernel and attaches it to the `execve` syscall.

```rust 
use aya::programs::TracePoint;
use aya::{include_bytes_aligned, Ebpf};
use aya_log::EbpfLogger;
use log::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::init();

    // Load the compiled eBPF ELF
    let mut bpf = Ebpf::load(include_bytes_aligned!(
        "../../target/bpfel-unknown-none/release/myapp"
    ))?;

    // Initialize logging
    EbpfLogger::init(&mut bpf)?;

    // Attach to the sys_enter_execve tracepoint
    let program: &mut TracePoint = bpf.program_mut("track_exec").unwrap().try_into()?;
    program.load()?;
    program.attach("syscalls", "sys_enter_execve")?;

    info!("Waiting for execve events... Press Ctrl-C to exit.");

    // Keep the program running
    tokio::signal::ctrl_c().await?;
    Ok(())
}
```
3. How to Run It

- 1. Generate Bindings: (If you haven't yet)

    `aya-tool generate task_struct > myapp-ebpf/src/bindings.rs`

- Build and Run:
    `RUST_LOG=info cargo run --package myapp`

-  When you run a command like ls in a terminal:
    - The kernel hits the sys_enter_execve tracepoint.
    - Our eBPF program wakes up.
    - It finds the task_struct for the process.
    - It follows the pointer to real_parent (the shell that launched it) to grab the parent's PID.
    - aya-log ships that data to your terminal via the ring buffer.

Note:   `bpf_probe_read_kernel`:

In eBPF, you cannot directly dereference pointers like `task->real_parent`. 
The BPF Verifier will stop you because that memory might not be loaded (paged out). 
You must always use `bpf_probe_read_kernel` to safely copy that data into the eBPF stack before using it.








