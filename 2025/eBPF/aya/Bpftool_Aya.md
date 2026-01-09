# `bpftool` vs `aya-tool`

While `aya-tool` and `bpftool` both interact with BPF Type Format (BTF) data, they serve different roles in 
the eBPF ecosystem. 

`aya-tool` is specifically a **bridge** between the kernel's BTF data and the Rust lang, whereas `bpftool`
is a **comprehensive management tool** for the kernel's BPF subsystem.

### Comparison Table: `aya-tool` vs. `bpftool`

| Feature / Operation | `aya-tool` | `bpftool` |
| --- | --- | --- |
| **Primary Language** | Rust | C |
| **Generate C Headers** | ❌ No | ✅ Yes (`format c`) |
| **Generate Rust Bindings** | ✅ Yes (`generate`) | ❌ No (Requires `bindgen`) |
| **Target Specific Structs** | ✅ Yes (by name) | ✅ Yes (by ID or full dump) |
| **List Running Programs** | ❌ No | ✅ Yes (`prog list`) |
| **Inspect BPF Maps** | ❌ No | ✅ Yes (`map dump`) |
| **Pin/Load Programs** | ❌ No | ✅ Yes (`prog load`) |
| **Visual/DOT Output** | ❌ No | ✅ Yes (`visual`) |
| **JSON Support** | ❌ No | ✅ Yes (`--json`) |
| **CO-RE Compatibility** | ✅ Yes (via `aya`) | ✅ Yes (via `libbpf`) |

---

### What `aya-tool` CAN do

* **Rust Type Generation:** 
    It automates the complex process of running `bpftool` to get C code and then running `bindgen` to get 
    Rust code. It wraps these into one command.

NOTE: `bindgen` : 
    Generates Rust bindings from C/C++ headers. 

* **Direct Naming:** 
    You can ask for `task_struct` by name directly, and it resolves the dependencies for you.

* **Build Integration:** 
    It is often used inside a project's `xtask` or build script to regenerate bindings whenever the kernel 
    environment changes.

### What `aya-tool` CANNOT do

* **System Management:** 
    It cannot see which BPF programs are currently attached to your network interfaces or tracepoints.

* **Data Inspection:**
    It cannot "peek" into a BPF map to see what data is currently stored there while the program is running.

* **Native Loading:** 
    It does not load `.o` files into the kernel; that is the job of the `aya` library within your Rust 
    application code.

* **Debugging:** 
    It cannot disassemble `JIT`-compiled code to see the actual machine instructions running in your CPU.

### Summary of the Workflow

Typically, you use **`aya-tool`** once during development to create your `bindings.rs` file. 
You use **`bpftool`** constantly during development and production to debug, verify, and manage the programs 
you've built.

---------------------------------------------------------------------------------------

# `bpftool` and how to use along side aya development:

`bpftool` is essential for Aya development, since Aya does not provide a built-in CLI for inspecting the
kernel state. Instead it relies on using standard kernel tools.

## `bpftool` Short intro:

A powerful CLI utility for interacting with Linux BPF sybsystem. It facilitates a simple convenient
interface for `querying`, `manipulating`, and `debugging` BPF programs and Maps. 

`eBPF` programs can perform tasks like:
- Packet Filtering ( network observability )
- Tracing ( Ex: bpftrace and perf rely on BPF )
- Security ( Ex: XDP for denial of service protection )
- Performance analysis ( Ex: bpf-related progs for profiling )

All of these have generally two components:
- **BPF program** (bytecode that gets compiled & executed inside the kernel) which get triggered by events
  such as packet arrival, system call , or tracepoint.
- **BPF Maps**: Data structures that allow BPF kernel program to store and retrive information efficiently.

## `bpftool`  use cases:

* **Network observability:** 
    Inspect BPF programs used for advanced network tracing (ex: XDP, tc, or BPF-based packet capture tools).

* **Debugging and troubleshooting:**
    Investigate running BPF programs, tracepoints, and kernel events.

* **Program management:** 
    Load, unload, and modify BPF programs dynamically.

* **Map management:** 
    Query and modify BPF maps, which store data for programs.


### Basic Commands in `bpftool`:

- Show BPF Programs : list all loaded BPF programs in the kernel:

```bash
bpftool prog show
```
Displays all active BPF programs, including their IDs, types, and associated file descriptors.

- Show BPF Maps: To list all BPF maps:

