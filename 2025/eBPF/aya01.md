# Aya 

## Aya's Architectural Distinction

Aya is fundamentally different from traditional eBPF tooling because of its pure **Rust** implementation 
and minimal dependencies, leading to better operability and developer experience.

* **No `libbpf` or `bcc` Reliance:** 
    Aya does not depend on the established C libraries like `libbpf` (the official in-kernel library) or 
    the Python-based `bcc` framework. This eliminates the need for C toolchains, kernel headers, and complex 
    build environments, making it faster and easier to deploy.
    
* **Pure Rust Implementation:** 
    The library is built entirely in Rust, which is used for both the **user-space application** and the 
    **eBPF program** itself (via the `aya-ebpf` crate). This allows for easier code sharing and a unified 
    development experience.

* **Direct Kernel Interaction:** 
    Instead of relying on an intermediary C library, the user-space portion of Aya directly interacts with 
    the Linux kernel's eBPF subsystem. This is achieved using the **`libc` crate** to execute the raw 
    **`bpf()` system call** for all essential operations, including:

    * Loading eBPF object files (programs and maps).
    * Creating and managing eBPF maps.
    * Attaching eBPF programs to various hook points (e.g., kprobes, tracepoints, XDP).

This design allows Aya to offer a **"Compile Once, Run Everywhere" (CO-RE)** solution, especially when 
linked with **musl**, as the resulting binary is self-contained with minimal dependencies.

### Comparison of eBPF User-Space Tooling

| Feature | Aya (Rust) | `libbpf` (C) / `libbpf-rs` (Rust bindings) | `bcc` (Python/C++) |
| :--- | :--- | :--- | :--- |
| **Language** | Pure Rust | C / Rust (bindings) | Python (Frontend) / C++ (Backend) |
| **Dependency on `libbpf`** | **No** (Uses `libc` for direct syscalls) | Yes (It *is* `libbpf` or a wrapper) | No (Uses LLVM/Clang for JIT) |
| **Toolchain Requirement** | Only Rust | Requires C toolchain | Requires Python, LLVM/Clang |
| **Deployment** | Single, self-contained binary (especially with musl) | Requires `libbpf` shared library on target | Requires runtime dependencies (Python, etc.) |




# Aya Crate: Roadmap 

Aya is a powerful ecosystem, but it *does* require understanding several moving parts.
Here is a **clean, structured, step-by-step roadmap** to study and master the Aya crates “piece by piece,” 
in the correct order.

This breaks Aya into **digestible components**, so you can learn it without getting overwhelmed.

---

## **Aya Architecture Overview**

Aya consists of several crates:

```
aya
├── aya-bpf          (Rust eBPF program support)
├── aya-bpf-macros   (macros for probes, maps, etc.)
├── aya-bpf-cty      (C type shim; minimal)
├── aya-btf          (BTF parser + CO-RE logic)
└── aya              (userspace loader)
```

To learn Aya properly, you should study them in this order:

1. **aya-bpf** (eBPF program side)
2. **aya-bpf-macros** (how probes and maps work)
3. **aya-btf** (how CO-RE works under the hood)
4. **aya** (userspace loader)

Optional deeper layers:

5. **Aya relocation + verifier interaction**
6. **Aya linker** (`aya-bpf` includes its own linking logic)

---

## PART 1 — Start with `aya-bpf`: The eBPF program side

This crate is the heart of writing eBPF programs in Rust.

### What to study in `aya-bpf`

Study the modules one by one:

### **1. Maps**

```
aya-bpf/src/maps/
```

* `HashMap`
* `Array`
* `PerfEventArray`
* `RingBuf`
* `LruHashMap`
* `SockHash`, `SockMap`
* `Queue`, `Stack`
* `CpuMap`

Understanding maps is **core to eBPF**.

---

### **2. Program types**

```
aya-bpf/src/programs/
```

Learn:

* `XdpContext`
* `SkSkbContext`
* `TracePointContext`
* `KProbeContext`
* `UProbeContext`
* `LsmContext`
* `TcContext`

Each context gives you access to helpers and packet/kernel data.

---

### **3. Helpers**

```
aya-bpf/src/helpers.rs
```

These are Rust-safe wrappers for BPF helpers (like libbpf’s bpf_helpers.h).

Examples:

* `bpf_probe_read_kernel`
* `bpf_map_update_elem`
* `bpf_get_current_pid_tgid`
* `bpf_ktime_get_ns`
* `bpf_redirect`
* etc.

Understanding these is necessary for almost all eBPF tasks.

---

## PART 2 — Study the macros: `aya-bpf-macros`

These macros generate:

* map definitions → relocatable map specs
* probe definitions → attach points
* program sections (`#[xdp]`, `#[tracepoint]`, etc.)

### Files to study

```
aya-bpf-macros/src/
```

Important macros:

* `#[xdp]`
* `#[tracepoint]`
* `#[kprobe]`
* `#[uprobe]`
* `#[cgroup_skb]`
* `#[map]`

Understanding these shows you how Aya organizes ELF sections and metadata.

---

