# bpftool + aya workflow:

Below are practical **end-to-end workflows** showing how `bpftool` and **Aya** are used *together* in real
development. 

These are the workflows many eBPF developers use when debugging, inspecting maps, validating BPF output,
and checking verifier behavior.

---

##  **Workflow 1 — Verify what Aya loaded into the kernel**

Your Aya user program loads an eBPF program:

```rust
let mut bpf = aya::Bpf::load_file("target/bpf/program.o")?;
let prog: &mut Xdp = bpf.program_mut("my_xdp").unwrap().try_into()?;
prog.load()?;
prog.attach("eth0", XdpFlags::default())?;
```

Now confirm it’s really loaded using `bpftool`:

### **List all eBPF programs**

```bash
sudo bpftool prog list
```

Look for something like:

```
ID 87  Type xdp  Name my_xdp  Tag abc123...
```

### **See program details**

```bash
sudo bpftool prog show id 87
```

### **Dump translated (JITed) bytecode**

```bash
sudo bpftool prog dump xlated id 87
```

### Useful for:

✔ Validating the program is loaded
✔ Inspecting JIT output
✔ Checking instruction count (verifier limits)

---

## **Workflow 2 — Inspect maps your Aya program created**

Aya example:

```rust
let mut bpf = Bpf::load_file("program.o")?;
let map = HashMap::<u32, u64>::try_from(bpf.map_mut("packet_counter")?)?;
```

### **Use bpftool to dump the map**

```bash
sudo bpftool map list
```

Find the map ID:

```
ID 12  Type hash  Name packet_counter
```

### **Dump its contents**

```bash
sudo bpftool map dump id 12
```

Output example:

```
key: 00000001  value: 00000012
key: 00000002  value: 00000005
```

### Useful for:

✔ Seeing what your BPF program is writing
✔ Checking counters/timestamps
✔ Debugging logic without modifying code

---

## **Workflow 3 — View verifier logs for failed Aya loads**

When Aya fails:

```
Error: load bpf program failed: VerifierError
```

You can ask Aya to print logs:

```rust
prog.load(&LoadOptions::default().verifier_log_level(2))?;
```

But `bpftool` gives full logs:

### **Try loading manually**

```bash
sudo bpftool prog load program.o /sys/fs/bpf/tmp verbose
```

This prints huge verifier details such as:

```
invalid access to map value
R1 type=map_value expected=ptr
```

### Useful for:

✔ Understanding why the kernel rejected your program
✔ Finding incorrect pointer arithmetic
✔ Spotting stack overflows or out-of-bounds issues

---

## **Workflow 4 — Verify pinning and attachment**

Aya pins programs/maps to bpffs:

```rust
prog.pin("/sys/fs/bpf/my_xdp")?;
```

Check pinning with:

```bash
sudo bpftool prog show pinned /sys/fs/bpf/my_xdp
```

List pinned maps:

```bash
sudo bpftool map show
```

### Useful for:

✔ Confirming Aya pinned objects correctly
✔ Ensuring programs stay alive after the loader exits

---

## **Workflow 5 — Inspect data written to ring/perf buffers**

Aya example:

```rust
let mut perf = PerfEventArray::try_from(bpf.map_mut("events")?)?;
```

Use `bpftool` to confirm the perf buffer map exists:

```bash
sudo bpftool map list | grep events
```

Dump metadata:

```bash
sudo bpftool map show id <map id>
```

### Useful for:

✔ Debugging event delivery
✔ Ensuring map size/type is what you expect

---

## **Workflow 6 — Confirm CO-RE/BTF correctness**

Aya relies heavily on BPF CO-RE.

### **Dump CO-RE relocations**

```bash
bpftool prog dump relo id 87
```

### **Dump kernel BTF**

```bash
sudo bpftool btf dump file /sys/kernel/btf/vmlinux
```

### Useful for:

✔ Confirming CO-RE was applied
✔ Diagnosing struct layout mismatches
✔ Avoiding silent failures due to incorrect offsets

---

## **Workflow 7 — Examine map memory layout**

To confirm your Rust struct matches the BTF layout:

```bash
sudo bpftool btf dump file program.o
sudo bpftool btf dump file /sys/kernel/btf/vmlinux
```

Useful for debugging:

* struct padding issues
* CO-RE rewrite problems
* incorrect `#[repr(C)]` usage

---

##  **Summary: Aya + bpftool together**

