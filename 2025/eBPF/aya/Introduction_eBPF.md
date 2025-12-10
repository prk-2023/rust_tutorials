# eBPF Introduction and Core Concepts:

Note: Many items in the document would require additional references from different domains, the documents
main focus is on the required topics for working/developing eBPF programs.


##  `eBPF` Overview : 


**eBPF** : (Extended Berkely Packet Filter)

A powerful Linux technology that allows developers to run safe, sandboxed programs directly within the 
operating system kernel.
It evolved from its original purpose as a fast network packet filtering into a general-purpose execution 
engine, enabling custom kernel behavior without modifying kernel source code or loading unstable kernel
modules. These extension allows to write programs for **observing**, **modifying** and **extending kernel
behaviour** without requiring kernel modules or custom kernel builds. 

| Key Aspect | Description |
| :--- | :--- |
| Purpose| "Enables safe, event-driven programmability of the Linux kernel."|
| Safety | "Programs must pass an in-kernel Verifier check to prevent crashes (e.g., infinite loops, illegal memory access)." |
| Performance | Verified bytecode is typically translated into native machine code by a Just-In-Time (JIT) compiler for near-native execution speed. |
| Core Use Cases | "Observability (Tracing, Profiling, Custom Metrics), Networking (Load Balancing, XDP), and Security (System Call Filtering)."|
| Interface | The entire framework is managed via the bpf() system call. |
| Unsafe Kernel modules| Avoids writing fragile or unsafe kernel modules |

This unique model allows powerful instrumentation while maintaining strong safety guarantees.
The `eBPF` programs are loaded into kernel from user-space and which run in a restricted, sandboxed
environment. The kernel verifies and executes these programs safely, ensuring system stability. 

To load `eBPF`  programs, handle other features of `eBPF` framework `bpf()` system call performs the
required magic.

User-space applications can be written in C, CPP, Rust. But this requires a two world model, which requires
the cooperation of two environments ( kernel and user space )

### - User-space: ( Controller: Prepares and manages `eBPF` programs )
This environment is responsible for writing, compiling, loading, and managing the lifecycle of the `eBPF` 
program and its data structures.

1. Development: Programs are typically written in C, C++, or Rust.

2. Toolchain: Tools like Clang/LLVM compile the source code into optimized `eBPF bytecode`.

3. Loading: Libraries like `libbpf` or frameworks like `BCC`, `Aya` use the `bpf()` syscall to load the
   bytecode into the kernel.

4. Interaction: User-space applications use `eBPF maps` to exchange data with the running kernel programs
   and retrieve results.

5. Attach `eBPF` programs to kernel hooks, `tracepoints`, or networking layers.

### - Kernel Space: ( Executor, runs a valid logic )

This is the secure, high-performance runtime environment where the `eBPF` program logic is executed.

1. Verification: The eBPF Verifier statically analyzes the bytecode to ensure safety (ex: bounded execution,
   valid memory access, no illegal system calls).
   The verifier does allow bounded loops (loops where the kernel can prove a max number of iterations). 
   i.e it checks for infinite loops.

2. Compilation: If verified, the bytecode is compiled by the JIT compiler into native machine instructions 
   for the host CPU.

3. Attachment: The program is attached to a specific Hook Point (e.g., a system call, a kernel function, a 
   network interface, or a static tracepoint).

4. Execution: The program runs only when the attached event occurs, executing the logic efficiently within 
   the kernel's context.

5. Kernel Helpers: Programs can call `eBPF` Helper Functions to interact with the kernel (ex: read time,
   get process information, manage maps).


### - Complete workflow: ( lifecycle of `eBPF` program )

End-to-End process from code development to runtime execution and data exchange.

1. Write Program: Developer writes the kernel-side logic (usually in C, Rust) and the user-space management
   application in (C, Rust or Go)

2. Compile: The kernel-side code is compiled into `eBPF` bytecodeusing `Clang`/`LLVM`.

3. Load: The user-space application uses `libbpf` to load the bytecode into the kernel via the `bpf()` syscall.
    
4. Verify & Compile: The kernel's Verifier checks the program, and the `JIT` compiler translates it to 
   machine code.

5. Attach: The program is attached to a specific kernel **Hook Point** ( network driver, tracepoint ...)

6. Execute: The program runs efficiently in the kernel whenever the associated event is triggered.

7. Data Exchange: Results are passed back to the user-space application via shared `eBPF Maps` or other 
   structures like the `perf buffer` or `trace pipe`.


### - `eBPF` ( super powers ):

- Safety: `eBPF` programs are checked by kernel verifier.

- Performance: The verified programs run at near-native speed ( JIT converts the byte code to machine code)

- Flexibility: Allows the developer to attach the byte code to various kernel hook points ( Networking,
  tracing, security )