## PART 3 — Study `aya-btf`: the CO-RE / BTF engine

This crate does the heavy lifting of:

* Reading kernel BTF (from /sys/kernel/btf/vmlinux)
* Matching struct layouts
* Applying CO-RE relocations
* Resolving field offsets
* Resolving type equality (kind-based matching)
* Array & bitfield relocation

### Key directories

```
aya-btf/src/
```

Study:

* `btf.rs` → BTF parsing
* `relocation/` → CO-RE resolve and apply
* `types/` → struct descriptions

This is where Aya differs from libbpf (Aya reimplements all of CO-RE in Rust).

If you're doing embedded or custom BTF processing, **this is gold**.

---

## PART 4 — Study the userspace loader: `aya`

This crate:

* reads compiled eBPF ELF files
* loads them into the kernel
* creates maps
* applies relocations
* pins maps/programs
* attaches programs (XDP, TC, uprobes, etc.)
* handles ringbuf + perf buffer

### Key modules

#### **1. `Bpf`**

```
aya/src/bpf/
```

The core type representing a loaded BPF object.

Key functions:

* `Bpf::load_file()`
* `Bpf::load()`
* `Bpf::program()`
* `Bpf::map()`

---

#### **2. Program loaders**

```
aya/src/programs/
```

Study:

* `KProbe`
* `UProbe`
* `TracePoint`
* `Xdp`
* `Tc`
* `CgroupSkb`
* etc.

Each implements:

* load()
* attach()
* detach()

---

#### **3. CO-RE integration**

Loader calls into `aya-btf` to do relocations.

---

#### **4. syscalls**

Aya wraps all BPF syscalls:

```
aya/src/sys/
```

* `bpf_load_program`
* `bpf_map_create`
* `bpf_obj_pin`
* map FD management

This is where Aya deviates from libbpf and becomes 100% Rust.

---

## PART 5 — Study Aya’s internal ELF + reloc handling

Aya includes its own minimal eBPF ELF linker logic:

Directories:

```
aya-bpf/src/elf/
aya/src/obj/
aya/src/relocation/
```

If you want to understand *how Rust eBPF programs get compiled, assembled, linked*, this part is extremely educational.

---

## **Suggested step-by-step learning path (1-month realistic plan)**

### **Week 1 — Write simple programs**

1. XDP hello-world
2. Kprobe tracing
3. Uprobe tracing
4. Perf buffer & ringbuf
5. Maps: HashMap / Array / LRU

At this stage, you're learning the public API.

---

### **Week 2 — Study the internals**

* Study `aya-bpf` (maps + program contexts)
* Study macros
* Understand how Rust → BPF sections work

---

### **Week 3 — BTF + CO-RE**

* Read through `aya-btf` crate
* Inspect BTF blobs
* Use Aya to relocate struct offsets

---

### **Week 4 — Loading mechanics**

* Read Aya loader (`aya/src/programs/`)
* Study BPF syscalls
* Understand eBPF verifier interaction
* Look at map pinning and lifetime

---

## *Best resources to accompany your study*

### Aya Book (official docs)

