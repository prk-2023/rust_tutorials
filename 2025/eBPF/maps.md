# Maps:

**eBPF Maps** are essential mechanism used in eBPF eco-system used for:
    - Storing Data 
    - Sharing Data between different components. 

They serve as generic, persistent, and reliable storage abstraction. 

Roles of eBPF:
1. Communication between Kernel And User Space: 

- Maps are the main channel for two-way communication. 

- User-Space applications can use maps to **configure** an eBPF program running in kernel(ex: Setting policy
  IP address). 

- Conversely an eBPF program can use maps to **send data** back to user-space ( ex: Counters, logs, or
  performance statistics).

2. Share state between eBPF programs:

- Multiple eBPF programs (even different types attached to different hook points) can use the same map to 
  share stateful information.
  Ex: A progr attached to pkt reception could update a counter, and another program attached to a
      system call could read that same counter.

3. Lookup Tables:

- Maps serve as highly efficient lookup tables that the eBPF Verifier allows for. 

- This enables eBPF programs to quickly make decisions without violating the kernel's safety rules.


The key aspect of eBPF is its ability to work with "maps", which is crucial data structures used to store
and manage state across different eBPF programs.

Maps in eBPF can be used for variety of purposes, such as tracking network statistics, maintaining counters,
or sharing data between user-space and kernel-space.

## Overview of How Maps work in eBPF system: 

### 1. What are eBPF Maps?

**eBPF Maps** are containers that store data in the kernel space.

These maps are **key-value** stores, and **each map is created with a specific type and configuration**. 

The key and value sizes are fixed at map creation time but can be customized 
(ex: a 4-byte IP addr key and a 64-byte struct value).

The data stored in these maps can be accessed and modified by eBPF programs running within the kernel, 
such as those attached to network devices, tracepoints, or performance monitoring hooks.

=> Maps are created, configured, and managed by the **user-space program** using `bpf()` system call. 
=> Once maps are created they reside in the kernel's memory and **persist** even if the eBPF program that
   uses them is detached or updated. 

=> **Kernel Helpers**: eBPF programs interact with maps using special **helper functions** provided by the
   kernel such as :
   - `bpf_map_lookup_elem()`  ( to read a value )
   - `bpf_map_update_elem()`  ( to write or update a value )
   The helpers job is to maintain safety and performance.

### 2. Types of eBPF Maps

eBPF maps come in several types, each optimized for a specific use case. 

There are about over 20+ different types of maps in eBPF, which can be grouped on their underlying data 
structure and primary purpose.

New maps keep getting added to kernel, the list of available *map types* is  defined by `enum_bpf_map_type`
in the kernel source code.

#### Generic Key-Value Stores ( Basic Map )

| Map Type | Description | Primary Use Case |
| :--- | :--- | :--- |
| Hash Table (BPF_MAP_TYPE_HASH) | A dynamic associative array (dictionary/hash map) that stores key-value pairs. Optimized for fast lookups.| Storing configuration, caching state (ex flow tracking), or collecting statistics."|
| Array (BPF_MAP_TYPE_ARRAY)| A fixed-size array where the key is a 32-bit integer index. All elements are pre-allocated.| Simple counters, lookup tables, or storing data where the key is known and sequential.|
| LRU Hash/Array (BPF_MAP_TYPE_LRU_HASH)|A hash map that automatically removes the Least Recently Used entries when it reaches its maximum capacity.| Caching frequently accessed data while bounding memory usage.|
| Per-CPU Variants (BPF_MAP_TYPE_PERCPU_*)| These maps (PERCPU_HASH, PERCPU_ARRAY) provide a separate copy of the map value for every CPU core.|Storing fast, lockless counters or scratch space for eBPF programs running on the same CPU.|

