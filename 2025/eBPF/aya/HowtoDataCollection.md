# How to data collection with Aya:

## Kernel side: eBPF program 

### 1. `aya_log`:

- `aya_log` is a utility that's commonly used with `aya` for logging and debugging. This is not the core
  part of the eBPF program logic but its handy for development and troubleshooting. 

- `aya_log` can be used to log important information such as variable values, program state and errors. 

Example:
```rust
use aya_log::log;

fn test(ctx: tracepointContext) -> Result<u32, ()> {
    log::info!("Accessing value from context: {:?}", ctx.some_field);  // Log the context field

    if ctx.some_field > 0 {
        log::debug!("Returning success with value 42");
        Ok(42)
    } else {
        log::error!("Error: Invalid value in context");
        Err(())
    }
}
```

This way `aya_log` is helpful when trying to debug or monitor the behaviour of your eBPF program in
production. 


### 2. maps:

- `eBPF` Maps are fundamental data structs used to store and share data between the kernel and user space. 

- In your Rust code, you can interact with these maps using the `aya` crate. 
  Maps can store things like **counters**, **strings**, or **complex data structures**, and they provide a
  way to persist state between tracepoints or function invocations.

Example, in an eBPF program, you could use a map to store some counter and update it on each tracepoint hit:

```rust 
use aya::maps::{Map, HashMap};
use aya::programs::{Program, TracePoint};

fn test(ctx: tracepointContext) -> Result<u32, ()> {
    // Access an eBPF map (example: a counter map)
    let map = unsafe { Map::<u32, u32>::load("/path/to/map").unwrap() };

    let current_value = map.get(&1).unwrap_or(0);  // Get the value from the map
    let updated_value = current_value + 1;  // Update the value
    map.insert(1, updated_value);  // Insert the new value back into the map

    Ok(updated_value)
}
```
- `Map`: 
  This represents the `eBPF` map. 
  In this case, we used a simple `HashMap` where `u32` is both the `key` and the `value`. 
  You’ll need to specify the map type and its path in the kernel.

- Operations: 
  You can use `.get()`, `.insert()`, and `.remove()` to interact with the map.

### 3. Ring Buffers:

Ring Buffers are important data structure for high-performance data logging in eBPF. 
They allow for FIFO (First In, First Out) data storage, which is crucial for scenarios where you need to 
collect event data and analyze it later.

Ring buffers are often used in logging scenarios where you want to store a series of events 
(such as tracepoint invocations) and then read them in the order they were produced, without blocking or 
dropping data.

To interact with a ring buffer in Rust with the `aya` crate, you could use something like this:
Example:
```rust 
use aya::maps::RingBuffer;

fn test(ctx: tracepointContext) -> Result<u32, ()> {
    // Assuming `ring_buffer` is a ring buffer initialized for tracepoint data
    let mut ring_buffer = RingBuffer::<u32>::load("/path/to/ring_buffer").unwrap();

    let event = ctx.some_field;  // Some data from the tracepoint context
    ring_buffer.push(event);  // Push data into the ring buffer

    // Optionally, read the latest data from the ring buffer
    if let Some(latest_event) = ring_buffer.pop() {
        println!("Latest event: {}", latest_event);
    }

    Ok(42)  // Or whatever value you need to return
}
```

- **`RingBuffer`**: 
  It allows you to push new entries into the buffer and pop existing ones. 
  This is useful for collecting tracepoint events over time.

- **Size Constraints**: 
  Important to keep in mind is that ring buffers have a fixed size. 
  When the buffer is full, older data gets overwritten, which is a key characteristic of the "ring" 
  structure.

### 4. Perf buffers:

- `Perf buffers` are similar to ring buffers but are designed specifically for high-throughput, low-latency
  data collection in eBPF programs. 

- They are mainly used for event tracing and performance monitoring. 

- Use case: Perf buggers are used when you want to collect and export large amount of event data from the
  kernel to user space without dropping any data, especially for things like networking or performance
  counters. 

- The kernel will push data into the buffer, and user-space program can poll the buffer to retrieve data for
  high-volume data collection.

Ex:
```rust 
use aya::maps::PerfBuffer;

fn test(ctx: tracepointContext) -> Result<u32, ()> {
    let perf_buffer = PerfBuffer::<u32>::load("/path/to/perf_buffer").unwrap();
    
    // You can push the data directly into the buffer
    let event = ctx.some_field;
    perf_buffer.push(event);

    Ok(42)
}
```

- The Key difference from Ring Buffers is  Perf buffers use an optimized kernel mechanism for faster data
  transfer to user space.

### 5. `Skb` Buffers ( for network tracing ):

- `eBPF` progs that interact with networking features (`XDP` or `tc` progs) often use `skb` (socket buffers) 
to collect data. These buffers store packet data, and `eBPF` programs can access network packets as they 
flow through the kernel.

- Use Case: Network tracing, monitoring, and even dropping packets at high speeds using XDP.

Example: 
An eBPF program might be used to collect network packet statistics, like byte counts, or perform security 
checks like filtering or blocking malicious traffic.