```bash
bpftool map show
```
Displays details about all loaded BPF maps, such as map IDs, types, and sizes.

- Show BPF Object Files: view all available BPF object files (such as ELF files) by using:

```bash
bpftool obj show
```

- Show BPF Link: If you're working with BPF links (which are a way to link BPF programs to specific 
  network interfaces or tracepoints), you can use:

```bash
bpftool link show
```

### Inspecting BPF Programs

You can dive deeper into BPF programs and understand their structure using `bpftool` 

- List all programs/maps: 

```bash 
bpftool prog list ( or show)

bpf maps list 
```
First command shows: ID, Type, Name, and loaded time.
Second shows all maps that are created. 

- Dump BPF Program Details

    Get detailed info about a specific BPF program, use the program ID. For example, to inspect program 1:

```bash
bpftool prog dump xlated id 1
```

The command will display the translated BPF bytecode for the specified program.

- Dump BPF Program Type : To get the program type (e.g., XDP, tc, tracepoint), use the `type` flag:

```bash
bpftool prog show type xdp
```

This command filters the results and only shows BPF programs of type XDP.

- Check Links: To show active "attachments" ( eg: an XDP prog attached to eth0 ).

```bash 
bpftool link list
```

### Working with BPF Maps

BPF maps store data that is shared between user space and BPF programs. 
These maps can be queried and modified using `bpftool`.

- Display Map Contents:

You can display the contents of a BPF map using its ID. For example:

```bash
bpftool map dump id 123
```

This will display the key-value pairs stored in map `123`.

- Modify Map Entries: To insert, update, or remove entries in a BPF map, use the `bpftool map` commands 
  with the `update`, `delete`, or `lookup` subcommands. For example, to update an entry:

```bash
bpftool map update id 123 key 1 value 100
```

This updates the value associated with key `1` in map `123` to `100`.


### Troubleshooting with BPFTool

If you're encountering issues with BPF programs, `bpftool` can help you troubleshoot. Here are some 
troubleshooting steps:

- Trace a Program: 
    Use the `bpftool` `trace` command to trace program execution and diagnose issues:

```bash
bpftool prog trace id 1
```

This command traces the execution of the BPF program with ID 1.

- Get System Logs 
  Sometimes, system logs can provide insight into issues with BPF programs. 
  You can check the kernel log buffer for relevant messages:

```bash
dmesg | grep bpf
```

This will show any relevant BPF-related log messages.

### Advanced Features:

`bpftool` also includes advanced features for users who want to interact with BPF more deeply.

- Loading BPF Programs from ELF Files

You can load BPF programs from an ELF file, which is often used for more complex setups involving multiple 
programs.

```bash
bpftool prog load <path_to_elf> <map_paths>
```

- Attaching Programs to Interfaces (eBPF/XDP)
  For network-related BPF programs, such as XDP (Express Data Path), you can attach a prog to a network 
  interface.

```bash
bpftool net attach xdp id 1 dev eth0
```

This attaches the BPF program with ID 1 to the `eth0` network interface using XDP.

- Profiling with BPF

BPF allows for performance profiling using the `perf` tool, and `bpftool` can be used to interact with it.

```bash
bpftool perf trace
```

This command allows you to capture and display trace data from BPF-based performance profiling.

- Dumping : checking under the hood:
    * Dump Bytecode: `bpftool prog dump xlated id <ID` 
    This helps to check how the Rust code is actually translated into eBPF instructions.

    * Dump JIT compiled code: `bpftool prog dump jited id <ID>`
    See the actual x86_64/ARM machine code the kernel is executing. 

    * Read Map data: `bpdtool map dump id <ID>`
    If Rust prog is supposed to count packets, run this to see the live values in the map. 

- Feature probing: ( Check if the kernel is ready ):
    Useful when moving binary to an older kernel :
    * Probe features: `bpftool feature probe` <== Useful 
    List every eBPF helper/ map types supported by the current kernel.

### Common problem solving: ( Zombie eBPF programs):

While developing eBPF programs: If userspace Rust program crashes, the eBPF program might stay "pinned" or
attached to the kernel, and they can be removed or killed using `bpftool`

```bash 
# find the link ID 
bpftool link list 

# Detach it 
bpftool link detack id <ID>
```


## Using `bpftool` with Aya workflow :

In the Aya workflow, you usually write your code, run `cargo xtask run`, and... **nothing happens.** 
Instead, you get a wall of text from the kernel saying "Permission Denied" or "Invalid Argument." 

