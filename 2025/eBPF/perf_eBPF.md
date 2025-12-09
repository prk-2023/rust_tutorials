# Perf : Kernel Performance sub-system:

- **perf subsystem** or **perf_events** or **Performance Counters for Linux (PCL)**, is a powerful framework
  integrated directly into the kernel that provides a mechanism for performance monitoring and analysis. 
  It abstracts various low-level HW and SW performance monitoring features, presenting a unified interface 
  for tools like the user-space utility `perf`.

- Its primary goal is to provide a low-overhead way to profile the entire system, covering both **kernel** 
  and **user-space** code, to identify performance bottlenecks and measure event statistics.

---
## Core Components and Architecture

The perf subsystem is built around the concept of **events**, which are occurrences that can be counted or 
sampled.

### 1. `perf_event_open()` System Call

The entire `perf` subsystem's interaction with user-space is handled primarily through a single 
system call: `sys_perf_event_open()`. 

This system call creates an event monitoring descriptor, which is essentially a file descriptor that allows 
a process to control and access performance data.

### 2. Events and Sources
The strength of the perf subsystem lies in its ability to monitor a wide variety of performance-related 
events from different sources:

* **Hardware Performance Counters (HPC) Events:** These are **CPU-specific** events provided by the 
  processor's **Performance Monitoring Unit (PMU)**. They count micro-architectural events like:
    * `cycles` (processor clock cycles)
    * `instructions` retired
    * `cache-references` (cache accesses)
    * `cache-misses` (data not found in cache)
    * `branch-misses` (mispredicted branches)
    

* **Software Events:** These are low-level events maintained by the kernel and are not tied to the 
  physical hardware. Examples include:
    * `context-switches` (process/thread changes)
    * `cpu-migrations` (task moving between CPUs)
    * `minor-faults` (memory page faults)

* **Kernel Tracepoint Events:** These are static instrumentation points explicitly placed in the 
  kernel source code by developers at logically interesting locations. They are used to trace specific 
  kernel activities, such as:
    * `sched:sched_switch` (scheduler events)
    * `ext4:ext4_da_write_begin` (filesystem operations)
    * `syscalls:sys_enter_openat` (system call entry/exit)

* **Dynamic Tracing:**
    * **KProbes:** Allows dynamic instrumentation of almost any instruction in the **kernel** without 
      modifying and recompiling the kernel source.
    * **UProbes:** Allows dynamic instrumentation of user-space programs, which is crucial for profiling 
      dynamically linked libraries or application functions.

### 3. Monitoring Modes (Counting vs. Sampling)

The perf subsystem supports two main ways of gathering data:

| Mode | Description | Overhead | Primary Use |
| :--- | :--- | :--- | :--- |
| **Counting** (`perf stat`) | Events are simply counted over a period. The kernel maintains an aggregate counter. | Very Low | Measuring total counts for a benchmark or quick system overview. |
| **Sampling** (`perf record`) | When the event count reaches a threshold (period), the kernel generates an interrupt. The interrupt handler collects data (like the program counter, stack trace, etc.) and writes it to a **mapped memory buffer**. | Low to Medium | Identifying **hot spots** (the exact functions/lines of code) that consume the most resources. |

---

## 💻 User-Space Tool: `perf`

The command-line tool `perf` (located in `tools/perf` in the kernel source) is the primary front-end for the 
kernel's `perf_events` interface. It provides various subcommands to interpret and display the collected data:

* **`perf stat`**: 

  Runs a program or monitors the system-wide activity and reports aggregate counts of specified events.

    *Example: `perf stat -e cycles,instructions ./my_program`*

* **`perf record`**: 

  Runs a command or monitors the system, collecting samples into a file named `perf.data`. 
  This is the core sampling utility.

    *Example: `perf record -g ./my_program` (records samples with call graphs)*

* **`perf report`**: 

    Reads the `perf.data` file and generates a summary report, often showing a list of functions and the 
    percentage of samples taken within them, helping to pinpoint bottlenecks.

* **`perf top`**: 

    Provides a dynamic, top-like view of the hottest funcs currently executing on the system in real-time.