- Low Overhead: Ideal for Observability and high-performance packet processing.

- Use Cases:
    * Networking: `XDP`, `TC`, `Socket Filters` 
    * Observability: `kprobes`, `tracepoints`, `perf events`
    * Security Monitoring: CPU, memory, IO instrumentation

## - The Two-World Model (User Space vs Kernel Space)**

`eBPF` operates using a dual-environment design, responsibilities are split between user and kernel space. 
Both sides work together to make `eBPF` programs safe, flexible, and easy to deploy.

### 1. User Space

User space is where the developer interacts with `eBPF` before it ever reaches the kernel.

#### Main Responsibilities

* **Write eBPF programs** (typically in C or Rust)
* **Compile** them into eBPF bytecode (via LLVM/Clang)
* **Load** the compiled program into the kernel using:

  * `libbpf`
  * `bpftool`
  * BCC
  * Direct syscalls (e.g., `bpf()` syscall)
* **Manage eBPF maps** (read/write shared data)
* **Attach** eBPF programs to kernel hook points

#### Key Tasks in User Space

* Prepare program → compile → load → attach
* Interact with `eBPF` maps to retrieve metrics, pass configuration, or send events.
* Handle program lifecycle (upgrade, reload, unload)

User space is essentially the “controller” that prepares and manages `eBPF` programs.

---

### 2. Kernel space 

`eBPF` programs actually *run*.

#### Main Responsibilities

* Verify the program for safety:

    - No illegal memory access
    - No infinite loops
    - Valid control flow
    - Limited stack depth

* JIT compile verified programs to native machine code for performance.

* Executes the program at the selected hook:

    - Network ingress/egress (XDP, TC).
    - System call entry/exit.
    - Kernel function probes (kprobes).
    - Tracepoints, perf events.
    - LSM hooks, cgroup hooks.
    - Provide **helper functions** to interact with kernel data.
    - Maintain **eBPF maps** shared with user space.

Kernel space is the “executor” that ensures safe, high-performance operation.

### 3. Why Two Worlds?

Since direct kernel programming (via modules) is risky.

Splitting provides:
    * Safety (kernel verifier)
    * Stability (no kernel crashes from bad code)
    * Flexibility (update logic at runtime)
    * Security (restricted sandbox)
    * Performance (JIT compilation)


## 3: Lifecycle of an eBPF Program (Creation → Loading → Execution)

### 1. Writing the eBPF Program (User Space)

An eBPF program is usually written in:
    * C (most common)
    * Rust (gaining popularity)
    * Higher-level frameworks (gobpf, aya, etc.)

The program must follow `eBPF` restrictions:

    - No unbounded loops
    - No arbitrary pointer arithmetic
    - Limited stack usage
    - Access memory safely

### 2. Compiling to eBPF Bytecode

The source code is compiled using LLVM/Clang:

```
clang -target bpf -O2 -c program.c -o program.o
```

This produces an **ELF file** containing:

    - eBPF instructions (bytecode)
    - Map definitions
    - Program metadata
    - Debug info (optional)

This file is still in user space.

---

### 3. Loading the Program into the Kernel

A user-space application loads the bytecode using tools/libraries like:

    - libbpf (standard for modern eBPF)
    - bpftool
    - BCC
    - Custom code calling the `bpf()` syscall directly

During this stage:

* Maps defined in the `ELF` file are created in the kernel
* The `eBPF` bytecode is prepared for verification

---

### 4. Verification by the Kernel Verifier

The kernel now inspects the program instruction-by-instruction.

It ensures:
    - No illegal memory access 
    - No out-of-bounds pointers 
    - No infinite loops 
    - Safe stack usage 
    - Valid control flow 
    - Calls only allowed helper functions

If verification fails, the load is rejected and the error is returned to user space.

---

### 5. JIT Compilation

After verification, the kernel:

    - Converts the eBPF bytecode into native machine code
    - Stores it in executable kernel memory

This allows the program to run at near-native speed.

---

### 6. Attaching the Program to a Hook

The user-space loader attaches the program to a kernel event, such as:

	- Network ingress (XDP)
	- Traffic control (TC)
	- kprobes (kernel function entry)
	- Tracepoints
	- sys_enter/sys_exit
	- cgroup hooks
	- LSM hooks

This defines *when* the program will run.

---

### 7. Execution in the Kernel

When the event triggers:

- The kernel executes the JIT-compiled eBPF program
- The program can:
	- Inspect data (packets, syscalls, kernel structs)
	- Modify packets or return values (depending on type)
	- Write to eBPF maps
	- Emit events to user space
	- Enforce policy (LSM, cgroup)

Execution must be fast and safe due to verifier constraints.

---

### 8. Interaction With Maps (Runtime Communication)