The most common types are:
    * Hash Map: 
        Stores key-value pairs in a hash table. 
        It's ideal for scenarios where you need to store data with arbitrary keys.

    * Array Map: 
        Stores values at indexed positions (like an array). 
        Each index must be a unique key, and it’s very fast for lookups.

    * Per-CPU Map: 
        Keeps separate data for each CPU core, useful for cases where you need to minimize contention 
        between cores.

    * LPM Trie Map: 
        A map that stores prefixes for IP addresses (Longest Prefix Match). 
        It’s used in network filtering, such as routing decisions.

    * Queue Map: 
        A fixed-size circular buffer where data can be pushed and popped. 
        It’s often used for logging or passing data between eBPF programs.

    * Stack Map: 
        A stack structure that allows eBPF programs to push and pop data.
        It’s often used for maintaining call stacks.

    * BPF Type Format (BTF) Map:
        A specialized map that works with BPF Type Format (BTF) data, useful for advanced kernel debugging.


#### Special purpose Storage:

These maps are optimized for specific networking, tracing, or performance use cases:

| Map Type | Description | Primary Use Case |
| :-- | :-- | :-- |
| Longest Prefix Match (LPM Trie) (BPF_MAP_TYPE_LPM_TRIE) | A data structure optimized for prefix matching on keys of variable length (like IP addresses).| "Routing and Policy: Finding the best match for an IP address in a routing table (e.g., matching a packet's destination IP to the most specific subnet)."|
| Program Array (BPF_MAP_TYPE_PROG_ARRAY)| An array that holds file descriptors (references) to other eBPF programs.| """Tail Calls"": Implementing control flow logic by dynamically calling another eBPF program, effectively chaining them together."|
| DevMap/SockMap (BPF_MAP_TYPE_DEVMAP)|Maps used for efficiently redirecting network traffic. DevMap stores the index of an egress network device.| High-Performance Networking: Redirecting packets to a different network interface (like a router) or to a specific socket.|
| Cgroup Storage (BPF_MAP_TYPE_CGROUP_STORAGE) | Stores data associated directly with a specific Linux Control Group (cgroup). | Storing per-cgroup state or configuration for monitoring and policy enforcement.|

### 3. **Map Operations**

eBPF programs interact with maps through a set of operations, typically:

* Map Creation: 
    Maps must be created and configured using specific system calls or eBPF helper functions. 
    For instance, the `bpf_map_create()` system call allows you to create maps with various properties such 
    as size, type, and more.

* Insertion/Updating Data: 
    eBPF programs can insert or update values in a map using helper functions like `bpf_map_update_elem()`,
    which takes a map, key, and value to update or insert.

* Lookup: 
    To retrieve data, eBPF programs use the `bpf_map_lookup_elem()` helper, which takes a key and returns 
    the associated value.

* Deletion: 
    Data can be removed using the `bpf_map_delete_elem()` function.

* Iterating: 
    Maps can be iterated over to access all keys:values. For ex, you can use `bpf_map_get_next_key()` to 
    traverse a map.

### 4. **Accessing Maps from User-Space**

Although eBPF programs are executed in the kernel, maps can be shared with user-space programs. 
User-space applications can interact with eBPF maps through the `bpf()` syscall (with `BPF_MAP_GET_FD_BY_ID`
and other operations). 

This allows user-space programs to read and modify data in kernel-space maps, which is essential for
monitoring, debugging, or dynamically controlling eBPF programs.

### 5. **Map Usage Patterns**

Maps can be used in various ways depending on the requirements of the eBPF program:

* State Management: 
    eBPF programs often maintain state (e.g., counters or aggregates) in maps. 
    For ex: a program attached to a network interface can count packets per IP addr and store the counts in 
    a hash map.

* Sharing Data Across Programs: 
    Maps allow different eBPF programs to share data. 
    For ex: a program collecting performance metrics might store the results in a map, while another program 
    reads the metrics to generate a report.

* Caching: 
    Maps can be used to cache frequently accessed data, reducing the need for repeated expensive kernel 
    operations. For ex: a program might cache information about a recently accessed network flow to speed 
    up future lookups.

### 6. Map Lifetime and Persistence

    Maps persist across invocations of eBPF programs as long as they are not deleted. 
    However, they are tied to the lifecycle of the BPF object (ex: a network filter or tracing program) 
    that created them. When the BPF program is unloaded or the map is deleted, its contents are erased.

    * Pinning: 
    Maps can be “pinned” to the filesystem using the `bpf_obj_pin()` helper function, which allows them to 
    persist beyond the lifetime of the eBPF program. This is useful for long-term storage or sharing data 
    across different processes.

### 7. Access Control and Synchronization

eBPF maps are inherently thread-safe, but the kernel enforces strict access control and synchronization 
mechanisms to ensure that multiple eBPF programs can safely access the same map concurrently.

* Locking: 
    Certain map types, such as hash maps, may require internal locking to prevent race conditions when 
    accessed by multiple CPU cores or multiple eBPF programs.

* Atomic Operations: 
    Some maps support atomic operations, which allow for efficient incrementing or other operations on 
    values without needing additional synchronization.

### 8. Examples of eBPF Map Use Cases

* Network Traffic Monitoring: 
    An eBPF program attached to a network device can use a hash map to store statistics for each IP address 
    or port. 
    For example, you might count the number of packets or bytes received per source IP.

* Security: 
    eBPF programs can use maps to store information related to security events, such as a list of known 
    malicious IPs, and use that data to filter or block packets.

* Tracing and Profiling: 
    eBPF can be used for system tracing, and maps can store various performance metrics or function call 
    data for later analysis.

### 9. Performance Considerations

While eBPF maps are efficient, it’s important to consider the performance impact of using them:

* Map Size: 
    Large maps can introduce overhead, especially if they require frequent updates or iterations. 
    It’s important to balance the map size with the performance requirements of the system.

* Contention: 
    If multiple eBPF programs are modifying the same map concurrently, you may encounter performance 
    bottlenecks due to contention. This is especially relevant for hash maps and other non-per-CPU maps.

* Synchronization Costs: 
    Operations like locking or atomics to prevent race conditions add overhead to the system. 
    Some map types (e.g., per-CPU maps) minimize this overhead by isolating data on each CPU core.


### Communication and Event Streaming:

| Map Type | Description | Primary Use Case |
| :-- | :-- | :-- |
| Ring Buffer (BPF_MAP_TYPE_RINGBUF) | "A modern, highly efficient alternative to the Perf Event Array for asynchronous data transfer." | "Logging/Monitoring: Streaming event data (logs, metrics) from eBPF programs to a user-space consumer with minimal overhead."|
| Perf Event Array (BPF_MAP_TYPE_PERF_EVENT_ARRAY)| An array of pointers to per-CPU kernel ring buffers.|"Telemetry: Exporting trace events, performance statistics, or sampled data from the kernel to user space."|
| Stack Trace (BPF_MAP_TYPE_STACK_TRACE) | Stores unique kernel or user-space stack traces. The map returns a handle (ID) that can be used as a key in other maps.| Profiling/Debugging: Collecting and storing stack traces to understand where events (like system calls or packet drops) are occurring.|

In summary, maps in eBPF are fundamental for storing and sharing data between eBPF programs and user-space 
applications. 
They offer a range of types for different use cases, such as hash maps, array maps, and per-CPU maps. 
Through maps, eBPF programs can maintain state, track events, and share information in a fast and efficient 
way. Understanding how maps work is key to leveraging the full potential of eBPF for monitoring, 
performance analysis, security, and networking.


# perfMaps:  eBPF and Perf subsystem:

There is a direct and important relationship between the **eBPF `BPF_MAP_TYPE_PERF_EVENT_ARRAY`** and the 
**perf subsystem** in Linux. 

In fact, the primary purpose of this map type is to integrate **eBPF** with the **perf subsystem**, which 
is responsible for performance monitoring and event tracing in Linux.

### Relationship Between `BPF_MAP_TYPE_PERF_EVENT_ARRAY` and the Perf Subsystem

#### 1. Event Delivery to User-Space via `perf` Subsystem

* Perf Subsystem: 
    The Linux **perf** subsystem provides tools for performance monitoring and analysis, enabling users to 
    track hardware events (e.g., CPU cycles, cache misses), software events (e.g., function call durations, 
    system calls), and other system-level events.

* eBPF and Perf Integration:
    The `BPF_MAP_TYPE_PERF_EVENT_ARRAY` is a map type that is designed specifically to work with the perf 
    subsystem. 
    eBPF programs use this map to **send events from the kernel to user-space** in the form of a **perf event**.

  * In other words, whenever an eBPF program collects data (such as performance metrics, tracepoint data, 
    or even stack traces), it can push this data into a **perf event array map**, making the event available
    to tools in user-space (such as `perf`).

#### 2. **How Data Moves from Kernel to User-Space**

* eBPF Program: 
    In the kernel, an eBPF program is attached to a specific kernel event, such as a network packet being 
    received, a system call being invoked, or a tracepoint being triggered. When this event occurs, the 
    eBPF program collects relevant data (e.g., packet statistics, system call information) and then sends 
    the data to a perf event array map.

* Perf Event Array Map:
    The map type (`BPF_MAP_TYPE_PERF_EVENT_ARRAY`) acts as a buffer, where the eBPF program places the 
    collected data. 
    The **perf** subsystem consumes this data by reading from the map and delivering it to user-space.

  * The map allows **real-time event streaming** from the kernel to user-space without the need for 
    explicit polling. Instead, user-space programs can consume the event data as it becomes available.

* **User-Space Consumption**: 
    Tools like `perf` or custom user-space applications can read the data from the perf event map, analyze 
    it, and present it to the user in a meaningful way. 
    The data is typically collected and displayed in real-time, allowing users to monitor system performance 
    dynamically.

#### 3. **Perf Buffer (Ring Buffer) and Event Collection**

* The **perf event system** uses a **ring buffer** to efficiently collect and store performance data in the
  kernel. This is the core mechanism that allows for high-performance event collection with minimal overhead.

* **BPF_MAP_TYPE_PERF_EVENT_ARRAY** works closely with this ring buffer concept. 
  When an eBPF program triggers an event (such as a counter overflow or a function call trace), the program 
  places the event data into this buffer, which is later accessed by user-space tools like `perf`.

  * The ring buffer allows the kernel to efficiently store events while minimizing the risk of data loss 
  during high-frequency events. When the buffer is full, old events may be dropped (or overwritten), 
  ensuring that new events are processed without blocking.

#### 4. **Example Use Case: Performance Profiling**

To demo how `BPF_MAP_TYPE_PERF_EVENT_ARRAY` works with the perf subsystem, here’s a high-level use case:

1. Attach an eBPF Program: 
    You attach an eBPF program to a tracepoint or a performance event. 
    For instance, you might track **CPU cycles**, **cache misses**, or **function execution times**.

2. eBPF Collects Data: 
    Each time the event is triggered (ex: a cache miss or a function call), the eBPF program collects some 
    data (ex: timestamp, counter, stack trace) and sends it to the `BPF_MAP_TYPE_PERF_EVENT_ARRAY` map.

3. Perf Event Array Map: 
    The kernel stores the collected data in the map, which acts as a buffer. 
    This data can be accessed by user-space applications that are subscribed to the perf events.

4. User-Space Tool (e.g., `perf`) Reads Events: 
    A user-space tool, like `perf`, can then read the events from the map, process them, and output them in 
    a human-readable format (e.g., aggregate statistics, stack traces, etc.).

   Example:

   ```bash
   perf record -e cycles -a -- ./my_program
   ```

   In this example, the `perf` tool collects **cycle count events** from all CPUs and stores them in the 
   kernel’s perf event buffer. An eBPF program could also be used to track and push these events into the 
   `BPF_MAP_TYPE_PERF_EVENT_ARRAY`, allowing for more advanced custom tracing or profiling logic.

#### 5. Real-Time Event Streaming

One of the key benefits of using the **perf event array** is the ability to stream events **in real-time** 
from the kernel to user-space. This makes it ideal for:

* **Live performance monitoring**
* **Tracepoint-based logging**
* **Low-latency profiling**
* **Dynamic system analysis** (e.g., monitoring system calls, network traffic, hardware events)

#### 6. `perf` Tool Integration

The **`perf` tool**, which is part of the **perf subsystem**, can be used to interact with the perf event 
arrays created by eBPF programs. For example:

* `perf trace`: Used for tracing function calls and events. It can capture events generated by eBPF programs.
* `perf stat`: Can collect performance statistics based on kernel events, including those collected via eBPF.
* `perf record`: Can be used to record events for later analysis, such as function call samples, which could 
  be coming from an eBPF program pushing data into a perf event array.

#### 7. Additional Features from the Perf Subsystem

* Perf Ring Buffer: 
    The underlying data structure that eBPF uses to store data in the kernel before delivering it to 
    user-space is a ring buffer. 
    The ring buffer ensures that events are efficiently buffered and that no data is lost due to high event 
    rates (up to the buffer size limit).

* Async Data Delivery: 
    The perf subsystem also provides asynchronous data delivery, meaning that user-space can process events 
    at its own pace, without blocking the kernel operations.

---

### Summary

* **Yes**, `BPF_MAP_TYPE_PERF_EVENT_ARRAY` is **directly related** to the **perf subsystem** in Linux. 
  It allows eBPF programs to send events to user-space via the perf event system.

* **Functionality**: It serves as a conduit for real-time event communication from the kernel to user-space, 
  enabling performance monitoring, tracing, and profiling via tools like `perf`.

* **Integration**: The data stored in `BPF_MAP_TYPE_PERF_EVENT_ARRAY` maps can be consumed by user-space 
programs using the **perf** tools or custom applications, providing real-time insights into kernel events 
and system performance.

In essence, **`BPF_MAP_TYPE_PERF_EVENT_ARRAY`** is a key mechanism for leveraging eBPF’s power in performance 
analysis, allowing the kernel to send high-throughput event data to user-space applications in an efficient, 
low-latency manner, integrating deeply with the Linux **perf subsystem**.

# **"Perf Map"** 

**PerfMaps** typically refers to **`BPF_MAP_TYPE_PERF_EVENT_ARRAY`**, which is a type of map in eBPF that 
is specifically designed for event communication between the kernel and user-space applications. 
This map type is one of the more specialized data structures used in eBPF, and it plays a central role in 
event-driven scenarios, particularly in performance monitoring and tracing use cases.

Here’s a breakdown of what it is and how it works:

### **BPF_MAP_TYPE_PERF_EVENT_ARRAY**

* **Purpose**:
  `BPF_MAP_TYPE_PERF_EVENT_ARRAY` is used to pass events from eBPF programs running in the kernel to 
  user-space programs. It is commonly used for tracing and performance monitoring, where an eBPF program 
  collects data (e.g., performance counters, stack traces, or events) and then sends these events to 
  user-space via the perf subsystem. These events can be read by user-space tools (like `perf` or other 
  monitoring applications) in real-time.

* **Mechanism**:
  The kernel generates events and stores them in this map. Once the eBPF program triggers an event, the 
  data (such as a sample or counter) is pushed into the map, and the user-space program can poll this map 
  to receive the event. This is useful for real-time data streaming and logging.

### **How It Works**

1. **Map Creation**:
   A `BPF_MAP_TYPE_PERF_EVENT_ARRAY` map is created using the `bpf()` syscall with the map type 
   `BPF_MAP_TYPE_PERF_EVENT_ARRAY`. It typically associates each entry in the map with a **perf event** 
   that can be read from user-space.

   Example:

   ```c
   struct bpf_map *map = bpf_map_create(BPF_MAP_TYPE_PERF_EVENT_ARRAY, key_size, value_size, max_entries);
   ```

2. **Event Insertion**:
   In the kernel, eBPF programs push events into this map using the `bpf_perf_event_output()` helper 
   function. This function allows the program to send event data (such as performance counters or other 
   metrics) to the user-space application.

   Example in a BPF program:

   ```c
   bpf_perf_event_output(ctx, map, 0, &data, sizeof(data));
   ```

3. **Event Consumption by User-Space**:
   Once the event is written to the map, user-space applications can use the `perf` tools or any other 
   custom user-space program that interfaces with the perf subsystem to consume and process the events. 
   The user-space application can access the data from the map as it becomes available.

   User-space tools like `perf` often interact with `perf_event_array` maps to gather trace data (e.g.,
   performance samples, function call traces, etc.) in real-time.

4. **Key Aspects of Perf Event Maps**:

   * **Real-time event streaming**: These maps provide an efficient way to stream events from the kernel
   to user-space without requiring continuous polling or manual fetching of data.
   * **Buffered data**: The data is stored in the perf ring buffer, which provides high-performance, 
   low-latency event collection.
   * **Integration with `perf` tools**: Many of the native tools that interact with eBPF (e.g., `perf`,
   `bcc`, `bpftrace`) leverage `perf_event_array` maps to collect and report kernel-level event data.

### **Example Use Case: Performance Monitoring**

A common use case for `BPF_MAP_TYPE_PERF_EVENT_ARRAY` is in performance monitoring. For example, you might
want to trace the number of cache misses or function calls over time. You can attach an eBPF program to a
relevant tracepoint (e.g., cache miss event), and each time the event is triggered, the program pushes the
data (e.g., a counter or timestamp) into the perf event array. A user-space application can then read this
data to monitor the system's performance.

Here's a simple high-level flow:

1. **Attach an eBPF Program**: An eBPF program is attached to a kernel tracepoint (such as a network event 
or a hardware counter).

2. **Trigger Event**: When the relevant event occurs (e.g., a packet is received or a cache miss happens),
the eBPF program collects some data and calls `bpf_perf_event_output()` to send the event data to the
perf event map.

3. **User-Space Consumption**: A user-space application or tool reads the event data from the perf event 
array and processes it (e.g., prints it, aggregates it, logs it, or triggers further actions).

### **When to Use `BPF_MAP_TYPE_PERF_EVENT_ARRAY`**

* **Tracing and Profiling**: This is especially useful for tracing kernel events or monitoring system 
performance. You can capture things like function call durations, hardware performance counters, or even
detailed event traces.

* **Real-Time Data Streaming**: It's ideal for real-time event collection where you want user-space
applications to respond immediately to kernel-level events.

* **Sampling Events**: If you're interested in sampling metrics (e.g., stack traces) at specific kernel
locations, you would use this map type to store and push sampled events.

### **Key Benefits**

* **Low Overhead**: Because it uses the perf subsystem, which is optimized for high-throughput event 
collection, this map type can efficiently deliver events to user-space with minimal overhead.
* **Ease of Integration**: Tools like `perf` and `bcc` are already designed to consume data from
`BPF_MAP_TYPE_PERF_EVENT_ARRAY` maps, so they integrate easily into monitoring setups.
* **Real-Time Communication**: Provides a simple and efficient mechanism for real-time communication
between the kernel and user-space for event-driven tasks.

### **Limitations**

* **Buffered Size**: Like any ring buffer, the perf event array has a finite size. If the buffer is full 
(due to high-frequency events), new events may be lost. You need to carefully consider buffer size and 
event rates in high-performance applications.
* **Not for Random Access**: This map type is designed for event-driven communication, not for 
general-purpose key-value lookups. It's not suitable for storing and accessing arbitrary data in a direct manner.

### **Conclusion**

In summary, **Perf Map**, or `BPF_MAP_TYPE_PERF_EVENT_ARRAY`, is an essential data structure used for 
event-driven communication between eBPF programs in the kernel and user-space applications.
It is widely used in performance monitoring, tracing, and real-time event collection scenarios, 
providing efficient, low-latency delivery of event data to user-space consumers.


# Realtime Tracing and monitoring:

**`perf event maps` (specifically `BPF_MAP_TYPE_PERF_EVENT_ARRAY`)** are **perfect** for 
**real-time tracing** and **monitoring**, especially in combination with **web services** or other 
external systems. 
The ability to hook eBPF-driven **perf events** into web services is a powerful use case, and with **Rust**, 
**Aya**, and async frameworks like **Tokio** and web service crates (such as **hyper** or **reqwest**), 
you can easily build a real-time monitoring system that integrates eBPF, tracing, and external services.

### Why Use `perf maps` for Real-Time Tracing?

* **Real-Time Data Streaming**: 
    The **perf event map** is designed to send data from the kernel to user-space in real-time, making it
    perfect for **live reporting** and **streaming data**.

* **Low Latency**: eBPF programs are executed in kernel space, and the **perf event array** efficiently
queues data into the kernel’s ring buffer, minimizing latency and overhead.

* **Easy Integration**: Since **Aya** allows you to manage these events in user-space with **Rust**,
you can easily pipe this data to various **web services** or **HTTP APIs** in real-time.

### Building Real-Time Trace Reporting with Rust + Aya

Let’s break down how you can build a **real-time trace reporting system** that sends **eBPF trace data**,
(from **perf maps**) to a web service (e.g., a REST API) in real-time.


Here’s a conceptual flow:

1. **eBPF Program**: Trace a kernel event (e.g., system calls, network packets, custom tracepoints).
2. **Perf Event Map**: The eBPF program pushes the trace data into a `perf event array`, which is 
accessible in user-space.
3. **Rust with Aya**: Use **Aya** to load the eBPF program, handle the events asynchronously, and send
those events to external web services.
4. **Web Service Integration**: Use web service crates (e.g., **hyper**, **reqwest**) to send the trace
data to a web service or API for monitoring, storage, or visualization.

### Example Workflow: Send Real-Time Trace Data to a Web Service

1. **Create and Load the eBPF Program** (Rust + Aya)

   * You write an eBPF program that collects data, e.g., tracepoints, system calls, or custom metrics.
   * The eBPF program sends data to a **perf event array map**.

2. **Consume Events in Rust (Aya)**

   * Use Aya’s `PerfEventArray` to asynchronously poll for events from the perf map.
   * Each event could be a message, log, or metric that you want to send to a web service.

3. **Send Data to a Web Service**

   * Each time an event is captured by Aya, you use an **async HTTP client** like **`reqwest`** to send
   this data to a web service (e.g., a monitoring API or data collection system).

---

### Full Example: Real-Time Trace Data to a Web Service (Rust + Aya + Reqwest)

Let’s walk through an example where we capture system call events (e.g., `execve`) and send them to a web 
service in real-time.

#### 1. **Create the eBPF Program (in C)**

This is the same as earlier. It will trace the `execve` system call and send the data to a **perf event map**.

```c
#include <linux/ptrace.h>
#include <linux/sched.h>

SEC("tracepoint/syscalls/sys_enter_execve")
int trace_execve(struct trace_event_raw_sys_enter *ctx) {
    char message[] = "Execve syscall triggered\n";
    bpf_perf_event_output(ctx, perf_map, BPF_F_CURRENT_CPU, message, sizeof(message));
    return 0;
}

char _license[] SEC("license") = "GPL";
```

#### 2. **Load and Handle eBPF Events in Rust**

In the Rust program, we load the eBPF program, attach it to the `sys_enter_execve` tracepoint, and then
read events from the **perf event map**. Each time an event is received, it’s sent to a web service.

```rust
use aya::maps::{Map, PerfEventArray};
use aya::programs::{TracePoint, Program};
use aya::Bpf;
use reqwest::Client;
use tokio::runtime::Runtime;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Step 1: Load the eBPF program using Aya
    let mut bpf = Bpf::load_file("trace_execve.bpf.o")?;

    // Step 2: Get the perf event map and program from BPF object
    let map = bpf.map_mut::<PerfEventArray>("perf_map")?;
    let program = bpf.program_mut::<TracePoint>("trace_execve")?;

    // Step 3: Attach the eBPF program to the tracepoint (sys_enter_execve)
    program.attach("syscalls:sys_enter_execve")?;

    // Step 4: Create a reqwest client for sending events to the web service
    let client = Client::new();

    // Step 5: Create an async task to read events and send them to a web service
    let rt = Runtime::new()?;

    rt.spawn(async move {
        loop {
            match map.poll(&online_cpus().await) {
                Ok(Some(event)) => {
                    // Step 6: Send event to a web service (e.g., REST API)
                    let event_str = String::from_utf8_lossy(&event);
                    println!("Received event: {}", event_str);

                    // Send the event to the web service via POST request
                    let response = client
                        .post("https://example.com/api/trace_event")
                        .json(&serde_json::json!({ "event": event_str.to_string() }))
                        .send()
                        .await;

                    match response {
                        Ok(res) => println!("Event sent to web service: {:?}", res),
                        Err(err) => eprintln!("Failed to send event: {:?}", err),
                    }
                }
                Ok(None) => {
                    // No event available, continue polling
                    tokio::time::sleep(Duration::from_millis(100)).await;
                }
                Err(err) => {
                    eprintln!("Error reading event: {}", err);
                    break;
                }
            }
        }
    })
    .await?;

    Ok(())
}
```

### Key Components:

1. **Aya eBPF Program**: The `trace_execve.bpf.o` eBPF program is loaded into the kernel and attached to 
the `sys_enter_execve` tracepoint. It sends event data to a **perf event array**.

2. **`PerfEventArray` Map**: Aya’s `PerfEventArray` is used to read events from the perf ring buffer.
These events are the data generated by the eBPF program.

3. **Web Service Integration**: When an event is received, the Rust program uses **`reqwest`** 
(an async HTTP client) to send the event data to a web service endpoint (in this case, a JSON POST request).

4. **Async Handling with Tokio**: The program uses **Tokio** for asynchronous event handling, allowing 
non-blocking calls to send events to the web service without interrupting the event polling loop.

---

### Benefits of This Approach:

1. **Real-Time Monitoring**: You can send eBPF events to a web service **immediately** as they happen, 
providing **real-time** monitoring and reporting.

2. **Scalability**: Since you're using asynchronous Rust, the system can handle **high-frequency events** 
efficiently without blocking, making it suitable for **high-throughput tracing** in production systems.

3. **Flexibility**: You can integrate the trace events with any web service, whether it's a custom 
monitoring system, logging service, or visualization dashboard.

4. **Low Overhead**: eBPF runs inside the kernel with **minimal performance impact**, and using 
**async Rust** ensures that you don’t block or slow down the system with network operations.

---

### Possible Web Service Use Cases:

* **Log Aggregation**: Send trace events to a central logging system (e.g., **Elasticsearch**,
**Logstash**, **Fluentd**).
* **Real-Time Metrics Collection**: Send trace data to a real-time metrics service (e.g., **Prometheus**,
**Datadog**, **New Relic**).
* **Alerting**: Use the trace events to trigger **alerts** based on certain conditions
(e.g., high system call frequency).
* **Data Analytics**: Send trace data to an analytics platform for further analysis and reporting.

---

### Conclusion

Yes, using **Aya** in **Rust** to capture eBPF trace events and forward them to web services in 
**real-time** is a fantastic approach! You can easily build **real-time tracing and monitoring systems**
by integrating **perf maps** with **web services**. With the power of **Rust**, **Aya**, and async libraries
like **Tokio**, you can create highly efficient, non-blocking, and scalable systems for monitoring and
responding to kernel events.