* **`perf list`**: 

    Displays all supported events (hardware, software, tracepoints) available on the current system.

## Interface and Data Flow

1. **Event Setup:**
    The `perf` tool (or any application) uses the `perf_event_open()` system call to ask the kernel to 
    monitor a specific event for a process, CPU, or the entire system.

2.  **Data Collection:**
    * In **Counting** mode, the kernel maintains the counter internally.
    * In **Sampling** mode, the kernel, upon a sampling interrupt, writes the collected data 
      (ex: instruction pointer, stack trace) into a **ring buffer** in kernel memory.

3.  **User-Kernel Communication:** 
    The ring buffer is typically accessed by the user-space tool through a **memory-mapped region** (mmap). 
    This allows the `perf` tool to asynchronously read the raw sampling data from the kernel buffer with 
    extremely low overhead, avoiding frequent system calls.

4.  **Reporting:** 
    The `perf` tool processes the raw data, resolves function names and source lines (using debugging 
    information and symbol tables), and presents the final, human-readable report.

The minimal interface (one syscall, one mapped memory region) is a key design feature that ensures 
**low overhead** and high efficiency, making `perf` suitable for profiling production systems.



# Perf in points and details:

## Key Concepts:

1. **Performance Monitoring Unit (PMU)**:

   * The PMU is hardware-specific and provides the capability to count low-level micro-architectural events.
     These are the events like CPU cycles, cache hits and misses, and branch mis-predictions that directly 
     reflect the underlying hardware behavior. These events can be very useful for identifying performance 
     bottlenecks at the hardware level.

2. **System Call: `sys_perf_event_open()`**:

   * This system call is the gateway for user-space tools like `perf` to interact with the kernel's 
     performance monitoring subsystem. It opens a new performance monitoring event, which returns a file 
     descriptor that is then used to gather data or control event tracking.

   * For instance, if you want to track CPU cache misses, you would use `sys_perf_event_open()` to create 
     an event for cache misses, and then you could interact with that event to retrieve or control data.

3. **Types of Events**:

   * **Hardware Events (HPC)**:
     These are typically tied to specific CPU architecture features and can vary from one CPU model to 
     another. 
     For example, if you're using an Intel processor, you may get access to events like 
        `cpu-cycles`, 
        `instructions`, 
        `L1-dcache-loads`, 
    and others.

   * **Software Events**:
     These events are not tied to specific hardware but reflect system-level activities. 
     Ex include `page-faults`, `context-switches`, `cpu-migrations`, and other high-level OS-level events.

   * **Tracepoint Events**:
     These allow the user to trace specific parts of kernel code, giving insight into system operations. 
     Tracepoints are often used for debugging and performance analysis.

   * **Dynamic Tracing (KProbes and UProbes)**:
     KProbes allow for kernel-level dynamic tracing, while UProbes allow users to trace user-space apps. 
     These tracing mechanisms allow you to hook into code and gather data without needing to modify the 
     program source.

4. **Modes of Data Collection**:

   * **Counting**:
     This is where `perf stat` comes into play. It simply counts occurrences of a particular event over a 
     specified period. It's great for quick benchmarking and getting a high-level overview of performance.

   * **Sampling**:
     The `perf record` mode takes periodic samples of specific events, such as the program counter (PC), 
     call stacks, and more. Sampling is more detailed than counting and is useful for performance profiling,
     especially when combined with tools like `perf report`, which help analyze the collected samples.

### Example Workflows:

* **Using `perf stat`**: 
    This command provides aggregate statistics for a set of events over a time period. 
    For example, to measure CPU cycles, instructions, and cache-misses while running a program, 
    you would use:

    ```bash
    $ perf stat -e cycles,instructions,cache-misses ./your_program
    ```

* **Using `perf record`**: 
    This is more detailed, and it collects stack traces when the specified events exceed a given threshold 
    (like 1000 cycles). 
    For example:

    ```bash
    perf record -e cycles -c 1000 ./your_program
    ```

    This will create a `.perf.data` file which can be analyzed with `perf report`.