This is the **eBPF Verifier** rejecting your program.

Aya tries to print this log for you, it can sometimes be truncated or hard to read in a busy terminal. 
This is where `bpftool` becomes your best friend.

Scenario 1:  The "Null Dereference" Trap

Imagine you wrote an Aya program that looks up a value in a `HashMap`. 
In Rust, you might instinctively try to use the value immediately.

```rust
// ❌ Dangerous Aya Code
let val = MY_MAP.get(&key); 
let data = unsafe { *val }; // The Verifier hates this!

```

When you run this, Aya will fail to load the program. 
Here is how to use `bpftool` to find out exactly why.

### Step 1: Manual Load Attempt with Debug Logs

If your Aya application's output is messy, use `bpftool` to attempt a manual load of your compiled 
`.pfe` (ELF) file. 
The `-d` (debug) flag is the "magic" switch that forces the kernel to dump the **entire** verifier log.

```bash
# -d enables libbpf and verifier debug logs
sudo bpftool -d prog load target/bpfel-unknown-none/debug/myapp /sys/fs/bpf/myapp

```

### Step 2: Reading the Verifier Log

The output will look like a trace of a CPU execution. 
You are looking for the very end of the log where the "Instruction" and "Error" meet.

**The error might look like this:**

```text
10: (85) call bpf_map_lookup_elem#1
11: (71) r1 = *(u8 *)(r0 +0)
R0 invalid mem access 'map_value_or_null'

```

**How to interpret this `bpftool` output:**

1. **Instruction 10:** The kernel called the map lookup.
2. **Register State:** It notes that `R0` (the return value) is `map_value_or_null`.
3. **Instruction 11:** You tried to read from `R0` (dereference it).
4. **The Verdict:** The verifier stops you because if the key wasn't in the map, `R0` is NULL, and 
   dereferencing NULL would crash the kernel.

### Step 3: Correlating to Rust Line Numbers

`bpftool` can show you exactly which line of Rust code produced that failing instruction if you compiled 
with debug info.

```bash
# Dump the "xlated" (post-verifier) code with line numbers
sudo bpftool prog dump xlated name my_program linum

```

This will print the eBPF instructions with comments like `; /src/main.rs:42`, pointing you directly to the 
offending line in your Rust project.

---

### Comparison of Debugging Options

| Method | Best For... | Command |
| --- | --- | --- |
| **Aya Logs** | Quick checks during `cargo run`. | (Built-in to `aya-log`) |
| **bpftool -d** | When the program won't even load. | `bpftool -d prog load ...` |
| **bpftool xlated** | Seeing how Rust `match` or `if` was optimized. | `bpftool prog dump xlated` |
| **bpftool visual** | Complex logic/loops causing complexity errors. | `bpftool prog dump xlated visual` |

---

### Summary Checklist for Aya Debugging

1. **Compile** your Aya project (`cargo xtask build`).
2. **Locate** the `.pfe` or `.o` file in `target/`.
3. **Use `bpftool -d prog load**` to see the raw verifier log.
4. **Fix the Rust code** (usually by adding a `val.is_null()` check or an `if let Some()`).


## `bpftool` Inspect **Global Variables**:

In BCC you can use macros and string interpolation to pass variables from user-space to BPF:
In Aya we use **Global Variables**, These get compiled into a specific BPF Map called `.data` or `.rodata`.

When Rust code is not behaving as expected `bpftool` is the only way to verify your user-space app actually
wrote the correct value into these variables:

### 1. The Setup: Defining the Variable in Aya

In your eBPF code (the `myapp-ebpf` crate), you define a global variable like this:

```rust
#[no_mangle]
static FILTER_PID: volatile_unnamed::Volatile<u32> = volatile_unnamed::Volatile::new(0);

```

In your userspace code, you set this before loading:

```rust
let mut bpf = Bpf::load(include_bytes_aligned!(path))?;
let mut filter_pid = bpf.global_variable_mut::<u32>("FILTER_PID", true)?;
filter_pid.write(&1234); // Setting the PID to filter

```

---

### 2. The Problem: "Why isn't my filter working?"

If your program is running but not filtering correctly, you need to check what is *actually* inside the 
kernel's memory.

#### **Step A: Find the Global Variable Map**

Global variables are stored in internal maps. Use `bpftool` to find the one associated with your program.