| Task                         | Aya               | bpftool  |
| ---------------------------- | ----------------- | -------- |
| Load program                 | Yes               | Yes      |
| Attach XDP/TC/kprobe/etc     | Yes               | No       |
| Debug verifier failures      | Partial           | **Full** |
| Inspect maps                 | Partial (runtime) | **Full** |
| View BTF, CO-RE, relocations | No                | **Yes**  |
| Dump bytecode/JIT            | No                | **Yes**  |
| Pin objects                  | Yes               | Yes      |
| Inspect pinned objects       | No                | **Yes**  |

`bpftool` is the **de-facto debugging companion** to Aya.

---

#  **Aya + bpftool Cheat Sheet**

## - aya+bpftool

### **1. List loaded eBPF programs**

```bash
sudo bpftool prog list
```

### **2. Inspect a program by ID**

```bash
sudo bpftool prog show id <prog_id>
```

### **3. Dump the translated (JITed) bytecode**

```bash
sudo bpftool prog dump xlated id <prog_id>
```

### **4. List all maps**

```bash
sudo bpftool map list
```

### **5. Dump a map’s contents**

```bash
sudo bpftool map dump id <map_id>
```

### **6. Update a map manually**

```bash
sudo bpftool map update id <map_id> key <key_hex> value <value_hex>
```

### **7. Pin a program or map**

```bash
sudo bpftool prog pin id <prog_id> /sys/fs/bpf/my_prog
sudo bpftool map pin id <map_id> /sys/fs/bpf/my_map
```

### **8. Show pinned objects**

```bash
sudo bpftool prog show pinned /sys/fs/bpf/my_prog
sudo bpftool map show
```

### **9. Dump BTF for CO-RE**

```bash
sudo bpftool btf dump file /sys/kernel/btf/vmlinux
bpftool prog dump relo id <prog_id>
```

### **10. Verbose program load (for verifier debugging)**

```bash
sudo bpftool prog load program.o /sys/fs/bpf/tmp verbose
```

---

## **Example end-to-end workflow**

Suppose you have a Rust Aya XDP program that counts packets per interface.

### 1️⃣ Build BPF program

```bash
cargo xtask build-bpf
# or directly
cargo build --target bpfel-unknown-none --release
```

### 2️⃣ Load program in Aya

```rust
let mut bpf = aya::Bpf::load_file("target/bpf/program.o")?;
let prog: &mut Xdp = bpf.program_mut("count_packets").unwrap().try_into()?;
prog.load()?;
prog.attach("eth0", XdpFlags::default())?;
```

### 3️⃣ Verify program loaded

```bash
sudo bpftool prog list
sudo bpftool prog show id <prog_id>
```

### 4️⃣ Inspect map contents

```bash
sudo bpftool map list
sudo bpftool map dump id <map_id>
```

### 5️⃣ If verifier rejects program

```bash
sudo bpftool prog load target/bpf/program.o /sys/fs/bpf/tmp verbose
```

* Analyze logs for instruction count, stack usage, or invalid accesses.

### 6️⃣ Pin maps/programs

```bash
sudo bpftool map pin id <map_id> /sys/fs/bpf/packet_counter
sudo bpftool prog pin id <prog_id> /sys/fs/bpf/count_packets
```

### 7️⃣ Inspect CO-RE and BTF

```bash
bpftool btf dump file target/bpf/program.o
bpftool prog dump relo id <prog_id>
```

---

### 📌 **Tips**

* Use `bpftool map dump` frequently to **see live stats** written by your BPF program.
* Always check `prog load verbose` if Aya fails to load; it shows **exact kernel verifier errors**.
* Pin objects if you want them to **persist beyond the life of your process**.
* `bpftool` complements Aya — Aya handles loading/attaching, `bpftool` handles **inspection and debugging**.

---

##  **End-to-End Debugging Example**

**end-to-end debugging ex** combining Aya + bpftool using a simple XDP packet counter program.


Suppose we have an Aya XDP program that counts packets per interface.

---

### **1️⃣ BPF Program (Rust)**

**`bpf/src/main.rs`**

```rust
#![no_std]
#![no_main]

use aya_bpf::{
    macros::map,
    maps::HashMap,
    programs::XdpContext,
    macros::xdp,
};
use core::mem;

#[map(name = "PACKET_COUNTER")]
static mut PACKET_COUNTER: HashMap<u32, u64> = HashMap::<u32, u64>::with_max_entries(64, 0);

#[xdp(name = "count_packets")]
pub fn count_packets(ctx: XdpContext) -> u32 {
    match unsafe { try_count(ctx) } {
        Ok(ret) => ret,
        Err(_) => 0,
    }
}

unsafe fn try_count(_ctx: XdpContext) -> Result<u32, ()> {
    let key = 0u32;
    let counter = PACKET_COUNTER.get(&key).unwrap_or(&0);
    let new_counter = *counter + 1;
    PACKET_COUNTER.insert(&key, &new_counter, 0)?;
    Ok(aya_bpf::programs::XDP_PASS)
}
```