* **Using `perf top`**: This command provides a real-time sampling view of the performance of the running 
  system, similar to `top`, but specifically for performance-related events. 
  It’s very useful for identifying bottlenecks on the fly.

### Data Reporting:

Once data is collected, the `perf` tool processes and presents it in human-readable formats. 
Common reporting tools include:

* **`perf report`**: 
    Provides a report of the collected sample data, typically displaying functions that were sampled the 
    most, along with their stack traces.

* **`perf top`**: 
    Provides a real-time, top-like interface for displaying the most frequent events in the system.

### Example Scenario: Profiling a Program

1. **Start with counting mode**: 
    Use `perf stat` to get a quick overview of CPU usage and bottlenecks in your program:

   ```bash
   perf stat ./your_program
   ```

   This gives you a quick count of CPU cycles, instructions executed, cache misses, and other events.

2. **Switch to sampling mode**: 
    For more detailed analysis, switch to `perf record` to sample the program's execution:

   ```bash
   perf record -e cpu-clock -c 1000 ./your_program
   ```

   This will collect samples every 1000 CPU cycles.

3. **Analyze the results**: 
   After running `perf record`, use `perf report` to analyze where the program spends most of its time:

   ```bash
   perf report
   ```

   This shows you a breakdown of the functions and their corresponding time spent in the application, 
   helping you identify hotspots.

### Why `perf` is useful:

* **Low overhead**: 
    One of the key benefits of `perf` is its low overhead, especially when using hardware counters. 
    The system can profile the entire system with minimal performance impact.

* **Flexibility**: 
    You can profile specific applications, specific CPU cores, or the entire system.

* **Granularity**: 
    `perf` allows you to choose between high-level statistics (counting) and low-level profiling (sampling), 
    depending on the detail you need.

If you're just getting into profiling with `perf`, the most straightforward way to start is by running 
`perf stat` on a program to get a quick overview of performance. 
From there, you can move to more advanced features like `perf record` and `perf report` for deeper insights.

# perf Data sharing mechanism:


To know in depth how perf data sharing mechanism exists first we look at eBPF method:

The data sharing mechanisms between **eBPF** and **user-space** (or even between **eBPF programs** within 
the kernel) rely on **maps**. Maps are key-value stores in the eBPF framework that enable efficient data 
sharing and communication. 

When it comes to **`perf`**, the data sharing mechanism is different but serves a similar purpose of 
transferring performance data between the kernel and user-space.

Let's break down how each system works.

### eBPF: Maps for Data Sharing

In eBPF, **maps** are used to share data between:

* **User-space and eBPF programs**: 
    Maps allow user-space programs (like `bpftrace`, `bcc`, or custom BPF programs) to read and write data 
    from eBPF programs running in the kernel.

* **Different eBPF programs**: 
    Maps provide a mechanism for eBPF programs running at different points in the kernel to share data, 
    enabling more complex logic or aggregating data across various parts of the kernel.

    Maps can hold different types of data structures, like integers, arrays, hash maps, and more. 
    The kernel and user-space programs can read and modify these maps, providing a low-latency, efficient 
    way to exchange information.

#### Example Use of eBPF Maps:

1. **Storing Counters in a Map**: 
    You can store counters in a map to keep track of how many times a certain event has occurred (e.g., the 
    number of times a system call is invoked). 
    These counters can be read by user-space programs that periodically pull data from the map to generate 
    reports or statistics.

   ```c
   // BPF program: Increment counter in a map
   bpf_map_update_elem(&my_map, &key, &value, BPF_ANY);
   ```

2. **Sharing Data Between Kernel and User-Space**: 
    A typical example is using **`perf_event`** (e.g., with `bpf_perf_event_output`) to send data from an 
    eBPF program to a user-space application. 
    This can be done by writing the data to a special map type called `perf_event_array`.

   ```c
   // Example: eBPF program writes data to user-space via perf_event
   bpf_perf_event_output(ctx, &perf_event_map, BPF_F_CURRENT_CPU, data, sizeof(data));
   ```

   In this case, data collected in the kernel (like a sample or an event) can be passed to user-space 
   applications using **maps**.