eBPF maps allow communication between:

- Kernel ↔ User space
- Kernel ↔ Kernel (multiple eBPF programs)

User space can:

- Read counters
- Dump events
- Update configuration dynamically

---

### 9. Unloading / Updating the Program

Because loading is dynamic:
	- Programs can be replaced at runtime
	- Maps can be preserved between reloads (pinning)
	- No system reboot is required


##  4: eBPF Program Types (Where eBPF Can Run in the Kernel)

**eBPF's** power comes from the ability to attach programs to many different kernel subsystems.
Each **program type** defines *where* the `eBPF` code runs, *what data it can access*, and *what actions it
can perform*.

Major program types:

### 1. XDP (eXpress Data Path)

Purpose: 
    High-performance packet processing at the earliest point in networking.

Where it runs: 
    Directly inside the network driver, *before* the Linux networking stack.

Typical uses:
	- DDoS mitigation
	- Load balancing
	- Packet filtering
	- Fast packet redirection

=> Extremely high performance (millions of packets/sec)
=> Runs before the kernel allocates socket buffers (zero-copy path)

### 2. TC (Traffic Control)

Purpose:
    Packet processing at a later stage, after the kernel has created socket buffers.

Where it runs:
    Ingress and egress hooks of the kernel’s traffic control subsystem.

Typical uses:
	- Traffic shaping
	- Routing decisions
	- More complex packet inspection
	- QoS enforcement

Compared to XDP:
    - Slower, but richer APIs 
    - Access to full skb (socket buffer) 
    - Allows both ingress and egress

### 3. kprobes / kretprobes

Purpose:
    Dynamic instrumentation of kernel functions.

Where it runs:
    - kprobe: When a specific kernel function is entered
    - kretprobe: When that function returns

Typical uses:
	- Performance tracing
	- Debugging kernel behavior
	- System monitoring tools (bcc, bpftrace)

Example:
    Trace all calls to `tcp_connect()` or `do_sys_open()`.

### 4. Tracepoints

Purpose:
    Stable kernel-provided instrumentation points.

Where it runs:
    - Predefined locations inside the kernel (e.g., scheduler, block I/O, networking)

Typical uses:
    - System observability
    - Low-overhead monitoring tools

Benefit: 
    Unlike kprobes, tracepoints are **stable APIs** and safe across kernel versions.

---

### 5. Uprobes / Uretprobes

Purpose:
    - Instrumentation of **user-space** applications.

Where it runs:
    - When a user-space function is entered (uprobe) or returns (uretprobe)

Typical uses:
    - Profiling applications
    - Debugging
    - Observability of user processes

Example:

Trace calls to `malloc()` in a user process.

### 6. Perf Events

Purpose:
    Hardware and software performance counters.

Where it runs:
    - Events triggered by perf (CPU cycles, cache misses, hardware counters)

Typical uses:
    - Profiling
    - Performance monitoring
    - Sampling-based observability tools

### 7. System Call Hooks

Purpose:
    Attach to syscall entry/exit points.

Types:
    - `sys_enter_*`
    - `sys_exit_*`

Uses:
    - Security monitoring
    - Tracing process activity
    - Auditing tools (Falco-like behavior)

### 8. Cgroup Hooks

Purpose:
    Apply policies or restrictions to specific control groups.

Where it runs:
    - On socket creation
    - On network packet ingress/egress
    - On device access

Uses:
    - Container isolation
    - Per-tenant network policies
    - Resource governance

### 9. LSM (Linux Security Module) eBPF Hooks

Purpose:
    Extend Linux security logic using eBPF.

Where it runs:
    - LSM security hooks (e.g., file access, socket creation)

Uses:
    - Mandatory access control
    - Custom security policies
    - Runtime enforcement

Why notable:
    This allows implementing parts of SELinux/AppArmor-like behavior in eBPF.

### 10. Fentry / Fexit (Modern Replacement for kprobes)

Purpose: 
    Ultra-low overhead kernel function entry/exit hooking.

Benefits:
    - Safer and faster than kprobes
    - Far fewer restrictions
    - No instruction patching required

Uses:
    - High-performance tracing
    - Production-grade observation tools (e.g., libbpf-based)

---

### - Summary Table (for clarity)

| Program Type         | Runs In               | Typical Use                      |
| -------------------- | --------------------- | -------------------------------- |
| **XDP**              | NIC driver layer      | Fast packet filtering/processing |
| **TC**               | Networking stack      | Routing, shaping, firewalling    |
| **kprobe/kretprobe** | Kernel functions      | Debugging, tracing               |
| **tracepoint**       | Kernel-provided hooks | Safe instrumentation             |
| **uprobe/uretprobe** | User-space processes  | Application tracing              |
| **perf events**      | Hardware counters     | Performance profiling            |
| **syscall hooks**    | Syscall boundary      | Monitoring, security             |
| **cgroup hooks**     | Resource controllers  | Container policies               |
| **LSM hooks**        | Security subsystem    | Access enforcement               |
| **fentry/fexit**     | Kernel functions      | High-speed tracing               |