> Simple: increments a counter in a BPF map for every packet received.

---

### **2️⃣ User-space Loader (Rust / Aya)**

**`src/main.rs`**

```rust
use aya::{Bpf, programs::Xdp, util::online_cpus};
use aya::maps::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut bpf = Bpf::load_file("target/bpf/debug/bpf_program.o")?;
    let prog: &mut Xdp = bpf.program_mut("count_packets")?.try_into()?;
    prog.load()?;
    prog.attach("eth0", aya::programs::XdpFlags::default())?;

    // Access map from user-space
    let mut map: HashMap<u32, u64> = HashMap::try_from(bpf.map_mut("PACKET_COUNTER")?)?;
    let key = 0u32;
    let value = map.get(&key, 0)?;
    println!("Packets counted: {}", value);

    Ok(())
}
```

---

### **3️⃣ Build BPF Program**

```bash
cargo build --target bpfel-unknown-none --release
```

> Produces `target/bpfel-unknown-none/release/bpf_program.o`.

---

### **4️⃣ Load Program via Aya**

Run your user-space loader:

```bash
sudo cargo run
```

* If everything works, packets are counted and printed.

---

### **5️⃣ Inspect with `bpftool`**

### **List programs**

```bash
sudo bpftool prog list
```

Expected output:

```
ID 85  Type xdp  Name count_packets  Tag abc123...
```

### **Dump instructions**

```bash
sudo bpftool prog dump xlated id 85
```

### **List maps**

```bash
sudo bpftool map list
```

```
ID 12  Type hash  Name PACKET_COUNTER
```

### **Dump map contents**

```bash
sudo bpftool map dump id 12
```

Output example:

```
key: 00000000  value: 00000005
```

---

### **6️⃣ Debug Verifier Failures**

If Aya fails to load:

```bash
sudo bpftool prog load target/bpf/debug/bpf_program.o /sys/fs/bpf/tmp verbose
```

You may see messages like:

```
R1 type=map_value expected=ptr
invalid access to map value
```

* Indicates issues in pointer usage or stack overflows.

---

### **7️⃣ Pin maps/programs for persistence**

```bash
sudo bpftool prog pin id 85 /sys/fs/bpf/count_packets
sudo bpftool map pin id 12 /sys/fs/bpf/packet_counter
```

* Objects persist after user-space program exits.

---

### **8️⃣ Inspect CO-RE/BTF**

```bash
bpftool btf dump file target/bpf/debug/bpf_program.o
bpftool prog dump relo id 85
```

* Confirms struct offsets are correct and CO-RE applied properly.

---

###  **Summary**

1. Build BPF program (`cargo build --target bpfel-unknown-none`)
2. Load/attach via Aya
3. Inspect maps & programs with `bpftool`
4. Debug verifier errors using `bpftool verbose`
5. Pin objects for persistence
6. Inspect CO-RE/BTF if needed

> This workflow lets you **step through loading, runtime stats, debugging, and verification** with Aya + bpftool.

---


## Makefile / xtask workflow that integrates Aya and bpftool:

**Makefile / xtask workflow** that integrates Aya and `bpftool` for building, loading, debugging, and 
inspecting eBPF programs. 

This will streamline the typical Aya + bpftool workflow.

---

### **Makefile for Aya + bpftool**

```makefile
# Makefile

BPF_TARGET = bpfel-unknown-none
BPF_OUT = target/$(BPF_TARGET)/debug
BPF_PROG = $(BPF_OUT)/bpf_program.o

# Adjust your network interface
INTERFACE = eth0

# Default: build both BPF and user-space loader
all: bpf user

# -------------------------------
# Build BPF program
bpf:
	cargo build --target $(BPF_TARGET) --release

# Build user-space loader
user:
	cargo build --release

# -------------------------------
# Load BPF program using Aya
# Assumes your loader binary is `target/release/loader`
load:
	sudo target/release/loader

# -------------------------------
# Inspect loaded programs
list-progs:
	sudo bpftool prog list

dump-prog:
	sudo bpftool prog dump xlated id $(PROG_ID)

# -------------------------------
# Inspect maps
list-maps:
	sudo bpftool map list

dump-map:
	sudo bpftool map dump id $(MAP_ID)

# -------------------------------
# Pin programs/maps
pin-prog:
	sudo bpftool prog pin id $(PROG_ID) /sys/fs/bpf/$(PROG_NAME)

pin-map:
	sudo bpftool map pin id $(MAP_ID) /sys/fs/bpf/$(MAP_NAME)

# -------------------------------
# Verbose load (for debugging verifier errors)
verbose-load:
	sudo bpftool prog load $(BPF_PROG) /sys/fs/bpf/tmp verbose

# -------------------------------
# Clean build artifacts
clean:
	cargo clean
```