---

### `perf`: Data Sharing Mechanism

For **`perf`**, the data sharing mechanism is different. 
The **`perf`** tool primarily uses **ring buffers** to transfer performance data from the kernel to
user-space, and this is done asynchronously. 

The kernel writes performance data into the ring buffer, and the user-space `perf` utility reads this data, 
processes it, and displays it.

#### Key Concepts:

1. **Ring Buffers**:

   * The **ring buffer** is a circular buffer used for storing performance data. 
   It allows the kernel to collect performance samples (e.g., CPU cycles, cache misses, etc.) and send them
   to user-space efficiently with minimal overhead.

   * When you run `perf record`, it sets up a **ring buffer** in kernel memory. 
   As events are sampled (e.g., when a threshold like 1000 cycles is crossed), the kernel places the data 
   into this buffer.

   * The **user-space tool (`perf`)** reads this data from the ring buffer asynchronously.
   The data is typically read in chunks and processed by tools like `perf report` or `perf top`.

2. **`perf_event` API**:

   * **`sys_perf_event_open()`** :

   Is the system call used to set up events in `perf`. 
   This call creates a file descriptor that represents an event, and it specifies how data is captured 
   (either by counting or sampling).

   * Once an event is set up, the kernel starts populating the ring buffer with event data (ex: PC,stack
   trace, CPU cycles, etc.).

   * User-space can then read this data by interacting with the event descriptor (via file operations like 
   `read()`) or by mapping the ring buffer into the user-space process memory.

#### Example Flow for `perf` Data Collection:

1. **Setting up the event**:
   You initiate the event using the `sys_perf_event_open()` system call, specifying the event to be 
   measured (e.g., CPU cycles, cache misses, etc.).

   ```bash
   perf record -e cycles -c 1000 ./my_program
   ```

2. **Kernel Collects Data**:
   The kernel collects the data and places it into a **ring buffer** in kernel space.

3. **User-Space Reads the Data**:
   The user-space `perf` tool reads data from the ring buffer and processes it to display to the user.

4. **Reporting**:
   After collecting data, `perf report` is used to analyze the performance data, ex: stack traces, 
   function call counts, etc.

---

### Comparing eBPF Maps and `perf` Ring Buffers

| **Feature**             | **eBPF Maps**                                     | **`perf` Ring Buffers**                                               |
| ----------------------- | ------------------------------------------------- | --------------------------------------------------------------------- |
| **Purpose**             | Share data between eBPF programs and user-space   | Share performance data between kernel and user-space                  |
| **Data Structure**      | Key-value store (e.g., hash maps, arrays, etc.)   | Circular buffer storing performance event data                        |
| **Data Access**         | Direct read/write by eBPF programs and user-space | User-space reads data asynchronously from kernel buffer               |
| **Flexibility**         | Very flexible, allows custom data types and logic | More rigid, designed for system performance events                    |
| **Common Use Case**     | Kernel networking, tracing, debugging, security   | Profiling CPU performance, system events, function calls              |
| **Asynchronous Access** | Yes, but with more manual intervention for users  | Yes, data is pulled asynchronously from the kernel buffer             |
| **Data Latency**        | Low, can be real-time for small events            | Low latency, but there may be more overhead due to sampling frequency |
| **User-Space Tool**     | `bpftrace`, `bcc`, custom programs                | `perf stat`, `perf record`, `perf report`, etc.                       |

### Key Differences in Data Sharing

* **eBPF**'s **maps** are much more flexible in terms of data types and usage. 
They are used for general-purpose data exchange between eBPF programs and user-space. 
You can use maps for a wide range of scenarios, such as tracking event counts, aggregating data, or even 
sending custom logs from the kernel to user-space.

* **`perf`**'s **ring buffers** are specifically designed for efficient, low-latency collection of 
performance data. They are optimized for event sampling and passing performance metrics between the kernel 
and user-space in a way that minimizes overhead.

---

### Use Cases and When to Choose Which