## 5: eBPF Maps (Shared Data Structures Between User Space & Kernel Space)

eBPF maps are **key/value storage objects** used by eBPF programs to store and share data.
They enable communication between:
    - eBPF program → user space 
    - user space → eBPF program 
    - eBPF program → eBPF program

Maps are essential because eBPF programs cannot dynamically allocate memory or use standard global C
variable to maintain state. Instead, Maps are the primary mechanism for storing and sharing persistent or 
large mutable data between the kernel program and user-space, as well as between different eBPF programs. 

This constraint is reinforced by the extremely limited local execution stack, which is strictly capped
at 512 bytes.  

### 1. What Are eBPF Maps?

eBPF maps are:
   - Persistent kernel memory objects
   - Accessed via file descriptors in user space
   - Accessible inside eBPF programs through helper functions
   - Typed: each map has a specific map type (hash, array, ring buffer, etc.)

Maps have two interfaces:

1. **User space API** → through `bpf()` syscall or libbpf
2. **Kernel API** → via eBPF helper functions such as:

   * `bpf_map_lookup_elem()`
   * `bpf_map_update_elem()`
   * `bpf_map_delete_elem()`

### 2. Why Maps Are Needed

eBPF programs are:
	- Short-lived
	- Stateless (no global writable variables allowed)
	- Running in strict environments (verifier limitations)

Maps allow:
	- Storing counters, statistics, configuration
	- Passing events to user space
	- Sharing data between multiple programs
    - Maintaining persistent state across invocations

### 3. Common Map Types

Below are the most widely used map types with explanations.

#### A. Hash Map 

    Key-value hash table.

Uses:
    - Counters
    - Statistics per PID/CPU/IP
    - General dynamic storage

#### B. Array Map

Fixed-size array, keys are indices.

Uses:
    - Constant lookup tables
    - Small sets of structured data
    - Storing configuration

#### C. Per-CPU Maps

Each CPU gets its own instance of the map.

Uses:
    - High-performance counters (no locking)
    - Metrics that are aggregated later

Types:
    - `PERCPU_HASH`
    - `PERCPU_ARRAY`

#### D. LRU Hash Map

Hash map with Least Recently Used eviction.
Uses:
    - Cache-like functionality
    - Flow tracking (networking)
    - Storing many entries without memory overflow

#### E. Ring Buffer

High-performance one-way channel **kernel → user space**.

Uses:
    - Streaming events
    - Logging
    - Tracing output

Replaced older "perf buffer" in many modern eBPF setups.

#### F. Stack Trace Map

Stores stack traces captured by eBPF.

Uses:
    - Profiling
    - Flamegraph generation
    - Debugging performance issues

#### G. Queue / Stack Maps

Simple FIFO (queue) or LIFO (stack) structures.

Uses:
    - Passing data between eBPF programs
    - Task scheduling models
    - Lightweight message passing

#### H. Sockops / Sockhash Maps

Specialized networking maps enabling socket redirection and load balancing.

Uses:
    - LB frameworks (Cilium) 
    - Connection tracking
    - Socket-level policies

### 4. Map Lifecycle

1. Defined in the eBPF ELF object
2. Created by user space when the program loads
3. Referenced by both kernel and user space
4. Used while programs run
5. Optionally pinned to bpffs to persist beyond program lifetime
6. Destroyed when no longer referenced

### 5. Accessing Maps

#### In User Space:

Using **libbpf**:

```c
int fd = bpf_obj_get("/sys/fs/bpf/my_map");
bpf_map_update_elem(fd, &key, &value, BPF_ANY);
```

#### In eBPF Program (Kernel):

```c
value = bpf_map_lookup_elem(&my_map, &key);
if (value) {
    (*value)++;
}
```

### 6. Why Maps Are Critical

Without maps:
	- No dynamic data
	- No shared state
	- No communication
	- No observability
	- No events or metrics

They are the backbone of advanced eBPF applications like:
	- Cilium
	- Bumblebee
	- bpftrace tools
	- Falco-like security monitoring
	- Performance profilers


## 6: The eBPF Verifier (Kernel Safety Mechanism)

The **eBPF verifier** is one of the most critical components in the eBPF ecosystem.
It ensures that any program loaded into the kernel is **safe**, **terminates**, and **cannot crash or 
corrupt the system**.

The verifier is what allows eBPF to extend the kernel **without risking kernel stability**.

### 1. What Is the eBPF Verifier?