[https://aya-rs.dev/book/](https://aya-rs.dev/book/)

### Aya source code (excellent comments)

[https://github.com/aya-rs/aya](https://github.com/aya-rs/aya)

### eBPF docs

[https://docs.kernel.org/bpf/](https://docs.kernel.org/bpf/)

### CO-RE guide (libbpf but still applies)

[https://nakryiko.com/posts/bpf-core-reference/](https://nakryiko.com/posts/bpf-core-reference/)

---

## 🙋 Want something easier?

If you'd like, I can generate:

* A **guided learning roadmap** (daily or weekly)
* A “read these 20 source files in this order” list
* A custom **Aya + embedded Linux study plan**
* Small exercises:

  * write your first Aya XDP
  * write a tracer
  * build a static Aya program for musl
  * inspect kernel BTF via Aya 



# A **practical, structured Aya + embedded Linux study plan**.

This plan assumes you already know Rust and have basic Linux experience.

---

## **Aya + Embedded Linux Study Roadmap**

## **Week 1 — Foundations: Writing eBPF Programs in Rust (aya-bpf)**

**Goal:** Get comfortable writing and running simple Aya eBPF programs.

### **Daily Tasks**

| Day | Focus                                                 | Files / Modules                  | Exercises                                               |
| --- | ----------------------------------------------------- | -------------------------------- | ------------------------------------------------------- |
| 1   | Install Aya, Rust + cross-compile for embedded target | N/A                              | Build a “Hello World” XDP program, print packet info    |
| 2   | Maps: HashMap, Array, LRU                             | `aya-bpf/src/maps/*`             | Write a program storing packet counts per PID           |
| 3   | Program contexts: XDP, KProbe, TracePoint             | `aya-bpf/src/programs/*`         | Trace `open()` syscalls using KProbe                    |
| 4   | Perf events and RingBuf                               | `aya-bpf/src/maps/perf_event.rs` | Push events from kernel to user-space, read them        |
| 5   | Helpers                                               | `aya-bpf/src/helpers.rs`         | Use `bpf_get_current_pid_tgid`, `bpf_probe_read_kernel` |
| 6   | Combine maps + context                                | all above                        | Count open syscalls per PID and report via perf buffer  |
| 7   | Review & debug                                        | N/A                              | Use `bpftool` to inspect loaded programs and maps       |

---

## **Week 2 — Aya Macros and Compilation Mechanics**

**Goal:** Understand how Rust attributes generate ELF sections and map metadata.

### **Daily Tasks**

| Day | Focus                      | Files / Modules                        | Exercises                                                         |
| --- | -------------------------- | -------------------------------------- | ----------------------------------------------------------------- |
| 1   | Macro system               | `aya-bpf-macros/src/lib.rs`            | Read `#[map]` macro and see generated BTF                         |
| 2   | Program attributes         | `#[xdp]`, `#[kprobe]`, `#[tracepoint]` | Write a KProbe + XDP program using macros only                    |
| 3   | How sections are generated | ELF sections                           | Inspect `target/bpfel-unknown-none/debug/*.o` with `llvm-objdump` |
| 4   | Linking & metadata         | `aya-bpf/src/elf/*`                    | See how maps are defined in ELF `.maps` sections                  |
| 5-7 | Mini-project               | Combine macros, maps, and perf buffers | Build XDP + perf buffer tracer reporting TCP packets per PID      |

---

## **Week 3 — BTF and CO-RE (aya-btf)**

**Goal:** Learn how Aya implements CO-RE relocations and parses BTF.

### **Daily Tasks**

| Day | Focus             | Files / Modules             | Exercises                                                                 |
| --- | ----------------- | --------------------------- | ------------------------------------------------------------------------- |
| 1   | BTF basics        | `aya-btf/src/btf.rs`        | Dump `/sys/kernel/btf/vmlinux` using Aya                                  |
| 2   | Types and strings | `aya-btf/src/types.rs`      | Iterate structs, enums, function prototypes                               |
| 3   | Relocation logic  | `aya-btf/src/relocation.rs` | Understand how struct offsets are fixed                                   |
| 4   | CO-RE in practice | All BTF + relocation        | Modify an XDP program for different kernel versions and verify relocation |
| 5   | Field resolution  | `aya-btf/src/resolve.rs`    | Query field offsets, bitfields, array members                             |
| 6   | Debugging         | N/A                         | Use Aya to detect missing kernel BTF, fallback to non-CO-RE               |
| 7   | Mini-project      | N/A                         | Build a portable CO-RE program across 2 kernel versions                   |

---

## **Week 4 — Userspace loader (aya)**

**Goal:** Learn how Aya loads programs, applies relocations, attaches, and interacts with maps.

### **Daily Tasks**

| Day | Focus              | Files / Modules      | Exercises                                                         |
| --- | ------------------ | -------------------- | ----------------------------------------------------------------- |
| 1   | Bpf loader         | `aya/src/bpf.rs`     | Load a compiled ELF program, inspect sections                     |
| 2   | Map management     | `aya/src/maps.rs`    | Lookup, pin, update maps                                          |
| 3   | Program attachment | `aya/src/programs/*` | Attach KProbe, XDP, TracePoint programs                           |
| 4   | CO-RE + loader     | `aya/src/btf.rs`     | Load program with relocation applied                              |
| 5   | Syscalls & FDs     | `aya/src/sys/*.rs`   | Study `bpf_map_create`, `bpf_prog_load`, pinning                  |
| 6   | Debugging          | N/A                  | Use Aya to inspect map contents, perf buffers, and program state  |
| 7   | Mini-project       | N/A                  | Build a full program: XDP + HashMap + CO-RE + user-space reporter |

---

## **Week 5 — Embedded Linux Focus**

**Goal:** Apply Aya on musl / minimal / embedded targets.

### **Tasks**

1. Cross-compile Aya for your embedded target (`bpfel-unknown-none`, `bpfeb-unknown-none`)
2. Ensure `/sys/kernel/btf/vmlinux` is present; if not, explore embedding fallback BTF
3. Test simple XDP or KProbe programs on embedded kernel
4. Pin maps for persistent state across reloads
5. Handle CO-RE errors gracefully (Aya fallback mode)

---

## **Extra Exercises / Advanced Topics**

* Implement a small `bpftool`-like Rust CLI using Aya-btf + aya-bpf to dump BTF types
* Build a tiny CO-RE program loader for embedded devices with no `/sys/kernel/btf/vmlinux`
* Explore Aya map types not usually covered: `Queue`, `Stack`, `SockMap`

---

## **Tips for Studying Aya Source**

1. Clone the repository:

```bash
git clone https://github.com/aya-rs/aya
cd aya
```

2. Start with `examples/` — Aya ships with working examples for XDP, TracePoints, KProbes, UProbes.

3. Use `cargo doc --open` for each crate: `aya-bpf`, `aya-btf`, `aya`.

4. Step through macros with `cargo expand` to see generated code.

5. Use `bpftool` and Aya debug logging to inspect maps and programs while learning.