* **Use eBPF** if:

  * You need highly customizable tracing and performance monitoring.
  * You need real-time data processing or want to track non-standard or custom events.
  * You're working with kernel-level tracing that isn't directly supported by `perf`, or if you want to 
  perform more complex logic within the kernel (e.g., modifying data or making decisions based on specific 
  conditions).

* **Use `perf`** if:

  * You need a straightforward and easy-to-use tool for performance profiling.
  * You're focused on monitoring standard hardware performance counters or well-known events like 
    cache misses, CPU cycles, context switches, etc.
  * You want a quick overview of system performance or need detailed stack traces for profiling.

---

### Conclusion

While **eBPF** uses **maps** for efficient data sharing between kernel programs and user-space, **`perf`** 
relies on **ring buffers** to asynchronously share event data with user-space tools. 
Both mechanisms are designed to provide low-overhead access to performance data, but eBPF offers more 
flexibility and customizability, while `perf` is designed for straightforward and efficient system-level 
performance monitoring.


# eBPF <=> perf:  (Sharing Components)

*Perf and eBPF share key components, creating a symbiotic relationship* where eBPF can both leverage the 
data sources and the efficient data transfer mechanism established by the perf subsystem.

=> Data source 
=> Data transfer mechanism 

---

## Shared Components and Synergy

The relationship between `eBPF` and the `perf` subsystem centers around two main areas: 
    - **Event Hooks (Data Source)** and 
    - **Ring Buffers (Data Transport)**.

### 1. The Perf Subsystem as an eBPF Hook

The first shared component is the source of the events themselves. 
eBPF is event-driven, and one of the most powerful sets of events it can attach to is provided by the 
**perf subsystem**.

* **Perf Events Hook:** 
    `eBPF` programs can be of type `BPF_PROG_TYPE_PERF_EVENT`. 
    This allows you to attach a custom eBPF program to any performance event that the `perf` tool itself can 
    monitor:
    - *Hardware Performance Counters (HPCs):* 
        eBPF can execute logic when a specific hardware event threshold is reached, such as every 10,000 
        **CPU cycles**, **cache misses**, or **instruction retires**. 
        This effectively allows you to replace the built-in `perf` sampling mechanism with your own highly 
        optimized, in-kernel filtering and aggregation logic.

    * *Software Events:*
        You can attach eBPF programs to events like **context switches** or **CPU migrations** to analyze 
        or modify the behavior immediately upon the event's occurrence.

    * **Kernel Tracepoints/Probes:** 
        eBPF can also attach to the same **Tracepoints, Kprobes, and Uprobes** that the `perf` tool uses for
        dynamic and static tracing.

In this context, the entire **perf_events framework acts as an entry point (a hook)** for an eBPF program, 
letting the BPF bytecode run in response to events that the PMU or kernel software counter registers.

### 2. The Perf Ring Buffer for eBPF Data Export

The second, and arguably most important, shared component is the mechanism for efficiently exporting data 
from the kernel to user-space.

* **The Original Shared Component: `BPF_MAP_TYPE_PERF_EVENT_ARRAY`**

    * The `perf` tool uses a **per-CPU ring buffer** structure (implemented using memory-mapped pages) to 
      stream performance samples from the kernel to user-space with minimal overhead (avoiding many system 
      calls and data copies).

    * eBPF introduced a special map type, **`BPF_MAP_TYPE_PERF_EVENT_ARRAY`** (often called the 
      *perf buffer* or *perf event array*), which **re-uses this exact same ring buffer mechanism**.

    * eBPF programs use a helper function, `bpf_perf_event_output()`, to write custom event data to these
      buffers. 
      The user-space program then reads from the memory-mapped buffer just as the `perf` tool does.

    * This allows eBPF to *piggyback* on the existing, highly optimized, and mature **perf subsystem's** 
      data transfer infrastructure.

> **Key takeaway:** 
>   The memory-mapped ring buffer technology for high-speed kernel $\rightarrow$ user-space communication, 
>   originally perfected by the **perf** subsystem, was adopted as a primary event streaming mechanism for eBPF. 

## Data Sharing Mechanisms Compared

While eBPF uses the perf ring buffer for **streaming events/samples** to user-space, it has its own unique 
data structure, the **BPF Map**, for sharing *state* and *aggregated statistics*.

