 # Folder Stack Log Format:


 The **Folded Stack** format is the universal "lingua franca" of profiling. 
 It was popularized by Brendan Gregg (the inventor of Flame Graphs) specifically to take the messy, 
 multi-line output of tools like eBPF, `perf`, and `dtrace` and turn them into something a computer can 
 easily parse and visualize.

Contained in files with file ending `.folded` or `.stacks`, this is likely what you're looking at.

---

## The Anatomy of a Folded Stack

The format is extremely simple. Each line represents one unique call stack and follows this pattern:

`[frame1];[frame2];[frame3];...;[leaf_function] [count]`

### A Real-World Example:

Imagine you are profiling a Python web server that is calling a database. 
A single line in your `.folded` file might look like this:

`main;run_server;handle_request;db_query;execute_sql 42`

* **The Semicolons (`;`):** 
    These separate the functions in the stack, starting from the entry point (`main`) on the left and moving 
    to the currently executing function (`execute_sql`) on the right.

* **The Space (`     `):** 
    Separates the call stack from the sample count.

* **The Number (`42`):** 
    This tells us that during the profiling session, the profiler saw the CPU in *this exact state* 42 
    different times.

---

## Why eBPF Uses This Format

eBPF is designed for speed. 
In a high-traffic system, a profiler might capture 10,000 samples per second. 
If it wrote every single sample to a file individually, the disk overhead would actually slow down the 
system you are trying to measure.

**The eBPF "Trick":**

1. **In-Kernel Aggregation:** 
    Instead of sending every sample to your terminal, eBPF maintains a "Hash Map" inside the Linux kernel.

2. **The Stack as a Key:** 
    The call stack itself is the "key," and the "value" is a simple counter.

3. **The Result:** 
    When you stop the profiler, it only has to send the *unique* stacks and their final counts to your 
    screen. This reduces the data transfer by 99+%+.

---

## Converting to Folded Format

Most eBPF tools (like those in the `bcc` or `bpftrace` toolkit) have a specific flag to output this format 
directly.

* **BCC (`profile`):** ```bash
sudo profile -f 10 > out.folded
```

```


* **bpftrace:** You can format the output manually in your script:
```bash
# Simple example to print folded kstacks
bpftrace -e 'profile:hz:99 { @[kstack] = count(); }'

```

### How Speedscope Uses It

When you drop a `.folded` file into speedscope:

1. It parses the semicolons to build a **tree structure**.
2. It sums up the counts to determine the **width** of the bars in the flamegraph.
3. It uses the order to show you the **depth** (the Y-axis).

---

The "Folded" format is what powers the **Left Heavy** view in speedscope, as it naturally groups identical 
paths together.

## Interpret the Sandwich View 

The **Sandwich View** is key for your performance data. 
Helps you find functions that are slow because they are called thousands of times from many different places
(often called "fragmented" bottlenecks).

### The 3-Part Layout

When you select a function in the list, speedscope "sandwiches" it:

1. **Top (Callers):** 
    Shows everyone who called this function. Use this to see **who is to blame** for the load.

2. **Middle (The Function):** 
    Shows the selected function's :
    **Self Time** (time spent in its own code) vs. **Total Time** (its code + everything it called).

3. **Bottom (Callees):** 
    Shows what this function called. Use this to see **where the time is going next**.

---

### A Practical Example: `json.dumps()`

Imagine you are profiling a Python API. 
In the Flamegraph, `json.dumps()` appears in 50 tiny, separate slivers because it's called by 50 different 
routes. It doesn't look like a big deal.

In **Sandwich View**, you sort by **Total Time** and find `json.dumps()` at the top:

* **The Discovery:** You see that across the *entire* app, `json.dumps()` is taking 30% of your CPU.
* **The "Bread" Analysis:**
* **Callers (Top):** You notice `log_request()` is calling it every single time, even when logging is disabled.
* **Callees (Bottom):** You see that a custom `DateTime` encoder inside the JSON logic is actually the specific part that's slow.



### Summary: When to use it?

* Use **Flamegraph** to see the **sequence** of a specific task.
* Use **Sandwich View** to find the **global cost** of a specific function across your whole program.

---

## Folder stack logging with Rust or libbpf():

For building your own eBPF profiler using **Rust (aya/libbpf-rs)** or **C (libbpf)**, the "Folded Stack" 
format is the most lightweight and compatible way to export your data for external analysis.

By outputting folded stacks, your custom tool becomes instantly compatible with **speedscope**, 
**FlameGraph.pl**, and even **Differential Flame Graphs**.

### How to Implement it in your Tool

If you are writing an eBPF-based profiler, your workflow generally looks like this:

#### 1. In the eBPF Kernel Code

You use a **`BPF_STACK_TRACE`** map (or `BPF_MAP_TYPE_STACK_TRACE`) to capture the call stack.

* **Key:** A unique ID for the stack.
* **Value:** The actual instruction pointers (IPs) representing the call stack.
* **Aggregation Map:** You use a second Hash Map where the **Key** is the Stack ID and the **Value** is 
  a counter.

#### 2. In your Userspace Code (Rust/C)

When it's time to generate the log, your userspace program iterates through the Aggregation Map:

1. **Retrieve the Stack ID** and the **Count**.
2. **Look up the Stack ID** in the Stack Trace map to get the list of addresses.
3. **Symbolize** those addresses (turn `0xffffffff810a...` into `vfs_read`).
4. **Print to string:** Join the function names with `;`, add a space, then the count.

---

#####  Example Logic (Pseudo-code)

If your eBPF program captured a stack, your Rust/C userspace "Exporter" would write a line like this to 
a `.folded` file:

```text
// Format: caller;callee;leaf_function count
systemd;nginx;ngx_http_handler;ngx_http_process_request 15