The verifier is a subsystem in the Linux that examines each eBPF program **before** it’s allowed to run.

It performs a form of static analysis by:
	* Evaluating all possible execution paths
	* Checking pointer safety
	* Verifying memory access rules
	* Ensuring termination

If the verifier detects anything unsafe, it **rejects the program**.

### 2. Why the Verifier Exists

Kernel modules can cause:
	* Crashes
	* Deadlocks
	* Security vulnerabilities

eBPF aims to provide kernel extensibility *without risk*.

The verifier guarantees:
	* No infinite loops
	* No null pointer dereferences
	* No out-of-bounds memory access
	* No arbitrary pointer arithmetic
	* No invalid helper usage
	* No unauthorized kernel access
	* Controlled stack usage

With these guarantees, eBPF is safe enough to run in production systems.

### 3. Main Checks Performed by the Verifier

#### A. Control Flow Validation

* No unbounded loops
* No excessively deep call stacks
* No invalid jumps
* All branches must lead to known, safe states

#### B. Memory Access Validation

* Pointers must always be safe
* Bounds must be known
* Stack access is strictly validated
* No writing to read-only data

#### C. Type Checking

The verifier tracks types of:

* Pointers
* Scalars (integers)
* Registers
* Stack slots

This prevents misuse of kernel data structures.

#### D. Helper Function Safety

Only approved kernel helper functions can be called.

Verifier checks:

* Correct arguments
* Allowed context
* Safe return values

#### E. Map Access Validation

* Key/value sizes must match the map definition
* Valid access to per-cpu maps
* Safe updating or deleting entries

### 4. Verifier Log Output

When a program fails, the kernel returns a detailed message such as:

```
invalid bpf_context access off=100 size=8
R1 min value is outside of allowed range
processed 1200 insns (limit 100000)
```

Developers usually enable verbose logs when debugging the verifier:

```
bpftool prog load x.o /sys/fs/bpf/x verbose
```

or via libbpf:

```c
libbpf_set_print(libbpf_print_fn);
```

### 5. Limitations Imposed by the Verifier

To maintain safety, the verifier enforces strict limits:

* **Stack size:** max 512 bytes
* **Program size:** maximum instruction count (~1M cap in modern kernels)
* **No unbounded loops:** bounded loops permitted only in newer kernels
* **No recursion**
* **Restricted pointer math**
* **No dynamic memory allocation** inside eBPF programs

### 6. Modern Improvements

Over time, the verifier has become more flexible:

* Support for **bounded loops**
* More map types allowed
* Better pointer tracking
* Fentry/fexit easing tracing requirements
* More helper functions and allowed contexts

This has significantly expanded eBPF’s capabilities.

### - Summary: What the Verifier Guarantees

If a program passes verification, the kernel guarantees:

* **No kernel crash**
* **No memory corruption**
* **No infinite loops**
* **Predictable execution time**
* **Safe access to kernel data**

These rules enable eBPF programs to run even on production-critical servers.

##  7: The eBPF JIT Compiler (How eBPF Achieves Near-Native Performance)

Once an eBPF program passes verification, the next stage is **execution**.
To make eBPF fast enough for real-time networking, tracing, and security tasks, the Linux kernel includes 
a **Just-In-Time (JIT)** compiler that converts eBPF bytecode into efficient native machine code.

This topic explains how the JIT works and why it’s crucial.

### 1. What Is the eBPF JIT Compiler?

The **eBPF JIT** is a kernel component that transforms validated eBPF bytecode into CPU-native instructions (x86, ARM64, etc.).
This means the program executes at **near the same speed as C code inside the kernel**.

Without JIT, eBPF would run in an interpreter mode, which is slower.

### 2. Why JIT Compilation Is Needed

eBPF programs often run in **hot paths**:

* Network packet processing (millions per second)
* Syscall monitoring
* High-rate tracing events
* Security hooks
* Kernel function entry/exit points

The JIT avoids the overhead of interpreting bytecode by compiling it once and reusing the machine code.

### **Performance benefits:**

* Runs at native CPU speed
* Handles massive event throughput
* Allows eBPF to replace traditional kernel modules
* Enables use cases like XDP at line rate (40–100 Gbps)

### 3. When the JIT Compiler Runs

The lifecycle looks like this:

1. Program passes verifier
2. Kernel checks if JIT is enabled (`/proc/sys/net/core/bpf_jit_enable`)
3. Kernel runs the architecture-specific JIT compiler
4. JIT emits executable machine code
5. Code is stored in kernel memory
6. Hook is attached and execution begins

If the JIT is disabled, the kernel falls back to **interpreter mode**, which is slower.

### 4. How the JIT Works Internally

#### A. Instruction Translation

Each eBPF instruction (load, store, ALU, branch, call, etc.) is translated into one or more CPU assembly instructions.