| Mechanism | Primary Purpose | Scope | Key Feature | Owner/Origin |
| :--- | :--- | :--- | :--- | :--- |
| **BPF Maps (e.g., Hash Tables)** | Store **state, configuration, and aggregated metrics** (e.g., histograms, counters). | Shared between eBPF programs and user-space. | Key-value store for *summary* data. **Minimal data is exported.** | **eBPF** |
| **BPF Ring Buffer** (`BPF_MAP_TYPE_RINGBUF` - Modern) | **Streaming raw events** (traces, logs, samples) to user-space efficiently. | Multiple Producers (CPUs), Single Consumer (User-space). | Single, multi-producer queue that preserves **global event ordering**. | **eBPF** (Newer, optimized Map type) |
| **Perf Event Array** (`BPF_MAP_TYPE_PERF_EVENT_ARRAY` - Legacy/Original) | **Streaming raw events** to user-space. | **Per-CPU** ring buffers. | Simple, proven ring buffer mechanism. **Does not preserve global ordering**. | **Perf Subsystem** (mechanism reused by eBPF) |

### The Evolution to the Pure BPF Ring Buffer

The reliance on `BPF_MAP_TYPE_PERF_EVENT_ARRAY` exposed two drawbacks:
1.  It was **per-CPU**, which led to inefficient memory use and made it hard to guarantee 
    **global event ordering** (events on CPU 0 might be read *after* events on CPU 1, even if they occurred earlier).

2.  It required an extra memory copy in the BPF program before writing to the buffer.

To address this, modern kernels (Linux 5.8+) introduced the **`BPF_MAP_TYPE_RINGBUF`**. 

This is a dedicated, optimized eBPF Map type that is:

    * A **single, multi-producer queue**, which simplifies memory management and preserves global ordering.

    * More efficient by using a `reserve/submit` API (`bpf_ringbuf_reserve` and `bpf_ringbuf_submit`), 
      allowing the BPF program to write the event data **directly** into the ring buffer memory without an 
      intermediate copy.

The new **BPF Ring Buffer** is essentially the next-generation, optimized replacement for the 
*Perf Event Array* for event streaming, completing eBPF's control over its own data transport mechanisms.


## BPF and Perf: The Interconnection Points

The **`BPF_MAP_TYPE_PERF_EVENT_ARRAY`** is a central point of synergy. 
However, its purpose is to enable an **eBPF-aware user-space application** to read the data, not typically 
the standard `perf` tool itself.

### 1. `BPF_MAP_TYPE_PERF_EVENT_ARRAY` and Data Flow

| Point | Your Statement | Refinement/Correction |
| :--- | :--- | :--- |
| **Main Connection Feature** | **`BPF_MAP_TYPE_PERF_EVENT_ARRAY`** is the main connector. | **Mostly True.** This map type *leverages* the perf ring buffer *infrastructure* for efficient data streaming, making it the primary *data export* connector. |
| **Data Sent to Perf Buffer** | This allows eBPF programs to send custom performance event data into the **perf\_event buffer**. | **True.** The eBPF program uses the `bpf_perf_event_output()` helper to write data to the per-CPU buffers established via this map type. |
| **User-Space Reader** | ...which can be read by the user space **"perf" tool**. | **False.** The standard `perf` tool (i.e., the `perf record` / `perf report` utility) is generally **not** configured to read and process these custom eBPF events. The data is read by **eBPF user-space tools** like BCC or `libbpf` applications. |

---

## The Actual Data Flow

The synergy between eBPF and perf involves two distinct paths:

### Path A: Perf Subsystem as an eBPF Hook (Source)

* **Relationship:** 
    eBPF uses the perf subsystem as a **data source**.

* **Mechanism:** 
    An eBPF program (e.g., of type `BPF_PROG_TYPE_PERF_EVENT`) is attached to a low-level event managed by 
    the perf subsystem (e.g., `cache-misses`, a `kprobe`, or a kernel tracepoint).