```

##### Why this is better than JSON?

If you are writing a custom tool, you might be tempted to use JSON immediately. However:

* **Size:** Folded stacks are much smaller than JSON.
* **Unix Philosophy:** You can pipe the output of your tool directly into others:
`./my-ebpf-tool | speedscope -`
* **Post-processing:** It's very easy to `grep` or `sed` a folded stack file to filter out specific functions before visualizing them.

---

#### Pro-Tip for Rust eBPF Developers

If you are using **Aya** or **libbpf-rs**, you can also look into the **`inferno`** crate. 
It is a Rust port of the original FlameGraph tools and includes utilities to collapse and manipulate these 
stack strings programmatically.


## Snippet: how the Stack Trace map is defined in C or Rust for an eBPF program:

 To implement this you need two maps working in tandem: 
    - one to store the "DNA" of the stack (the addresses) and 
    - one to count how many times that specific DNA was seen.

---

### 1. The Kernel Side (C / BPF)

In your eBPF program (e.g., `profile.bpf.c`), you define a `STACK_TRACE` map to store the instruction 
pointers and a `HASH` map to aggregate the counts.

```c
// Map to store the actual call stacks (Instruction Pointers)
struct {
    __uint(type, BPF_MAP_TYPE_STACK_TRACE);
    __uint(key_size, sizeof(u32));
    __uint(value_size, 127 * sizeof(u64)); // Up to 127 frames deep
    __uint(max_entries, 10000);
} stack_map SEC(".maps");

// Map to associate a Stack ID with a frequency count
struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(key_size, sizeof(u32)); // Key is the Stack ID from the map above
    __uint(value_size, sizeof(u64)); // Value is the counter
    __uint(max_entries, 10000);
} counts_map SEC(".maps");

SEC("perf_event")
int do_sample(struct bpf_perf_event_value *ctx) {
    // 1. Capture the stack and get a unique ID for it
    int stack_id = bpf_get_stackid(ctx, &stack_map, BPF_F_USER_STACK);
    if (stack_id < 0) return 0;

    // 2. Increment the counter for this specific Stack ID
    u64 *count, init_val = 1;
    count = bpf_map_lookup_elem(&counts_map, &stack_id);
    if (count) {
        (*count)++;
    } else {
        bpf_map_update_elem(&counts_map, &stack_id, &init_val, BPF_ANY);
    }
    return 0;
}

```
---

### 2. The Userspace Side (Rust / libbpf-rs)

This is where you perform the "Symbolization" (converting hex addresses to function names) and format them 
into the **Folded Stack** string.

```rust
// Simplified logic using libbpf-rs/Aya style
fn print_folded_stacks(counts_map: &Map, stack_map: &Map, resolver: &SymbolResolver) {
    for (stack_id_bytes, count_bytes) in counts_map.iter() {
        let stack_id = u32::from_ne_bytes(stack_id_bytes);
        let count = u64::from_ne_bytes(count_bytes);

        // 1. Retrieve the list of IPs for this stack_id
        if let Ok(stack_ips) = stack_map.get_stack(stack_id) {
            let mut symbols = Vec::new();

            // 2. Resolve addresses to names (e.g., 0xff123 -> "malloc")
            for ip in stack_ips {
                let sym = resolver.resolve(ip).unwrap_or("[unknown]");
                symbols.push(sym);
            }

            // 3. Format as Folded Stack: "func1;func2;func3 42"
            // Reverse the list so the root is first
            symbols.reverse();
            println!("{} {}", symbols.join(";"), count);
        }
    }
}