Example:

```
BPF_MOV64_REG r1, r2
```

may become:

```
mov rax, rbx
```

#### B. Register Mapping

eBPF has 11 virtual registers.
These are mapped to hardware registers depending on architecture.

#### C. Optimizations

The JIT may apply:

* Dead code elimination
* Peephole optimizations
* Constant folding
* Efficient control-flow mapping

#### D. Finalization

The kernel:

* Writes the machine code into executable pages
* Applies memory protection (RX only)
* Keeps a pointer to this code for fast invocation

### 5. Security Considerations

Because the JIT emits **raw executable code**, strict safety rules apply:

* Only verified bytecode is compiled
* No self-modifying code
* No RWX memory regions (only RX)
* JIT debugging can be disabled in production
* Hardened JIT modes prevent speculative execution leaks (mitigation for Spectre-like attacks)

The verifier guarantees safety before JIT is even invoked.

### 6. Debugging the JIT Output

Developers can inspect generated machine code:

```
cat /sys/kernel/debug/tracing/jit_dump
```

Or enable JIT dump via `/proc/sys/net/core/bpf_jit_enable = 2`.

This is invaluable when analyzing performance issues or debugging verifier behavior.

### 7. Summary: Why the JIT Is Critical

The eBPF JIT provides:

* **Performance** close to kernel-native C
* **Safety** enforced by verifier
* **Dynamic programmability** without kernel modules

Because of the JIT, eBPF programs can safely process packets at line rate, trace millions of events per 
second, and enforce security policies with minimal overhead.

## 8: eBPF Helper Functions (How eBPF Programs Interact With the Kernel)

eBPF programs run in a restricted environment with no direct access to kernel internals.
To safely interact with the kernel—read data, write to maps, emit events, manipulate packets—they rely on 
**helper functions**.

Helper functions act as **controlled gateways** into kernel functionality.

---

### 1. What Are eBPF Helper Functions?

Helper functions are kernel-provided APIs that eBPF programs can call.
They are the **only allowed way** to:

* Access eBPF maps
* Interact with kernel structures
* Parse or modify network packets
* Send data to user space
* Generate random numbers
* Get time information
* Redirect network packets
* Perform checksum operations

These helpers keep eBPF programs powerful yet safe.

### 2. How Helpers Are Called

Helpers are invoked by a special instruction:

```
call BPF_FUNC_xxx
```

Example inside an eBPF program:

```c
struct data_t *val;
val = bpf_map_lookup_elem(&my_map, &key);
```

This expands to a call to:

* `bpf_map_lookup_elem()`

### 3. Common Categories of Helper Functions

Below is a structured view of the most commonly used helpers.

#### **A. Map Operations**

These helpers allow eBPF programs to interact with maps.

##### **1. Lookup**

```c
void *bpf_map_lookup_elem(map, key)
```

##### **2. Update**

```c
int bpf_map_update_elem(map, key, value, flags)
```

##### **3. Delete**

```c
int bpf_map_delete_elem(map, key)
```

Used in:

* Counters
* Event tracking
* Configuration storage

---

#### B. Ring Buffer / Perf Buffer Output

##### Ring buffer (modern method):

```c
int bpf_ringbuf_output(rb, data, size, flags)
```

##### Perf buffer (older method):

```c
int bpf_perf_event_output(ctx, map, flags, data, size)
```

Use cases:

* Tracing
* Logging
* Event streaming to user space (e.g., security events, syscalls)

#### C. Packet Helpers (Networking)

Used in XDP, TC, and socket-level programs.

##### Packet access helpers:

```c
int bpf_skb_load_bytes(skb, offset, buf, len)
int bpf_skb_store_bytes(skb, offset, buf, len, flags)
```

##### Redirect helpers:

```c
int bpf_redirect(ifindex, flags)
int bpf_redirect_map(map, key, flags)
```

Use cases:

* Load balancing
* Firewalls
* Packet dropping
* Packet modification

#### D. Time and Clock Helpers

```c
u64 bpf_ktime_get_ns()
u64 bpf_jiffies64()
```

Uses:

* Latency measurement
* Profiling
* Tracing

#### E. Kernel Structure Access Helpers

##### Probe reads:

```c
int bpf_probe_read_kernel(dst, size, src)
```

##### Read user-space memory:

```c
int bpf_probe_read_user(dst, size, src)
```

Used in:

* kprobes/uprobes
* Tracing system calls
* Observing kernel internals safely

#### F. Security & LSM Helpers

Used by eBPF LSM programs:

```c
int bpf_inode_storage_get()
int bpf_task_storage_get()
```

Use cases:

* Security policy enforcement
* Access control logic

#### G. Random Number and Hash Helpers

```c
u32 bpf_get_prandom_u32()
u32 bpf_get_smp_processor_id()
```