* **Result:** 
    The eBPF program runs when that specific perf event occurs. 
    It can then perform aggregation and filtering **in the kernel**.

### Path B: Perf Ring Buffer as eBPF Data Transport (Sink)

* **Mechanism:** 
    The **`BPF_MAP_TYPE_PERF_EVENT_ARRAY`** map creates and manages the memory-mapped, per-CPU ring buffers 
    that the perf subsystem pioneered.

* **eBPF Program Action:** 
    The eBPF program uses the `bpf_perf_event_output()` helper to write arbitrary custom data 
    (ex: a process ID, a latency value, and a function name) to this buffer. 
    This data is **not** in the standard `perf.data` format.

* **User-Space Reader:** 
    A dedicated **eBPF user-space application** (written using Aya, BCC, `libbpf`, or `bpftrace`) **mmap**s 
    these buffers and reads the custom data. It then processes, formats, and prints the result.

> **Example:** A `bpftrace` script attaches to a syscall and outputs the syscall name and duration. 
> The standard `perf` tool cannot read or interpret this custom data; only the `bpftrace` runtime client can.

##  Modern Improvement: `BPF_MAP_TYPE_RINGBUF`

While **`BPF_MAP_TYPE_PERF_EVENT_ARRAY`** was the original link for data streaming, modern eBPF tools 
increasingly favor the newer **`BPF_MAP_TYPE_RINGBUF`** (introduced in Linux 5.8) for event streaming.

The newer ring buffer is a native eBPF map that is **globally ordered** and has a more efficient data 
submission API, making it superior for event streaming and reducing the need to rely on the 
older, per-CPU, perf-derived structure.

## Realtime monitoring applications :

The BPF_MAP_TYPE_RINGBUF is a significantly better choice than BPF_MAP_TYPE_PERF_EVENT_ARRAY for building 
a real-time streamer application over HTTP, especially one leveraging a modern user-space environment 
like Rust with the Aya crate.

`BPF_MAP_TYPE_RINGBUF` resolves critical limitations of the older `PERF_EVENT_ARRAY` that are essential for 
reliable, high-throughput, real-time data streaming.

## Key Advantages of `BPF_MAP_TYPE_RINGBUF` for Real-Time Streaming

The advantages of the new ring buffer map type (Linux 5.8+) center on improved efficiency, memory use, and, 
most importantly for a streamer, **event ordering**.

### 1. Guaranteed Global Event Ordering (Crucial for Streamers)

| Feature | `BPF_MAP_TYPE_PERF_EVENT_ARRAY` | `BPF_MAP_TYPE_RINGBUF` | Advantage for Streaming |
| :--- | :--- | :--- | :--- |
| **Buffer Structure** | **Per-CPU** array of ring buffers. | A single, **shared** ring buffer. |
| **Event Ordering** | **No guaranteed global order.** Events from different CPUs can be read out of sequence by the user-space program. | **Strictly preserves global order.** Events submitted first are consumed first, even if they originated on different CPUs. |
| **Real-Time HTTP** | **Poor.** Requires complex re-ordering logic in user-space to ensure a proper stream timeline (e.g., *open* event before *close* event). | **Excellent.** Simplifies user-space logic, as events can be immediately forwarded to the HTTP client in the correct sequence. |

### 2. Zero-Copy Efficiency (Better Performance)

