# How to Use Ayas Context "ctx"  in eBPF  kernel programs:


To read values from a context, you first need to understand that the "context" is a pointer to a 
struct that changes depending on the type of program you are writing 
(e.g., `XdpContext`, `SkBuffContext`, or `TraceContext`).

How to access data from these contexts safely.

Aya eBPF Context allows you to access data related to the event or tracepoint that triggered the eBPF
program. 

In Rust accessing the context is done via *pointer-based* or *memory-mapped* techniques. 

The context typically give you access to **registers**, **arguments** or structures related to the event or
tracepoint.

Example:
```rust 
// Assuming `tracepointContext` has fields like 'registers', 'args', etc.
fn test(ctx: tracepointContext) -> Result<u32, ()> {
    let value_from_context = ctx.some_field;  // Access a specific field from the context

    if value_from_context > 0 {
        Ok(42)  // Return success with a u32 value (42)
    } else {
        Err(())  // Return failure with no error data
    }
}
```

In real-world eBPF programs, this context might refer to **register value**, **stack traces** or **kernel
data** that you can access. 

In Rust these values are typically accessed via FFI bindings, but in `aya` this might be abstracted behing
safe abstraction.

---

## 1. Understanding the Context

Every eBPF program has a single argument, usually called `ctx`. 
This context provides access to the data associated with the event that triggered your program.

| Program Type | Context Struct | Common Data Accessed |
| --- | --- | --- |
| **XDP** | `XdpContext` | Raw network packets (Direct memory access) |
| **TC/Sched** | `SkBuffContext` | Network packets + Socket metadata |
| **Kprobe** | `ProbeContext` | CPU registers and function arguments |
| **Tracepoint** | `TraceContext` | Specific fields defined by the kernel tracepoint |

---

## 2. Reading Network Packets (XDP Example)

In XDP, the context gives you a `data` pointer (the start of the packet) and a `data_end` pointer. 
You must perform **bounds checking** before reading, or the kernel verifier will reject your program.

### The Code

```rust
use aya_ebpf::programs::XdpContext;
use aya_ebpf::bindings::ethhdr;
use memoffset::offset_of;

pub fn try_packet_read(ctx: XdpContext) -> Result<u32, ()> {
    let start = ctx.data();
    let end = ctx.data_end();

    // 1. Calculate the size of the header we want to read
    let len = ethhdr::LEN;

    // 2. Bounds Check (Essential for the Verifier!)
    if start + len > end {
        return Err(());
    }

    // 3. Read the data safely
    // We cast the raw pointer to our header type
    let eth_header = unsafe { &*(start as *const ethhdr) };
    
    // Now you can access fields like eth_header.h_proto
    Ok(0)
}

```

---

## 3. Reading Function Arguments (Kprobe Example)

When using a `Kprobe`, you are usually interested in the arguments passed to a kernel function. 
Aya provides a helper method `.arg()` to extract these from the registers.

```rust
use aya_ebpf::programs::ProbeContext;

pub fn handle_kprobe(ctx: ProbeContext) -> u32 {
    // Read the first argument (index 0) of the function being probed
    // For example, if probing 'do_sys_open', arg(1) might be the filename pointer
    let arg0: u64 = match ctx.arg(0) {
        Some(val) => val,
        None => return 1,
    };

    0
}

```

---

## 4. Reading Memory from Pointers (Helpers)

Often, the context gives you a pointer to a memory address in the kernel or user-space 
(like a filename string). You cannot dereference these directly in eBPF. 
You must use `bpf_probe_read_kernel` or `bpf_probe_read_user` helpers.

Aya provides a safe wrapper for this:

```rust
use aya_ebpf::helpers::gen::bpf_probe_read_kernel;

// Assuming 'ptr' is a pointer found within your context
let mut my_value: u32 = 0;
unsafe {
    bpf_probe_read_kernel(
        &mut my_value as *mut u32 as *mut _, // Destination
        4,                                   // Size
        ptr as *const _                      // Source pointer
    );
}

```

---

### Key Takeaways

* **Always Bounds Check:** 
  If you are reading packet data, you must prove to the compiler that `start + offset` is less than `end`.

* **Use `aya-ebpf` Bindings:**
  Use the pre-generated bindings for kernel structs (like `ethhdr`, `iphdr`) to ensure memory layouts match.

* **Unsafe Blocks:** 
  Reading from raw pointers in the kernel is inherently `unsafe` in Rust, so you will need to wrap these 
  operations in `unsafe {}` blocks while keeping the surrounding logic as safe as possible.