Uses:

* Random selection
* Load balancing
* CPU affinity logic

### 4. Safety and Restrictions of Helper Calls

Helpers are strictly validated by the verifier:

* Correct number of arguments
* Correct types (pointer types, numeric types)
* Correct context (some helpers only allowed in certain program types)
* Cannot be called with dynamic pointers unless verified safe
* Cannot cause arbitrary kernel access

If usage is unsafe, verifier rejects the program.

### 5. How Helpers Extend eBPF Capabilities

Without helper functions, eBPF programs would be extremely limited.
Helpers allow:

* Reading kernel memory safely
* Writing to buffers
* Emitting events
* Redirecting packets
* Managing persistent data
* Applying security hooks

They are effectively the **system calls of eBPF**, but far more restricted to ensure safety.

### - Summary

Helper functions provide safe access to:

* Maps
* Kernel memory
* Packet data
* Events
* Security hooks
* Timing information

They form the bridge between eBPF sandboxed bytecode and kernel capabilities.


##  9: eBPF Tooling (How Developers Work with eBPF Programs)

eBPF programs are powerful, but writing, loading, and managing them manually is complex.
The Linux ecosystem provides **tools and libraries** to simplify working with eBPF programs. These tools cover compilation, loading, attaching, debugging, and monitoring.

### 1. libbpf (The Core Library)

**libbpf** is the official C library for interacting with eBPF programs in user space.

#### Responsibilities:

* Load eBPF ELF files
* Create and manage maps
* Attach programs to kernel hooks (XDP, kprobe, tracepoint, cgroup)
* Handle pinning maps and programs in `bpffs`
* Provide BPF system call wrappers

#### Why it matters:

* Standardized way to interface with the kernel
* Used by modern tools like Cilium, Falco, and BPFTrace
* Supports newer features like CO-RE (Compile Once – Run Everywhere)

#### Example (libbpf)

```c
struct bpf_object *obj;
bpf_object__open_file("prog.o", NULL);
bpf_object__load(obj);
bpf_program__attach(obj->programs[0]);
```

### 2. bpftool (Command-Line Utility)

**bpftool** is a versatile command-line tool to:

* Inspect eBPF programs and maps
* Load and pin programs
* Dump maps contents
* Show verifier logs
* Manage skeletons generated by libbpf

#### **Example Commands:**

```bash
# Show loaded eBPF programs
bpftool prog show

# Show maps
bpftool map show

# Dump map content
bpftool map dump id 1
```

### 3. BCC (BPF Compiler Collection)

**BCC** is a framework that makes eBPF easier for developers.

#### Features:

* Python, Lua, C++ interfaces
* JIT compilation and loading for eBPF programs
* Provides prebuilt tools for tracing, networking, and monitoring

#### Use Cases:

* Quickly writing trace tools
* Visualizing system calls
* Network observability

Example:

```python
from bcc import BPF

b = BPF(text="kprobe:do_sys_open { printf(\"open\\n\"); }")
b.trace_print()
```

### 4. bpftrace (High-Level Tracing Tool)

**bpftrace** is a high-level language inspired by **awk** for writing tracing programs.

#### Features:

* One-liners for tracing
* Supports aggregations, maps, histograms
* Uses LLVM/Clang to compile into eBPF bytecode

Example:

```bash
# Trace all execve syscalls
sudo bpftrace -e 'tracepoint:syscalls:sys_enter_execve { printf("%s\n", comm); }'
```

### 5. Other Tools and Ecosystem

* **bpffs** – a virtual filesystem to pin maps/programs persistently
* **Cilium** – eBPF-based networking and security platform
* **Falco** – eBPF-powered security and anomaly detection
* **Perf / perf_event** – used for kernel profiling with eBPF hooks
* **Aya (Rust)** – Rust library for eBPF program development

### - Tooling Summary

| Tool / Library | Purpose                             | Language / Interface |
| -------------- | ----------------------------------- | -------------------- |
| **libbpf**     | Load, attach, manage eBPF programs  | C                    |
| **bpftool**    | Inspect, debug, pin programs & maps | CLI                  |
| **BCC**        | Write and manage eBPF programs      | Python, Lua, C++     |
| **bpftrace**   | High-level tracing                  | DSL / CLI            |
| **Aya**        | Rust eBPF programming               | Rust                 |

These tools help developers focus on **logic** rather than low-level syscalls and bytecode management.

## 10: Example eBPF Program with Explanation

To consolidate everything learned, let’s go through a **simple eBPF program** example and explain its structure, purpose, and workflow.

### 1. Example Goal

We will create an eBPF program that:

* Counts the number of `open()` syscalls made by each process
* Stores the count in a **hash map**
* Allows user space to read the statistics

