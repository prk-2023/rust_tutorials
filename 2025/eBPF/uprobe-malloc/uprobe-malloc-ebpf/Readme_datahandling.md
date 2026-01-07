# managing data in the eBPF program:

The key issue while working with eBPF programs in Rust is the hurdle to manage data without a heap while
satisfying the borrow checker. 

Since You cannot move data onto the heap ( no `String` , `Vec` ... heap Types ) you must rely on
**static-pinning** or **stable storage**, below are the three best patters to handle this effectively in
a constrained environment. 

## 1. The Re-Binding patter ( Common approach )

In this the owner of the bytes is bound to a variable at least as long as the reference you are using.

```rust 
// WRONG: The temporary result of ctx.command() is dropped at the semicolon
let comm = unsafe { core::str::from_utf8_unchecked(&ctx.command().unwrap()) }; 

// RIGHT: comm_bytes stays on the stack for the whole function
let comm_bytes = ctx.command().unwrap_or([0u8; 16]); 
let comm = unsafe { core::str::from_utf8_unchecked(&comm_bytes) };
```


## 2. Using BPF Maps for Global Scratch Space:

If the data to manage is larger then 512 bytes which is the stack limit, in which case you should not use
large strings or structures on the stack at all. 

Instead use a **Per-CPU Array Map** as *"scratch buffer"*. This keeps memory off the stack and provides a
stable location for data.  ( NOTE:  refer to hash_map example )

```rust 
#[map]
static mut SCRATCH_BUF: PerCpuArray<[u8; 16]> = PerCpuArray::with_max_entries(1, 0);

// Inside your function:
let ptr = SCRATCH_BUF.get_ptr_mut(0).ok_or(0u32)?;
let comm_bytes = unsafe { &mut *ptr };
// Now fill comm_bytes and convert to &str...
```

## 3. The Zero-Copy Pattern with `CStr` :

Since eBPF often deals with null-terminated strings from the kernel, using `core::ffi::CStr` is  often
safer and more idiomatic than `str::from_utf8`. 
This avoids the need to manually manage slices and handles the "garbage" data after the null terminator.


## Comparison of Strategies:

| Strategy | Memory Location | Pros | Cons |
| :--- | :--- | :--- | :--- |
| Stack Binding |Stack |"Fastest, simplest code."|Risky; easy to hit the 512B stack limit. |
| Per-CPU Maps | Kernel Memory |"Stable, bypasses stack limits."| Slightly slower; requires map lookup.|
|Inline Processing |Register/Stack |Lowest overhead | Very difficult to satisfy the borrow checker|

For logging small data like command name ( 16bytes) **Stack Binding** is prefectly fine. 
If processing larger structures ( like `task_struct` of file paths ) Switch to **Per-CPU Maps** to avoid
stack limit exceeded error during verification.

--------------------------------------------------

# Using Per-CPU Array Map in eBPF (Aya + Rust) ( refer to hash_map example )

In eBPF, a **per-CPU array map** is a type of map where each CPU has its own independent entry in the map. This is useful when you want to store data that is CPU-local (such as statistics, counters, or buffers) and avoid contention between CPUs. Using a **per-CPU array map** as a "scratch buffer" can be a good way to keep memory off the stack and provide a stable location for data that’s isolated per CPU.

### Why Use a Per-CPU Array Map?

A **per-CPU array map**:

1. **Avoids contention** between different CPUs: Each CPU has its own copy of the array, so there is no need for locks or synchronization when writing data from different CPUs.
2. **Improves performance**: It allows concurrent access without costly synchronization mechanisms (like mutexes).
3. **Provides stability**: The data is stored in kernel memory, which is more stable than relying on stack or heap memory, which could be invalidated once the function exits.

You can use this map as a "scratch buffer" to hold data temporarily in kernel space. For example, you could store the command name or other values you need to keep track of during eBPF program execution.

### Setting Up a Per-CPU Array Map

In Aya, you can set up a per-CPU array map using `Map` from `aya::maps`. Here’s an example of how to create and use a per-CPU array map for storing data:

#### 1. Creating a Per-CPU Array Map

To create a per-CPU array map, you need to initialize it in your eBPF program. The map will be indexed by the CPU ID, so each CPU will have a separate entry.

```rust
use aya::maps::{Map, PerCpuArray};
use aya::programs::{Xdp, UProbe};
use aya::ebpf::{BpfMap, BpfMapType};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct MyData {
    count: u32,
    size: u32,
}

fn create_per_cpu_array_map() -> Result<PerCpuArray<MyData>, Box<dyn std::error::Error>> {
    let mut map = PerCpuArray::<MyData>::new(16, 1024)?;
    map.set_flags(0); // optional, if you want any flags

    Ok(map)
}
```

In the code above:

* **`PerCpuArray<MyData>`**: This is a per-CPU array map where each CPU has its own entry of type `MyData`.
* **`16`**: This specifies the number of CPUs, and the size of each entry will be `MyData` (which can be used as a scratch buffer).
* **`1024`**: This is the map's capacity (the maximum number of entries).

