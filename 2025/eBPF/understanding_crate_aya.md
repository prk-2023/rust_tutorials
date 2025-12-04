 # Aya: ( Understanding Aya )
 
 
 ## Introduction 
 Crate Aya is a complex Rust ecosystem as it spans 2 world:
 - User-Space (host) : loading, interacting with and managing eBPF programs. 
 - eBPF bytecode/ kernel side : writing restricted, verifier-safe Rust code that compiles to eBPF target.
 
 Aya Proj is divided into sub-crates with each split responsible for a given task. 
 
 To Understand the Crate deeply, the keu is to approach it in layers:
 - from the "outer shell" (aya)  into "inner core" (aya-ebpf /kernel contraints)
 
### Generate docs Or refer from the docs.rs site:

 cargo xtask docs
   Generate the docs in aya/site/
   contains 4 basics groups:
 .........
 Documenting aya v0.13.2 ( aya/aya )
 Documenting aya-ebpf v0.1.2 ( aya/ebpf/aya-ebpf )
 Documenting aya-log v0.2.2 ( aya/aya-log )
 Documenting aya-log-ebpf v0.1.2 (~/Aya/aya/ebpf/aya-log-ebpf)

 The documents contain 2 parts for userspace and 2 parts for kernel space.
 
## Understand the Purpose of Each Aya Crate

Functional explanation that helps while reading the Code:

### aya (user space loader + runtime)

This is the crate you use in normal Rust programs running on Linux.

It contains:
* program loading (TC, XDP, cgroup, kprobe, uprobes, tracepoints, sockets, perf events)
* managing eBPF maps (hashmaps, arrays, perf ring buffer, BPF iter)
* BTF parsing
* eBPF object parsing (ELF)
* pinning / persistent maps
* program attaching / detaching
* low-level syscall wrappers

You can think of aya as:

The orchestrator that loads and controls eBPF programs.

### aya-ebpf (kernel-side library)

This library helps to generate `no_std` , `no_main` bytecode program for the BPF Virtual machine inside the kernel. 

It provides:

* no_std environment
* helpers bindings (bpf_probe_read, get_current_pid_tgid, etc.)
* map access APIs (HashMap, Array, PerfEventArray, …)
* macros for writing eBPF entry points
* verifier-friendly wrapper functions

Think of it as:

A toolkit that gives you safe(ish) Rust to write eBPF programs.

This is the most critical piece to understand for writing actual BPF programs.

### aya-log (userspace)

Receives logs from the kernel eBPF code.
Integrates with Rust’s log crate.

### aya-log-ebpf (kernel space)

Used inside the eBPF program to emit log messages to userspace.


## How to Study the aya code:

Suggested Order for Studying the Code
You will understand Aya most efficiently in this order:

### **1. Start with a simple example**

Specifically:

* **xdp** examples
* **tracepoint** examples
* **kprobe** examples

Run them and understand the end-to-end pipeline:

* Build eBPF object
* Load it into kernel
* Attach the program
* Read data from a map/register events

You will see which parts of the `aya` crate are used.

---

### **2. Study the userspace workflow (`aya` crate)**

I suggest reading these modules in order:

#### **1. programs/**

Contains types like:

* `Xdp`
* `KProbe`
* `TracePoint`
* `TC`
* `SocketFilter`
* `UProbe`

Read:

* how program loading works (`EbpfLoader`)
* how each program type attaches (syscalls, BPF_PROG_ATTACH, link types)
* how maps are created/resolved
* how pinning is implemented

---

#### **2. maps/**

Important for understanding BPF map operations:

* `HashMap`
* `Array`
* `RingBuffer`
* `PerfEventArray`
* `BpfMap`

Learn:

* how maps are opened
* how FD ownership is handled
* how map names & BTF types are resolved

This knowledge is required when writing kernel-side BPF code, because the map definitions must match userspace expectations.

---

#### **3. sys/**

Contains low-level BPF syscalls:

* `bpf_load_program`
* `bpf_map_create`
* `bpf_map_update_elem`
* `bpf_map_lookup_elem`

This is where Aya interfaces directly with the kernel using:

* `libc::syscall`
* raw FFI to kernel APIs

This module explains how Aya avoids `libbpf`.

---

#### **4. btf/ and util/**

Understanding BTF is optional initially, but crucial later.

---

### **3. Study kernel-side code (`aya-ebpf`)**

This crate is **no_std**, so it looks very different.

I recommend reading these:

#### **1. macros.rs**

Learn how entrypoints for eBPF programs are generated:

* `#[kprobe]`
* `#[xdp]`
* `#[tp]`
* `#[map]`

This is where type checking and naming conventions happen.

---

#### **2. maps/ folder**

Contains data structures used inside the BPF program:

* HashMap
* Array
* PerCPUArray
* PerfEventArray
* RingBuf

Important:

* The struct definitions must match the kernel’s expectations (map def layout)
* Using these incorrectly will cause verifier rejection

---

#### **3. helpers.rs**

This is where the actual BPF helper functions live.
Each helper corresponds to something in:
`include/uapi/linux/bpf.h`

Examples:

* `bpf_map_lookup_elem`
* `bpf_perf_event_output`
* `bpf_probe_read`
* `bpf_ktime_get_ns`

Understanding these is mandatory for deep mastery.

---

#### **4. verifer_compliance/** (if present)

Aya often wraps certain operations in safe helpers to satisfy verifier constraints.

---

## 3. How to Learn Aya the Smart Way (Recommended Path)

Here’s the optimal sequence to really master it:

### **Step 1: Build and run a tiny complete example**

Follow the simplest example:

* a tracepoint program that prints data
* with a map to exchange data with userspace

Understanding end-to-end flow is important.

---

### **Step 2: Trace through the Aya loader (`aya/src/programs/`)**

Learn how Aya:

* parses the ELF file
* identifies program sections (".xdp", ".kprobe", etc.)
* loads bytecode into kernel with BPF_PROG_LOAD
* attaches using different mechanisms

Focus on:

* `programs/common.rs`
* `programs/kprobe.rs`
* `EbpfLoader`

---

### **Step 3: Study Aya-ebpf**

Learn how Rust code gets turned into restricted eBPF code.

Important files:

* `lib.rs`
* `macros.rs`
* `helpers.rs`

---

### **Step 4: Map definitions**

Understand how the map definitions in eBPF crate
match the loading & management in Aya.

---

### **Step 5: Build your own small eBPF program**

Start with:

* XDP packet counter
  or
* A kprobe on `do_sys_open` that records PID.

This helps you understand verifier issues.

---

### **Step 6: Read deeper into verifier logs**

Experiment with verifier rejections.
You cannot master BPF without this painful but necessary step.

---

## 4. Tips for Understanding the Crate Internals

### **1. Draw the architecture**

Put:

```
aya                <-- userspace Rust
aya-log
------------------------------------
kernel boundary
------------------------------------
aya-ebpf           <-- no_std Rust (compiles to BPF)
aya-log-ebpf
```

### **2. Trace a single map through the stack**

Follow:

* its definition in `aya-ebpf`
* how the ELF section is generated
* how Aya loads the section
* how the map FD is created
* how userspace updates/lookups the map

This exercise is incredibly valuable.

### **3. Read the BPF helper definitions**

They are cryptic but essential.

-----------------------------------------------------------

# Repo Structure of Aya (from GitHub)

The aya repo contains many sub-crates: 

Here are some of the most relevant ones, and what they correspond to:

| Sub-directory / crate name | Purpose / Notes                                                                                                                                                                              |
| -------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| `aya/`                     | The main userspace library — responsible for loading, managing, and interacting with eBPF programs from standard Rust code. ([GitHub][1])                                                    |
| `aya-ebpf-macros/`         | Macros used for kernel-side (eBPF) Rust code generation — e.g. to define BPF programs, maps, entrypoints. ([GitHub][1])                                                                      |
| `aya-log/`                 | Userspace logging support for eBPF programs (to receive logs emitted from BPF). ([GitHub][1])                                                                                                |
| `aya-log-ebpf-macros/`     | Kernel-side (eBPF) macros to enable logging from inside BPF code. ([GitHub][1])                                                                                                              |
| `aya-obj/`                 | Low-level object-file parsing library (ELF + BTF + relocation) used internally by Aya — useful if you are doing advanced eBPF object manipulation or analyzing ELF/BTF. ([GitHub][1])        |
| `aya-tool/`                | A helper tool to generate Rust bindings for kernel data structures (via BTF + bindgen) — useful when you need kernel types (e.g. `task_struct`) in Rust for writing eBPF code. ([GitHub][1]) |