```

### 3. Summary of the Flow

1. **Kernel:** Captures `[0x1, 0x2, 0x3]`  Assigns ID `50`  Increments `Map[50]`.
2. **Userspace:** Sees ID `50` has count `100`.
3. **Userspace:** Looks up ID `50`  Gets `[0x1, 0x2, 0x3]`.
4. **Userspace:** Translates to `main;calculate;add`.
5. **Output:** Prints `main;calculate;add 100` to the `.folded` file.

This modular approach is why eBPF is so performant; the kernel never has to care about function names 
(strings), only numbers.


## Symbolization  example

Symbolization is the process of mapping a raw memory address (e.g., `0x7f8a12`) back to a human-readable 
function name (e.g., `malloc`).

### 1. Kernel Space (`/proc/kallsyms`)

For kernel profiling, the OS maintains a dynamic "phonebook" of every kernel function and its current memory 
address.

* **The Tool:** 
    Your userspace program reads `/proc/kallsyms`.

* **The Logic:** 
    It finds the largest address in the file that is still *less than or equal* to your captured instruction 
    pointer.

### 2. User Space (ELF & DWARF)

For applications (like your Rust or Python code), the names are stored inside the binary file itself.

* **Symbol Table:** A basic list inside the ELF binary that maps names to addresses.
* **Debug Info (DWARF):** A much larger set of data (often stripped in production) that maps addresses back 
  to specific **file names** and **line numbers**.

### 3. The Challenges

* **Stripped Binaries:** 
    If you run `strip` on your Rust binary, the symbol table is deleted. 
    Your profile will just show hex addresses unless you provide a separate "debug link" file.

* **JIT (Just-In-Time):** 
    Languages like Java or Node.js generate code on the fly. 
    Since this code isn't in a file on disk, these runtimes must write a "perf map" file 
    (usually `/tmp/perf-PID.map`) so profilers can resolve the names.

---

### Comparison Table

| Type | Source of Truth | Format |
| --- | --- | --- |
| **Kernel** | `/proc/kallsyms` | Plain text list |
| **Native (Rust/C)** | ELF Binary / DWARF | Binary metadata |
| **JIT (Node/Java)** | `/tmp/perf-PID.map` | Runtime-generated text |

With this full picture from eBPF maps to folded stacks and symbolization, below is a basic Rust 
"symbol resolver" snippet using the `addr2line` or `gimli` crates:

## Example: Rust Symbol Resolver ( using `addr2line` or 'gimili')

In Rust, the most common way to handle symbolization is using the **`backtrace`** crate (for simple 
resolution) or the **`addr2line`** crate (if you need to read specific ELF files and DWARF data).

For an eBPF tool, you typically use **`addr2line`** because you are resolving addresses captured from a 
*different* process, not your own.

---

### Basic Resolver using `addr2line` & `gimli`

This snippet shows how to map a raw address from an ELF file to a function name.

```rust
use addr2line::{Context, object::{self, Object}};
use std::fs;
use std::path::Path;

fn resolve_addr(bin_path: &Path, addr: u64) -> Result<String, Box<dyn std::error::Error>> {
    // 1. Load the binary file (ELF)
    let file_data = fs::read(bin_path)?;
    let obj = object::File::parse(&*file_data)?;
    
    // 2. Create a context for DWARF debugging info
    let ctx = Context::new(&obj)?;

    // 3. Find the function name for the specific address
    // We subtract the base address if the binary is a Position Independent Executable (PIE)
    let mut frames = ctx.find_frames(addr).skip_all_loads()?;
    
    if let Some(frame) = frames.next()? {
        if let Some(function) = frame.function {
            // Demangle Rust/C++ names (e.g., _ZN3std... -> std::...)
            return Ok(function.demangle()?.to_string());
        }
    }

    Ok("??".to_string())
}

```

---

### Handling "The Map" (Process Memory)

In a real eBPF tool, addresses are **Virtual Addresses**. To resolve them, you have to find where the 
library or binary is mapped in memory:

1. **Read `/proc/[PID]/maps**`: This tells you that `libssl.so` is loaded at base address `0x7f8a0000`.
2. **Calculate Offset**: If the eBPF capture gives you address `0x7f8a1234`, you calculate:


3. **Search ELF**: You look for symbol `0x1234` inside the `libssl.so` file on disk.

### Recommended Libraries for Production

* **[blazesym](https://github.com/libbpf/blazesym)**: 
    A high-performance symbolization library from the Libbpf team (written in Rust). 
    It is specifically designed for eBPF profilers and handles `/proc/maps` and symbol caching automatically.

* **[inferno](https://github.com/jonhoo/inferno)**: 
    Excellent for taking these resolved names and "folding" them into the Flamegraph/Speedscope format.