#### 2. Accessing the Map in the eBPF Program

You can read and write to the per-CPU array map directly from the eBPF program. When you access the map, the data is automatically isolated to the current CPU. Here’s how to interact with it in an eBPF program:

```rust
#[uprobe]
pub fn uprobe_malloc(ctx: ProbeContext) -> u32 {
    // Access the per-CPU map
    let map: &mut PerCpuArray<MyData> = get_map(ctx);

    // Retrieve the current CPU's entry (based on CPU ID)
    let cpu_id = ctx.cpu_id();
    let mut data = map.get_mut(cpu_id).unwrap();

    // Update the "scratch buffer" for the current CPU
    data.count += 1;
    data.size += ctx.arg(0).unwrap_or(0) as u32; // example for size

    // Return success
    0
}
```

* **`ctx.cpu_id()`**: Retrieves the current CPU ID. This is useful for reading and writing to the appropriate entry in the per-CPU map.
* **`map.get_mut(cpu_id)`**: This retrieves the data for the current CPU, so each CPU maintains its own state in the map.

### Rust FFI for Handling Data in eBPF

You also asked about using **Rust FFI (Foreign Function Interface)** and `core::ffi::CStr` in this context. Here's an explanation and how it could fit into the situation:

#### Can You Use Rust FFI and `CStr` for Data Handling?

Yes, **you can use Rust FFI** to handle C-style strings (`CStr`) in eBPF programs, especially when working with data that comes from C code, like process names or other C-string data passed into the kernel from user space.

#### What is `core::ffi::CStr`?

The `CStr` type in Rust is used to work with **C-style strings** (i.e., null-terminated strings). It provides a safe wrapper around raw C strings (`*const c_char`) and allows you to safely convert them into Rust strings (`&str`).

```rust
use core::ffi::CStr;

fn to_rust_string(c_str: *const i8) -> Option<&str> {
    // SAFETY: We assume the C string is valid and null-terminated
    unsafe {
        CStr::from_ptr(c_str).to_str().ok()
    }
}
```

In an eBPF program, you might be dealing with data such as the **command name** of a process, which might come as a C-string (e.g., from `ctx.command()` or other kernel interfaces).

You can use `core::ffi::CStr` to convert these raw C strings to a Rust string and process them safely.

#### Example: Using `CStr` with eBPF

If you want to convert a C-style string to a Rust `&str` within your eBPF program (for example, from a process's command name stored in a map or passed through the probe), you can do so as follows:

```rust
use core::ffi::CStr;
use aya::programs::ProbeContext;

fn get_command_name(ctx: &ProbeContext) -> Option<&str> {
    // Fetch the command name, which is expected to be a C-style string
    let comm_ptr = ctx.command().ok()?;
    
    // SAFETY: We're assuming the pointer is valid and points to a C-style string
    unsafe {
        // Convert the C string to a Rust string
        CStr::from_ptr(comm_ptr.as_ptr() as *const i8).to_str().ok()
    }
}
```

* **`CStr::from_ptr(comm_ptr.as_ptr() as *const i8)`**: Converts the raw pointer to a `CStr`.
* **`.to_str().ok()`**: Converts the `CStr` to a `&str`, handling potential invalid UTF-8 gracefully.

### Combining Per-CPU Array Map with `CStr`

You could combine a per-CPU array map with the `CStr` conversion to handle process command names and other C-string data efficiently. Here's how:

1. **Use the per-CPU map** to store temporary data or a scratch buffer.
2. **Convert C strings** to Rust `&str` using `CStr` to handle data that’s passed between the kernel and user space (like command names).

Example combining both ideas:

```rust
#[uprobe]
pub fn uprobe_malloc(ctx: ProbeContext) -> u32 {
    let size: usize = ctx.arg(0).unwrap_or(0);
    let pid = ctx.pid();
    
    // Access the per-CPU map
    let map: &mut PerCpuArray<MyData> = get_map(ctx);
    
    // Get the current CPU's entry in the map
    let cpu_id = ctx.cpu_id();
    let mut data = map.get_mut(cpu_id).unwrap();
    
    // Update the per-CPU data
    data.count += 1;
    data.size += size as u32;

    // Get command name (C-string) and convert to Rust string
    let comm_name = get_command_name(ctx);
    
    // Log info
    info!(
        &ctx,
        "{} app with PID {}: alloc called for {} bytes",
        comm_name.unwrap_or("unknown"),
        pid,
        size
    );
    
    Ok(0)
}
```

### Conclusion

* **Per-CPU Array Map**: Use a per-CPU array map to store data that is local to each CPU, avoiding contention and reducing the need for synchronization. This can act as a "scratch buffer" for temporary data in your eBPF program.
* **Rust FFI and `CStr`**: You can use `core::ffi::CStr` to handle C-style strings safely in your eBPF program. This is useful for converting raw C strings (like process names) into Rust strings.

Both approaches can be combined to efficiently store and manipulate data in kernel space, ensuring that you handle data safely and without unnecessary memory allocations.