Plus some other auxiliary crates (for test-infrastructure, supporting builds, etc.) which you can ignore for now. ([GitHub][1])

---

## Reading Plan — using Real Paths in the Repo

Here’s a refined plan with actual folder names/paths, so you can directly open files in your editor or GitHub.

1. **Get a working example running**

   * Clone the repo, build an example.
   * Try out any example under the root `examples/` (if present) or use the scaffold from `aya-template`. 
   Note: If no root `examples/`, see the documentation via the “Aya Book” for guidance. ([GitHub][2])
   * aya-rs : Aya book has `examples/` clone "https://github.com/aya-rs/book"
   * This step helps you see the full cycle: compile BPF → load → attach → interact.

2. **Read the userspace loader — the `aya/` crate**

   * Inside `aya/`, examine sub-modules such as:

     * `aya/programs/` — where different BPF program types are implemented (XDP, kprobe, tracepoint, etc.).
     * `aya/maps/` — map abstractions (HashMap, Array, RingBuffer, etc.).
     * `aya/sys/` or similar — syscall wrappers that perform the low-level `bpf(2)` syscalls to 
       load programs or create maps.
   * This will show how Aya handles loading, attaching, map creation, and how it uses only libc syscalls 
     (no libbpf). Indeed, the repo declares that — purely Rust + libc. ([GitHub][3])
   * Also check how pinning works, how BTF support is handled (if relevant), and how programs are identified
     inside ELF file.

3. **Study kernel-side macros — `aya-ebpf-macros/`**

   * This crate defines procedural macros that you use in your BPF Rust code (e.g. to mark a function as an 
     XDP handler, or define BPF maps).
   * Look for files like `macros.rs` or similar inside that folder to see how those macros generate code & 
     metadata that the loader expects.

4. **For BPF code: understand map definitions + BPF helpers**

   * Although much of the internal eBPF-safe Rust runtime might be hidden in generated code / macros, it 
     helps to search where generic map abstractions and BPF helper wrappers live.
   * Use combinations of the map abstractions (from userspace) + the macros (kernel-side) to ensure types 
     and layouts line up.

5. **Use `aya-tool/` when you need kernel type definitions**

   * If your BPF program needs kernel structs (like `task_struct`, socket structures, etc.), run `aya-tool` 
     to generate Rust bindings via BTF + bindgen. This avoids hand-writing C or manually maintaining 
     incompatible types. ([aya-rs.dev][4])
   * This is often critical when writing more advanced BPF programs referencing kernel data.

6. **Inspect lower-level object parsing via `aya-obj/` (optional / advanced)**

   * If you ever need to introspect or manipulate the BPF ELF object (e.g. read BTF, relocation info, 
     custom sections), `aya-obj` is the crate to explore. 
     Useful if doing custom loader code, analysis, or tools built around Aya. ([Docs.rs][5])

--------------------------------------------

# Extra Official Resource: “The Aya Book”

In addition to the code, the Aya project maintains “The Aya Book” — an official, continuously updated guide 
for using Aya to build eBPF programs. ([GitHub][2])

I strongly recommend reading it **alongside** browsing the source code: it helps connect how conceptual 
pieces map to real code.

---

##  What This Structure Means for You (as you learn / build)

* Because Aya is **self-hosted (in Rust only)** with multiple crates in one repo, it’s easier to trace how 
  the userspace loader and the eBPF-side macros interact.

* The separation into crates (userspace vs macros vs tool) gives you **clear boundaries**, which is huge 
  when trying to reason about user-space vs kernel-space code.

* If you need kernel types — no need for C: you can generate them automatically with `aya-tool`.