This demonstrates:

* **eBPF program creation**
* **Map usage**
* **kprobe attachment**
* **User-space interaction**

### 2. Kernel-Space Program (eBPF)

```c
#include <linux/bpf.h>
#include <bpf/bpf_helpers.h>

struct {
    __uint(type, BPF_MAP_TYPE_HASH);
    __uint(max_entries, 1024);
    __type(key, u32);      // PID
    __type(value, u64);    // Count
} open_count_map SEC(".maps");

SEC("kprobe/do_sys_open")
int count_open(struct pt_regs *ctx) {
    u32 pid = bpf_get_current_pid_tgid() >> 32;
    u64 *count, zero = 0;

    count = bpf_map_lookup_elem(&open_count_map, &pid);
    if (!count) {
        bpf_map_update_elem(&open_count_map, &pid, &zero, BPF_ANY);
        count = bpf_map_lookup_elem(&open_count_map, &pid);
    }

    if (count)
        (*count)++;

    return 0;
}

char LICENSE[] SEC("license") = "GPL";
```

#### Explanation

1. Map Declaration:

   * `open_count_map` stores counts per process (PID → u64)
   * Declared in `.maps` section

2. Program Attachment:

   * `SEC("kprobe/do_sys_open")` attaches to the `do_sys_open` kernel function

3. Counting Logic:

   * Retrieves current PID
   * Looks up PID in map
   * Initializes count if missing
   * Increments count

4. License Declaration:

   * Required for kernel to allow use of GPL-only helpers

### 3. User-Space Loader (C Example)

```c
#include <bpf/libbpf.h>
#include <stdio.h>

int main() {
    struct bpf_object *obj;
    int map_fd;

    obj = bpf_object__open_file("count_open.o", NULL);
    bpf_object__load(obj);

    map_fd = bpf_object__find_map_fd_by_name(obj, "open_count_map");

    // Periodically read map
    while (1) {
        u32 key = 1234; // example PID
        u64 value;
        if (bpf_map_lookup_elem(map_fd, &key, &value) == 0)
            printf("PID %u open count: %llu\n", key, value);
        sleep(5);
    }

    return 0;
}
```

#### Explanation

* Opens and loads eBPF object file
* Finds the map file descriptor
* Reads counts from map in a loop
* Prints counts for demonstration purposes

### 4. Workflow Recap

1. Write program in C (kernel-space logic)
2. Compile to eBPF bytecode (`clang -target bpf -O2 -c count_open.c -o count_open.o`)
3. Load program into kernel using **libbpf**
4. Map is created in kernel
5. Program executes on every `do_sys_open` syscall
6. Map keeps counts per PID
7. User space reads map to report metrics

### 5. Key Concepts Demonstrated

* **Two-world model:** user space loader + kernel eBPF program
* **eBPF map usage:** hash map for shared state
* **kprobe attachment:** intercepting kernel function calls
* **Verifier compliance:** safe memory access, bounded stack, safe helper calls
* **JIT compilation:** kernel executes efficiently

This is a minimal yet practical example of a **complete eBPF workflow**.


## Complete picture:
```
        ┌─────────────────────┐
        │   User Space Loader │
        │ (libbpf / bpftool)  │
        └─────────┬───────────┘
                  │ Load eBPF program (.o)
                  ▼
        ┌──────────────────────┐
        │ Kernel Verifier      │
        │ - Checks safety      │
        │ - Ensures termination│
        └─────────┬────────────┘
                  │ Pass?
                  │ Yes
                  ▼
        ┌────────────────────────┐
        │   JIT Compiler         │
        │ - Convert eBPF bytecode|
        │   to native instructions
        └─────────┬──────────────┘
                  │
                  ▼
        ┌────────────────────────┐
        │  Kernel Hook / Event   │
        │  (kprobe: do_sys_open) │
        └─────────┬──────────────┘
                  │ Executes eBPF program
                  ▼
        ┌─────────────────────┐
        │  eBPF Program       │
        │ - Reads PID         │
        │ - Updates map       │
        └─────────┬───────────┘
                  │
                  ▼
        ┌─────────────────────┐
        │  eBPF Map (Hash Map)│
        │ - Key: PID          │
        │ - Value: Count      │
        └─────────┬───────────┘
                  │
                  ▼
        ┌─────────────────────┐
        │  User Space Reads   │
        │ - Periodically polls│
        │   the map           │
        │ - Displays counts   │
        └─────────────────────┘
```

**Flow explanation:**

1. User space loads eBPF program and maps.
2. Kernel verifier ensures safety.
3. JIT compiler converts bytecode into fast native code.
4. Kernel hook triggers the eBPF program on `do_sys_open`.
5. eBPF program updates the shared hash map.
6. User space polls the map to get syscall counts.

---