### 6. BPF Type Format (BTF) and BPF Object Files

- `BTF` is a debugging and type-information mechanism that allows `eBPF` programs to access complex data 
  structures in kernel memory. This is important when your `eBPF` program needs to *parse or collect data*
  from structures with rich data types (like structs, unions, etc.) rather than raw integers or arrays.

- BPF Object Files (eBPF program objects): 
  They are compiled eBPF programs (like .o files) that can be loaded and attached to various parts of the 
  kernel. These files contain compiled instructions but can also be used to map data from user space.

### 7. BPF-Trace (eBPF-based Tracing Framework)

- BPF-Trace allows for tracing in a way that simplifies collecting data for performance and debugging. 
  It uses tracepoints and event-based collection. You typically write short trace scripts in a high-level 
  language, and the data is passed back to user space.

- Use Case: Collect performance metrics, trace system calls, and more.

- Integration: 
  While `BPF-Trace` is a user-space tool, it's built on top of `eBPF` and can be used in combination with 
  other methods (like `maps` or `perf buffers`).

---

## User-Space part:

While some data collection happens in the kernel (within the eBPF program), the user-space program is 
responsible for interfacing with the kernel, polling for events, retrieving data, and possibly analyzing 
the data.

### 1. Maps in User-Space :

`eBPF` maps are one of the primary mechanisms for data collection and sharing between kernel-space eBPF prog 
and user-space programs.

- How to interact with `maps` in user space:
  In user-space application using `aya`, you can use the `aya::maps::Map` API to interact with `eBPF maps`. 
  The program in user space opens the `map`, `reads` or `writes` to it, and monitors changes.

Example of how a user-space program interacts with an eBPF map:
```rust 
use aya::maps::{Map, HashMap};
use aya::programs::TracePoint;
use aya::Bpf;

fn user_space_example() -> Result<(), Box<dyn std::error::Error>> {
    let bpf = Bpf::load_file("/path/to/bpf/program.o")?;

    // Access a map from the BPF program
    let map = bpf.map_mut::<HashMap<u32, u32>>("map_name")?;

    // Read the value stored in the map
    if let Some(value) = map.get(&1)? {
        println!("Value from map: {}", value);
    }

    Ok(())
}
```
- The program calls `map.get()` or `map.insert()` to interact with the map. 
- Maps can store complex data types (like `counters`, `histograms`, etc.) and provide a mechanism to collect
  that data in user space.

### 2. Perf Buffers in User-space:

- Perf buffers are also pollable in user space. 
- In user-space programs, you use the `PerfBuffer` API to read events or messages from the kernel. 
  A user-space program listens for events that were pushed to the buffer and processes them.

Example of how to use perf buffers with aya:

```rust 
use aya::maps::{PerfBuffer, PerfBufferConfig};

fn user_space_perf_example() -> Result<(), Box<dyn std::error::Error>> {
    let bpf = Bpf::load_file("/path/to/bpf/program.o")?;
    let perf_buffer = PerfBuffer::new(bpf.perf_buffer_mut("events")?);

    // Poll for data from the perf buffer
    loop {
        if let Some(event) = perf_buffer.next() {
            println!("Received event: {:?}", event);
        }
    }

    Ok(())
}
```
- The `PerfBuffer` is a user-space object that allows polling for kernel-generated events.

- The user-space program waits for new events in the buffer and processes them as they arrive. 
  This makes it ideal for real-time event processing or performance monitoring.

### 3. Ring Buffers in User-Space 

Ring buffers are usually handled in the kernel, user-space programs can interact with ring buffers through 
the `aya` crate. The typical use case is that a kernel-side `eBPF` program writes to the buffer, and the 
user-space program reads data from it.

Example of interacting with a ring buffer:

```rust 
use aya::maps::RingBuffer;

fn user_space_ring_buffer_example() -> Result<(), Box<dyn std::error::Error>> {
    let bpf = Bpf::load_file("/path/to/bpf/program.o")?;
    let ring_buffer = RingBuffer::new(bpf.ring_buffer_mut("ring_buffer_name")?);

    // Poll for data in the ring buffer
    loop {
        if let Some(event) = ring_buffer.pop() {
            println!("New event: {:?}", event);
        }
    }

    Ok(())
}
```
- The user-space program reads the events pushed into the ring buffer. 
- Ring buffers are typically used when you have high-frequency events that need to be captured and processed
  in real-time.

### `aya_log` and User-Space Interaction

`aya_log` itself is primarily for debugging and logging inside the `eBPF` program or user-space program. 
It doesn't handle data collection directly but helps developers by logging messages during development or 
troubleshooting.

Example, while working with `aya` to debug the `eBPF` program in user space, you use logging as follows:

```rust 
use aya_log::log;

fn debug_example() {
    log::info!("Debugging eBPF program execution");
}
```
- Logs in the kernel: 
  If you're debugging the kernel-side eBPF program, `aya_log` will print messages to the kernel's debug log 
  (/sys/kernel/debug/tracing/).

- Logs in user space: 
  If you're debugging user-space, `aya_log` can print messages to the console or file, helping you trace 
  data flow.