* If you ever need advanced behavior (ELF manipulation, custom loader, static linking), you have modules 
  like `aya-obj`, and the whole repo is Rust — you don’t need to switch context to C or libbpf.

---

**Generate a visual diagram** (ASCII-art or markdown) that shows how the crates/modules connect 
(loader ↔ macros ↔ tool ↔ your code ↔ kernel).

This can be **very useful** especially if you plan to contribute or build non-trivial eBPF programs.


[1]: https://github.com/aya-rs/aya "GitHub - aya-rs/aya: Aya is an eBPF library for the Rust programming language, built with a focus on developer experience and operability."
[2]: https://github.com/aya-rs/book?utm_source=chatgpt.com "GitHub - aya-rs/book: The Aya Book is an introductory book about using the Rust Programming Language and Aya library to build extended Berkley Packet Filter (eBPF) programs."
[3]: https://github.com/aya-rs/aya?utm_source=chatgpt.com "GitHub - aya-rs/aya: Aya is an eBPF library for the Rust programming language, built with a focus on developer experience and operability."
[4]: https://aya-rs.dev/book/aya/aya-tool/?utm_source=chatgpt.com "Using aya-tool - Aya"
[5]: https://docs.rs/aya-obj?utm_source=chatgpt.com "aya_obj - Rust"

--------------------------------------------------------------------

# Aya Crate relations:

 *Aya* architecture,how all crates in the GitHub repo relate to each other, and how user-space and 
 kernel-space pieces communicate.

I’ll give you 
    **(1) a high-level diagram**, 
    **(2) a folder-accurate detailed diagram**, and 
    **(3) a data-flow diagram for a real eBPF program**.

---

## **1. High-Level Aya Architecture Diagram**

```
                    ┌──────────────────────────────────────────┐
                    │                 Userspace                │
                    │ (your Rust application using Aya crate)  │
                    └──────────────────────────────────────────┘
                                      │
                                      ▼
                   ┌───────────────────────────────────┐
                   │               aya/                │
                   │  - Program loading (BPF_PROG_LOAD)│
                   │  - Map creation & interaction     │
                   │  - BTF & ELF parsing (via aya-obj)│
                   │  - Attaching (kprobe, xdp, tp…)   │
                   └───────────────────────────────────┘
                                      │
                                      │ uses
                                      ▼
                   ┌──────────────────────────────────┐
                   │             aya-obj/             │
                   │ - Parses eBPF ELF files          │
                   │ - Handles relocations            │
                   │ - Reads BTF types                │
                   │ - Extracts map definitions       │
                   └──────────────────────────────────┘
                                      │
                                      │
                                      ▼
                   ┌──────────────────────────────────┐
                   │            aya-log/              │
                   │   Handles logs from kernel BPF   │
                   └──────────────────────────────────┘
                                      │
                                      ▼
─────────────────────────────────────────────────────────────────────────────
████████████████████████  KERNEL BOUNDARY (syscalls)  ███████████████████████
─────────────────────────────────────────────────────────────────────────────
                                      │
                       Raw syscalls: bpf(), perf_event_open()
                                      │
                                      ▼
                   ┌──────────────────────────────────┐
                   │           Linux Kernel           │
                   │     eBPF verifier + runtime      │
                   └──────────────────────────────────┘
                                      ▲
                                      │
                   eBPF bytecode produced by Rust → LLVM → BPF
                                      │
                                      ▲
                   ┌──────────────────────────────────┐
                   │          aya-ebpf/ (no_std)      │
                   │ - eBPF program Rust runtime      │
                   │ - Map definitions for eBPF side  │
                   │ - BPF helper FFI wrappers        │
                   └──────────────────────────────────┘
                                      ▲
                                      │ macros generate
                                      │ entrypoints, sections
                                      ▼
                   ┌─────────────────────────────────┐
                   │       aya-ebpf-macros/          │
                   │ - #[map], #[xdp], #[kprobe], …  │
                   │ - Generates metadata for loader │
                   └─────────────────────────────────┘
                                      ▲
                                      │
                   ┌──────────────────────────────────┐
                   │       aya-log-ebpf-macros/       │
                   │   Logging macros for eBPF code   │
                   └──────────────────────────────────┘
                                      ▲
                                      │ BTF-based bindings
                                      ▼
                   ┌──────────────────────────────────┐
                   │             aya-tool/            │
                   │ - Generates Rust bindings for    │
                   │   kernel structs using BTF       │
                   └──────────────────────────────────┘
```