```bash
sudo bpftool map list

```

Look for a map named something like `myapp.rodata` or `myapp.data`.
*Example ID: 42*

#### **Step B: Dump the Data Map**

Now, use the ID to see the hex dump of your variables.

```bash
sudo bpftool map dump id 42

```

**Output:**

```text
[{
        "key": 0,
        "value": {
            ".data": [
                "0xd2", "0x04", "0x00", "0x00"
            ]
        }
    }
]

```

*Note: `0xd2 0x04` in little-endian hex is `1234` in decimal. 
If you see `0x00 0x00`, you know your userspace Rust code failed to write the variable before loading!*

---

### 3. Comparing the Workflow: BCC vs. Aya

| Feature | BCC Style | Aya (Modern eBPF) Style |
| --- | --- | --- |
| **Mechanism** | Textual replacement in C strings. | Memory-mapped Global Variables. |
| **Inspection** | Hard (requires `bpf_trace_printk`). | Easy (use `bpftool map dump`). |
| **Type Safety** | None (String based). | Full (Rust types on both sides). |
| **Performance** | Slower (re-compiles every time). | Faster (compiled once, patched at load). |

---

### 4. Advanced: Modifying Globals Live

One "ninja" trick with `bpftool` is that you can actually change a global variable 
**while the program is running** without restarting your Rust app (as long as it's in `.data` and not `.rodata`).

```bash
# Update the value at key 0 in map ID 42 to PID 5678 (0x162E)
sudo bpftool map update id 42 key 0 0 0 0 value 0x2e 0x16 0x00 0x00

```

This is incredibly powerful for toggling debug logs or changing filter criteria on the fly.

---

### Summary Checklist for `bpftool` + Aya

1. **`bpftool prog list`**: Check if your Aya app successfully attached.
2. **`bpftool map list`**: Find the ID of your `.data` or `.rodata` maps.
3. **`bpftool map dump`**: Verify your configuration variables are correct.
4. **`bpftool btf dump`**: Check the memory alignment if the hex dump looks "shifted."


## High Performance Networking: with Aya and XDP:

When moving into high-performance networking with Aya and XDP, you are effectively shifting from 
"observing" the system to "owning" the data plane. 
At 10Gbps or 100Gbps, every nanosecond counts.

`bpftool` provides the visibility required to ensure your XDP program isn't just "running," but running 
efficiently on the right hardware layer.

### 1. Identifying the Attachment Mode

XDP can run in three modes. Before benchmarking, you must verify where your Aya program is actually hooked.

```bash
# List all network-attached BPF programs
sudo bpftool net show

```

**What to look for in the output:**

* **`xdpgeneric`**: 
    The "slow" path. Packets have already been converted to `skb` (socket buffers) by the kernel.

* **`xdpdrv` (Native)**: 
    The "fast" path. Your Aya code runs directly in the NIC driver before memory allocation.

* **`xdpoffload`**:
    The "lightspeed" path. 
    The program is running on the NIC's own processor (requires SmartNICs like Netronome or BlueField).

---

### 2. Measuring Execution Time (Micro-benchmarking)

By default, the kernel does not track how long each eBPF program takes because the measurement itself adds 
overhead. You must enable it globally first.

**Step A: Enable BPF Stats**

```bash
sudo sysctl -w net.core.bpf_stats_enabled=1

```

**Step B: Check the performance metrics**

```bash
sudo bpftool prog show id <YOUR_PROG_ID>

```

Look for these two fields in the output:

* **`run_cnt`**: 
    How many packets have hit your program.

* **`run_time_ns`**:
    The cumulative time spent in your program.

* **Calculation**: `run_time_ns / run_cnt` = **Average nanoseconds per packet.** 

  **Tip:** In high-performance XDP, if your average is above **20-30ns**, you may start dropping packets on 
  a 10Gbps link.

---

### 3. Debugging Hardware Offload

If you are trying to offload your Aya program to a SmartNIC, `bpftool` is your primary diagnostic tool. 
Hardware offload is extremely strict; if you use a helper function or map type the NIC doesn't support, it 
will fail.

**Check if offload is possible:**

```bash
# Probe the hardware capabilities of a specific interface (e.g., eth0)
sudo bpftool feature probe dev eth0

```

**Verify offloaded instructions:**
If a program is offloaded, `bpftool` can show you the native machine code running on the NIC:

```bash
sudo bpftool prog dump jited id <ID>

```

If this returns **NFP (Netronome)** or **Mellanox** specific assembly instead of x86, you have successfully 
offloaded your logic.

---

### 4. Real-time Monitoring with `bpftool prog profile`

Newer versions of `bpftool` (v5.1+) have a built-in profiler that uses hardware performance counters 
(L1 cache misses, instructions per cycle) to see exactly how your Rust code impacts the CPU.

```bash
# Profile your XDP program for 10 seconds
sudo bpftool prog profile id <ID> duration 10 cycles instructions l1d_misses

```

* **High `l1d_misses`?** Your Aya HashMap might be too large, causing cache thrashing.
* **Low `instructions` per cycle?** You might have too many branches (`if/else`) in your packet parsing.

---

### Summary Checklist for High-Performance XDP

| Goal | `bpftool` Command |
| --- | --- |
| **Verify Mode** | `bpftool net show` (Ensure it says `xdpdrv`) |
| **Check Latency** | `bpftool prog show` (Check `run_time_ns`) |
| **Hardware Check** | `bpftool feature probe dev <ifname>` |
| **CPU Impact** | `bpftool prog profile id <ID>` |

### Next Step

To truly achieve high performance in Rust/Aya, you often need to move away from standard `HashMap` and 
toward **Per-CPU Maps** to avoid CPU lock contention.

## Optimizing High performance data plane in Aya ( `HashMap` and `PerCpuHashMap`):

When optimizing a high-performance data plane in Aya, the choice between a standard `HashMap` and a 
`PerCpuHashMap` is often the difference between a bottleneck and a 100Gbps-ready application.

`bpftool` is the best way to visualize why this performance gap exists by letting you peer into the memory
layout of each.

### 1. The Performance Theory: Lock Contention

In a standard `HashMap`, multiple CPU cores share a single value for a given key. 
To prevent data corruption, you must use **Atomics** or a **Spinlock** (which Aya/eBPF supports since 
Kernel 5.1). 
At high packet rates, CPUs spend more time waiting for the lock than processing packets.

A `PerCpuHashMap` avoids this by giving every CPU core its own private "shard" of the map. 
No locks are needed because CPU 0 only ever touches its own version of the value.

### 2. Comparing Memory Layout with `bpftool`

Let's look at how the data is stored. Suppose your Aya program tracks packet counts using a `u64` value.

#### **Scenario A: Standard HashMap**

In a standard map, one key maps to exactly one value.

```bash
# Dump a standard HashMap (ID 10)
sudo bpftool map dump id 10

```

**Output:**

```json
{
    "key": 1,
    "value": 1500
}

```

*Total memory for this entry: 8 bytes (value size).*

#### **Scenario B: PerCpuHashMap**

In a Per-CPU map, one key maps to an **array of values**, where the index of the array corresponds to the CPU ID.

```bash
# Dump a Per-CPU HashMap (ID 11)
sudo bpftool map dump id 11

```

**Output:**

```json
{
    "key": 1,
    "values": [
        {"cpu": 0, "value": 400},
        {"cpu": 1, "value": 600},
        {"cpu": 2, "value": 200},
        {"cpu": 3, "value": 300}
    ]
}

```

*Total memory for this entry: 32 bytes (8 bytes × 4 CPUs).*

---

### 3. Using `bpftool` to Spot "Memory Bloat"

While `PerCpuHashMap` is faster, it is much hungrier for memory. 
You can use `bpftool` to see the total memory impact:

```bash
sudo bpftool map show id 11

```

Look for the **`memlock`** field.

* **HashMap:** 
    `memlock 167936B`

* **PerCpuHashMap:** 
    `memlock 1343488B` (on a 32-core system, this is ~8x larger for the same number of entries).

---

### 4. Comparison Summary

| Feature | Standard `HashMap` | `PerCpuHashMap` |
| --- | --- | --- |
| **Aya Type** | `HashMap` | `PerCpuHashMap` |
| **Locking** | Needs Spinlocks/Atomics | **Lock-free** (Natural) |
| **Performance** | Slower (High contention) | **Maximum** (Linear scaling) |
| **Memory** | Efficient | Expensive (Val × Num_CPUs) |
| **Userspace** | Single lookup | Must aggregate (sum) all CPU values |

---

### 5. Pro-Tip: Aggregating in Rust

When using `PerCpuHashMap`, your userspace Aya code won't get a single number. 
It gets a `PerCpuValues<T>`. You have to sum them up to get the "total" count:

```rust
let val = map.get(&key, 0)?;
let total: u64 = val.iter().sum(); // Summing values from all CPU cores

```

NOTE: 
BPF Ring Buffers are the highest-performance way to send metadata (like logs or sampled packets) from your 
Aya XDP program back to your userspace Rust app.

## BPF Ring Buffers

They are the highest-performance way to send metadata (like logs or sampled packets) from your Aya XDP 
program back to your user-space Rust app.

Before BPF Ring Buffers (introduced in Kernel 5.8), **Perf Buffers** (`PerfEventArray`) were used. 

While common, Perf Buffers had a major flaw for high-performance apps: they are **per-CPU**. 
If CPU 0's buffer was full, it would drop packets even if CPU 1's buffer was empty.

**BPF Ring Buffers** solve this by providing a single, shared, multi-producer memory area.

---

### 1. The Architectural Shift: Shared Memory

| Feature | Perf Buffer (Old) | Ring Buffer (New) |
| --- | --- | --- |
| **Structure** | Multiple buffers (one per CPU) | **Single shared buffer** |
| **Memory Efficiency** | Poor (Often over-allocated) | **High** (Dynamic sharing) |
| **Event Ordering** | Not guaranteed across CPUs | **Guaranteed** globally |
| **API Style** | Copy-based | **Zero-copy** (Reserve/Submit) |

---

### 2. Using `bpftool` to Monitor Ring Buffers

Because a Ring Buffer is just another map type (`BPF_MAP_TYPE_RINGBUF`), you can inspect it with standard 
`bpftool` commands.

#### **Check Capacity & Usage**

```bash
sudo bpftool map show name MY_RINGBUF

```

* **`max_entries`**: 
    For Ring Buffers, this isn't the "number of items," but the **total size in bytes** (must be power of 2).

* **`frozen`**: 
    Shows if the map has been made read-only.

#### **The "No-Copy" Verification**

In your Aya code, you use `RING_BUF.reserve()`. 
This gives you a pointer directly to the kernel's memory. 
To verify that your data is correctly formatted *before* your Rust app reads it, you can peek at the raw hex:

```bash
sudo bpftool map dump name MY_RINGBUF

```

---

### 3. High-Performance Optimization: Wakeup Strategies

In XDP, sending a "signal" to userspace for every single packet is expensive. 
`bpftool` can help you verify if your **wakeup strategy** is working.

In Aya, you can use flags when submitting:

* **`BPF_RB_NO_WAKEUP`**: 
    Put data in the buffer but don't tell userspace yet.

* **`BPF_RB_FORCE_WAKEUP`**: 
    Force an immediate interrupt/notification.

**How to verify with `bpftool`:**
    Run `bpftool prog show`. 
    If your `run_time_ns` is spiking but your userspace app is lagging, you likely have a 
    "Thunderous Herd" problem where too many wakeups are killing your CPU performance.

---

### 4. Implementation Snippet (Aya)

**Kernel Side (`myapp-ebpf`):**

```rust
#[map]
static EVENTS: RingBuf = RingBuf::with_byte_size(1 << 20, 0); // 1MB buffer

// Inside your XDP/Kprobe function:
if let Some(mut entry) = EVENTS.reserve::<MyEvent>(0) {
    let e = unsafe { &mut *entry.as_mut_ptr() };
    e.pid = bpf_get_current_pid_tgid() as u32;
    entry.submit(BPF_RB_NO_WAKEUP); // Don't wake userspace every time
}

```

**Userspace Side (`myapp`):**

```rust
let mut ring_buf = RingBuf::try_from(bpf.map_mut("EVENTS").unwrap())?;
loop {
    if let Some(event) = ring_buf.next() {
        // Process event...
    }
}

```

---

### Summary Checklist

1. **Is the Kernel 5.8+?** Ring Buffers won't work on older kernels.
2. **Is the size a power of 2?** `1024`, `4096`, etc.
3. **Check `bpftool map show**`: Is the memory allocation (`memlock`) what you expected?


##  "Cheat Sheet" of all these bpftool commands into one categorized reference:

Useful for inspection of programs, maps, performance, and ring buffers, 
Helps in bridging gap between Rust development and kernel's execution state:

### The Aya Developer's `bpftool` Cheat Sheet:

#### 1. Program Management (Discovery & Cleanup)

| Command | Purpose |
| --- | --- |
| `sudo bpftool prog list` | List all loaded programs (find your Aya IDs/Names). |
| `sudo bpftool prog show id <ID>` | Show detailed info: memory use, attached maps, and `btf_id`. |
| `sudo bpftool net show` | See all network attachments (XDP/TC) and their modes. |
| `sudo bpftool link detach id <ID>` | **Emergency:** Force detach a program if your Rust app crashed. |

#### 2. Debugging & The Verifier

| Command | Purpose |
| --- | --- |
| `sudo bpftool -d prog load <ELF> <PATH>` | **The Verifier Debugger:** Load your `.pfe` file with full logs. |
| `sudo bpftool prog dump xlated id <ID> linum` | See BPF instructions mapped to your Rust source lines. |
| `sudo bpftool prog dump xlated id <ID> visual > v.dot` | Generate a flowchart of your program logic. |
| `sudo bpftool prog tracelog` | View the kernel trace pipe (where `aya_log` or `bpf_printk` goes). |

#### 3. Map Inspection (The "State" of your App)

| Command | Purpose |
| --- | --- |
| `sudo bpftool map list` | List all maps. Look for `.data` and `.rodata` for global variables. |
| `sudo bpftool map dump id <ID>` | Dump all keys and values (great for verifying `HashMap` counts). |
| `sudo bpftool map lookup id <ID> key hex <HEX>` | Look up a specific entry using hex-encoded keys. |
| `sudo bpftool map update id <ID> key <K> value <V>` | Manually change a global variable or map entry while running. |

#### 4. High-Performance & XDP

| Command | Purpose |
| --- | --- |
| `sudo sysctl -w net.core.bpf_stats_enabled=1` | **Required** before checking performance metrics. |
| `sudo bpftool prog show id <ID>` | Check `run_time_ns` and `run_cnt` to calculate avg latency. |
| `sudo bpftool feature probe dev <IFNAME>` | See which helpers/maps are supported by your hardware NIC. |
| `sudo bpftool prog profile id <ID> duration 5` | Profile CPU cycles and instructions per packet. |

---

### Integrating `bpftool` with Aya: A Real Use-Case

When you are developing, your terminal workflow should ideally look like this:

1. **Build:** `cargo xtask build`
2. **Verify Logic:** If loading fails, use `bpftool -d prog load ...` to read the verifier log.
3. **Monitor:** Use `bpftool map dump` in a separate watch window:
```bash
watch -n 1 "sudo bpftool map dump name MY_STATS_MAP"

```


4. **Visualize:** If your `match` statements are getting complex, use the visualizer:
```bash
sudo bpftool prog dump xlated name my_prog visual | dot -Tpng > logic.png

```
---

####  Final Thought

For a person coming from **BCC** and **bpftrace**, is used to the tool doing the "heavy lifting" for you. 
In **Aya**, you are the architect. `bpftool` is your ruler and level—it ensures that what you built in Rust 
actually stands up inside the kernel.


### Verification Checklist 

As the text is generated using LLM's it need to be verified and worked along to build skills and update this
document for corrections: 
Below is a checklist for doing that:


| Layer | What to Verify | Tools / Method |
| --- | --- | --- |
| **Bindings** | Did `aya-tool` map the `struct` correctly? | Open your generated `bindings.rs` and compare a field's type/offset with `bpftool btf dump file /sys/kernel/btf/vmlinux format c`. |
| **Logic** | Did the Rust compiler optimize out my safety check? | Run `sudo bpftool prog dump xlated id <ID>` and look for the specific jump instructions (`jne`, `je`). |
| **Memory** | Is my `RingBuffer` or `HashMap` actually filling up? | Use `sudo bpftool map dump id <ID>`. If you see only zeros, your kernel-side `submit()` or `insert()` is failing. |
| **Logs** | Why isn't my `info!()` showing up? | Ensure you have `RUST_LOG=info` set in userspace and check the kernel pipe: `sudo cat /sys/kernel/debug/tracing/trace_pipe`. |

---

Tips for testing: 

To start verifying the text against your actual code, keep two terminal windows open:

1. **Window 1:** `cargo xtask run` (to keep your app alive).
2. **Window 2:** A "Watch" command to see the kernel's internal state in real-time:
```bash
# Watch your program's run-time and packet count update live
watch -n 1 "sudo bpftool prog show name YOUR_PROG_NAME"

```