---

#### **Usage Examples**

### - Build everything

```bash
make
```

### - Build only BPF program

```bash
make bpf
```

### - Run user-space loader (loads/attaches program)

```bash
make load
```

### - List loaded programs

```bash
make list-progs
```

### - Dump a map

```bash
make dump-map MAP_ID=12
```

### - Pin program for persistence

```bash
make pin-prog PROG_ID=85 PROG_NAME=count_packets
```

### - Verbose load for verifier debugging

```bash
make verbose-load
```

---

### - **Optional: xtask workflow (Rust)**

Aya projects often use **xtask** for custom build tasks. Example structure:

```
xtask/src/main.rs
Cargo.toml (workspace with xtask)
```

```rust
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example: build BPF, load program, print bpftool info
    std::process::Command::new("cargo")
        .args(&["build", "--target", "bpfel-unknown-none", "--release"])
        .status()?;

    // Run user-space loader
    std::process::Command::new("sudo")
        .args(&["target/release/loader"])
        .status()?;

    // List programs
    std::process::Command::new("sudo")
        .args(&["bpftool", "prog", "list"])
        .status()?;

    Ok(())
}
```

* Advantage: everything in **Rust**, cross-platform, and easily extended for additional tasks (pinning, 
  map dumps, verbose load).
* Can integrate with CI pipelines or automated debugging scripts.

---

This setup gives a **fully integrated build + load + inspect + debug workflow** for Aya + bpftool, making
repeated development cycles much faster.

---

## workflow diagram (summarizes the full Aya + bpftool eBPF development pipeline) 

---

#### - **Aya + bpftool Workflow Diagram**

```
  ┌───────────────┐
  │ Rust Source   │
  │ (.rs files)   │
  └───────┬───────┘
          │
          ▼
  ┌───────────────┐
  │ rustc / Aya   │
  │ compiler      │
  └───────┬───────┘
          │
          ▼
  ┌───────────────┐
  │ MIR → LLVM IR │  <- Intermediate Representation
  └───────┬───────┘
          │
          ▼
  ┌───────────────┐
  │ LLVM Backend  │  <- Target: BPF bytecode
  └───────┬───────┘
          │
          ▼
  ┌───────────────┐
  │ bpf-linker    │  <- Links ELF sections & BTF
  └───────┬───────┘
          │
          ▼
  ┌───────────────┐
  │ BPF ELF Object│
  │ (.o file)     │
  └───────┬───────┘
          │
          ▼
  ┌───────────────┐
  │ Aya Loader    │
  │ (User-space)  │
  └───────┬───────┘
          │
          ▼
  ┌───────────────┐
  │ Linux Kernel  │
  │ eBPF verifier │
  └───────┬───────┘
          │
          ▼
  ┌───────────────┐
  │ Running BPF   │
  │ Programs      │
  └───────┬───────┘
          │
          ▼
  ┌───────────────┐
  │ Maps / Events │
  │ Counters      │
  └───────┬───────┘
          │
          ▼
  ┌───────────────┐
  │ bpftool       │
  │ Inspect/Debug │
  └───────────────┘
```

---

### **Step-by-Step Explanation**

1. **Rust Source → rustc / Aya**

   * BPF programs written in Rust (no_std, Aya macros).
   * Compiled to **MIR**, then **LLVM IR**.

2. **LLVM IR → LLVM Backend → bpf-linker**

   * LLVM generates **BPF instructions**.
   * `bpf-linker` creates a verifier-friendly ELF object with proper **sections, alignment, and BTF**.

3. **BPF ELF → Aya Loader → Linux Kernel**

   * Aya user-space loader attaches programs (XDP, kprobe, etc.) and maps.
   * Kernel verifier checks program safety and loads it.

4. **Running BPF Programs → Maps / Events**

   * Program executes safely in kernel context.
   * Updates **maps**, emits **perf/ring events**.

5. **bpftool Inspect / Debug**

   * Query kernel: list programs, dump maps, check pinned objects, view BTF/CO-RE info, read verifier logs.

> This completes the cycle: write → compile → load → execute → inspect/debug.

---

This diagram shows **where Aya and bpftool fit in**:

* Aya: **compilation + loader**
* bpftool: **inspection + debugging**
* Kernel: **execution + verification**

---