* **`PERF_EVENT_ARRAY` (Older API):** 
    Events are typically prepared in a separate memory location (like the eBPF program's stack) and then 
    copied into the ring buffer via the `bpf_perf_event_output()` helper. This involves an extra **memory copy**.

* **`RINGBUF` (Modern API):** 
    It supports a **reserve/submit** API (using `bpf_ringbuf_reserve()` and `bpf_ringbuf_submit()`). 
    The eBPF program reserves space in the ring buffer, writes its data *directly* into that memory location, 
    and then submits it. 
    This is a **zero-copy** operation, which significantly boosts performance and lowers CPU overhead, a 
    major benefit for real-time high-throughput applications.

### 3. Improved Memory Utilization

* **`PERF_EVENT_ARRAY`:** 
    Requires a buffer for **every single CPU** (even if a CPU is idle). This often leads to wasted memory 
    and high memory overhead, especially on systems with many logical cores.

* **`RINGBUF`:** 
    Uses a single, **globally shared buffer**. 
    The size can be configured to match the expected total workload, dynamically absorbing spikes from any 
    CPU more efficiently, thus providing better memory efficiency and better **spike tolerance**.

### 4. Direct Support in Rust Crates

Modern Rust eBPF frameworks like **`Aya`** fully embrace and provide robust, safe abstractions for 
`BPF_MAP_TYPE_RINGBUF` (e.g., the `RingBuf` struct). 
Using it is the idiomatic and recommended way to stream events in new Rust-based observability tools.

---

## Conclusion

While you *can* build an HTTP streamer using **`BPF_MAP_TYPE_PERF_EVENT_ARRAY`**, it would be fighting the 
architecture:

* It forces your Rust user-space program to deal with complex **multi-CPU event re-ordering** just to 
  provide a coherent real-time stream.
* The extra memory copy and fragmented per-CPU buffers add unnecessary overhead.

**`BPF_MAP_TYPE_RINGBUF`** is the clear and modern choice for your application. 
It provides the **global event ordering** and **zero-copy efficiency** needed to reliably and performantly 
stream low-latency kernel events to your user-space Rust application for immediate forwarding over HTTP.


Usecase example: How to handle **back-pressure** (what happens when your HTTP consumer can't keep up with
the kernel events) using the `RINGBUF`'s features?

Handling **back-pressure** is one of the crucial features where the modern **`BPF_MAP_TYPE_RINGBUF`** excels, 
making it ideal for your real-time HTTP streamer.

In short, the kernel uses two primary methods to manage back-pressure when the user-space reader (your Rust 
application) is slow: **dropping events** and **reserving/committing**.

## Back-Pressure Mechanisms of `BPF_MAP_TYPE_RINGBUF`

The goal of back-pressure handling is to ensure the kernel doesn't waste CPU cycles producing data that the 
user-space consumer won't be able to process, or (critically) run out of memory.

### 1. Dropping Events (Default Behavior)

The ring buffer is a **fixed-size, circular queue** in memory.

* When the eBPF program attempts to write an event to the buffer, the kernel first checks if there is 
  enough contiguous free space.

* If the user-space reader has not consumed enough events, and the buffer is full, the new event is simply 
  **dropped**.

* **Advantage:** This is the most efficient form of back-pressure. The kernel doesn't block or wait; it 
  ensures the eBPF program returns quickly, preserving kernel performance.

* **Application Impact:** Your Rust program must be designed to tolerate and report these drops (known as 
  "loss"), as they indicate the HTTP endpoint is saturated.

### 2. Reservation Mechanism and Failure

The efficiency of the `RINGBUF` is tied to its `reserve/submit` API:

1.  **Reserve:** 
    The eBPF program calls `bpf_ringbuf_reserve()`. 
    If the buffer is full, this helper returns a **NULL pointer**, signaling failure.

2.  **Submit:** 
    The eBPF program checks for the NULL pointer. If the reservation fails, it **bypasses the write logic 
    entirely** and exits quickly.

This atomic check and immediate return upon buffer overflow is the mechanism that prevents the kernel from 
being overwhelmed by a slow user-space reader, enforcing the back-pressure policy.

---

## Comparing Back-Pressure

| Map Type | Back-Pressure Strategy | Performance on Overflow |
| :--- | :--- | :--- |
| **`BPF_MAP_TYPE_RINGBUF`** | **Explicit Drop.** Reservation fails, and the eBPF program stops event preparation immediately. | **Excellent.** Minimal CPU overhead. |
| **`BPF_MAP_TYPE_PERF_EVENT_ARRAY`** | **Implicit Drop.** Event output helper fails silently or overwrites data, and the failure is harder to handle explicitly. | **Good, but less efficient.** May involve more wasted work before the failure is determined. |

For a real-time streamer, the **`RINGBUF`'s** explicit drop mechanism is safer and provides clearer 
feedback to your Rust application that it's losing data due to consumer lag.