---

## **2. Repo-Accurate Folder-Level Diagram**

Directly matching the GitHub repo root:

```
aya-rs/aya/                     <-- Userspace loader
aya-rs/aya-obj/                 <-- ELF + BTF parser used by aya/
aya-rs/aya-log/                 <-- Userspace logging (recv logs)
aya-rs/aya-ebpf/                <-- Kernel-side eBPF Rust (no_std)
aya-rs/aya-ebpf-macros/         <-- #[xdp], #[map], #[kprobe] macros
aya-rs/aya-log-ebpf-macros/     <-- Logging macros inside BPF
aya-rs/aya-tool/                <-- BTF-based Rust bindings generator
aya-rs/aya-build/               <-- Build helper (invoked by cargo)
aya-rs/aya-chart/               <-- Benchmarks / perf profiling
aya-rs/aya-examples/            <-- Example projects (xdp, kprobe…)
aya-rs/book/                    <-- The Aya Book documentation
```

---

## **3. Full Data-Flow Diagram for a Real eBPF Program**

Here is the complete step-by-step flow when you build and run an Aya-based eBPF program.

### ### **LEFT: Build-time** → **RIGHT: Runtime**

```
                           BUILD TIME (cargo build --target bpfel-unknown-none)
                           ────────────────────────────────────────────────────

 your-ebpf-program/
    ├─ src/main.rs  -- (no_std, uses aya-ebpf + macros)
    └─ Cargo.toml

         ▲
         │ you write Rust eBPF program using:
         │   - aya-ebpf
         │   - aya-ebpf-macros (#[xdp], #[map])
         │   - aya-log-ebpf-macros
         ▼

  (1) aya-ebpf-macros/ generates section metadata
        - program section: "xdp/…"
        - map sections: ".maps"
        - BTF/relocations metadata

  (2) LLVM compiles Rust → eBPF bytecode
         - via rustc + bpfel target

  (3) Generated .o ELF goes into:
       target/bpfel-unknown-none/release/your_program.o


                           RUNTIME (userspace program)
                           ────────────────────────────────────────────────────

 your-userspace-app/
    ├─ src/main.rs  -- uses aya crate
    └─ Cargo.toml

         ▼ userspace loader (via aya/)
  (4) aya::EbpfLoader reads the eBPF ELF:
         - uses aya-obj/ to parse:
             * BTF types
             * maps
             * program sections
             * relocations

  (5) aya::programs::* loads programs into kernel:
         - calls bpf(BPF_PROG_LOAD)
         - sets up attributes (XDP, kprobe, etc.)
         - verifier runs
         - returns program FD

  (6) aya::maps::* handles the maps:
         - creates maps via bpf(BPF_MAP_CREATE)
         - loads pinned maps (if any)
         - shares FDs with programs

  (7) aya attaches:
         - for XDP:  bpf_set_link_xdp_fd()
         - for kprobe: perf_event_open() + BPF_LINK_CREATE
         - for tracepoints: BPF_RAW_TRACEPOINT_OPEN
         etc.

  (8) Kernel runs eBPF program in response to events:
         - XDP: incoming network packets
         - kprobe: function entry
         - tracepoint: kernel event
         - uprobe: userspace function call

  (9) eBPF code writes logs or map data:
         - logs via aya-log-ebpf-macros → ring buffer
         - maps: HashMap, Array, PerfEventArray, etc.

 (10) Userspace aya-log/ listens for logs
 (11) Userspace aya::maps::<…> reads/writes maps
```

---

## Summary

This complete diagram shows:

### ✔ how **each crate** in the repo fits together

### ✔ how **build-time (eBPF)** and **runtime (userspace)** layers cooperate

### ✔ how data flows across the kernel boundary

### ✔ where **macros**, **maps**, **ELF parsing**, **BTF**, and **syscalls** operate

---
