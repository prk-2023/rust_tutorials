# Rust based eBPF Programming: ( Prerequisites )

## Essentials in Short:
In short, the topics essential for working with `eBPF` programs include both user-space and kernel-space 
programming. Below is a short list of the key topics that require a solid understanding:

- What are Hooks  and how they work
- What are tracepoints
- what is XDP
- What are Maps and helper functions 
- eBPF VM and how its interfaced to other kernel sub-systems.
- Work flow of eBPF sub-systems How programs get loaded, verified, compiled and executed and its limitations 
- What system calls are supported to : load, attach to hooks, create/configuring maps and more
- What libraries exist to work with eBPF programs.
- CO-RE, BTF: How BTF works (kernel config, to BTF File and how to work with it cmdline and via libbpf, aya)
- Misc: Parsing, Loading, llvm, Async, ....


## Main Topics: 
`eBPF` is an advanced and complex area to work with.

Below are some background on each of the topics, provide key concepts and how they connect 
with Aya, and mention any tools and system calls that could be useful.

### 1. What are Hooks and How They Work

* Hooks are points in the kernel where eBPF programs can be attached to monitor or modify the kernel's
  behavior.
* Examples of hooks include:

  * **XDP** (eXpress Data Path) for network packet processing.
  * **TC**(Traffic Control) for network traffic shaping.
  * **tracepoints**, **kprobes**, **uprobes**, and **perf** events for tracing events in the kernel.
* Aya provides abstractions to load eBPF programs at these hooks using the `Bpf` object.

### 2. What are Tracepoints

* Tracepoints are static points in the kernel code where eBPF programs can attach to collect data about
  kernel events (e.g., function calls, system events).( these points are put in by the kernel developers )
* Tracepoints can be used for tracing without needing to recompile the kernel.
* Aya supports tracepoint attachment through its `Bpf` object (`Bpf::tracepoint()`).

### 3. What is XDP

* **XDP** (eXpress Data Path) is a framework for high-performance packet processing, allowing you to run
  eBPF programs at the earliest point in the Linux networking stack (on the network interface card driver).
* **XDP** enables features like:

  * DDoS Mitigation
  * Traffic Filtering
  * Load Balancing
* Aya supports XDP through the `Xdp` program type and provides functions to attach and configure XDP programs.

### 4. What are Maps and Helper Functions

* Maps are used to store data that eBPF programs can read/write. They persist between different program
  executions and are an essential part of eBPF’s interaction with the kernel.

* Common map types include:

  * Hash maps
  * Array maps
  * Perf event maps for tracing.

* Aya simplifies map creation with abstractions like `Map` and `MapBuilder`.

* Helper Functions are used by eBPF programs to interact with the kernel (ex:loading data, updating maps). 
  Some examples include:

  * `bpf_map_update_elem()`
  * `bpf_map_lookup_elem()`
  * `bpf_redirect()` for XDP.

* Aya provides Rust bindings for these kernel helpers.

### 5. eBPF VM and How It's Interfaced to Other Kernel Sub-Systems

* eBPF programs are executed in a virtual machine in the kernel, called the eBPF VM.
* This VM allows you to run small programs on various subsystems, such as networking (XDP, TC), tracing
  (tracepoints, kprobes), and monitoring (perf).
* Aya interfaces with this eBPF VM by loading programs into the kernel using system calls (`bpf()` syscall)
  and interacting with various kernel subsystems like networking, tracing, and perf.

### 6. Workflow of eBPF Sub-Systems

* The general eBPF program lifecycle includes:

  * **Loading**: Load the eBPF program into the kernel using system calls like `bpf()`.
  * **Verification**: eBPF programs are verified for safety and correctness before execution.
  * **Compilation**: In some cases, eBPF code needs to be compiled into a form compatible with the kernel 
    (via LLVM).
  * **Attachment**: Programs are then attached to hooks (e.g., tracepoints, XDP, etc.).
* Aya abstracts most of these steps, providing high-level functions to load, verify, and attach programs.

### 7. What System Calls are Supported

* eBPF programs interact with the kernel via the `bpf()` system call, which supports several commands for:

  * Loading programs (`BPF_PROG_LOAD`)
  * Attaching programs to hooks (`BPF_PROG_ATTACH`)
  * Creating and managing maps (`BPF_MAP_CREATE`, `BPF_MAP_UPDATE_ELEM`)
  * Querying program status (`BPF_OBJ_GET_INFO_BY_FD`)
* Aya interfaces with the `bpf()` syscall, making these tasks easier by wrapping them in high-level
  abstractions.

### 8. What Libraries Exist to Work with eBPF Programs

* Aya is one of the most popular and actively developed Rust libraries for working with eBPF. 
  It provides a Rust-native API for:

  * Loading and managing eBPF programs.
  * Creating and manipulating maps.
  * Attaching programs to kernel subsystems.
* Other eBPF libraries include:

  * "libbpf" (C-based, often used directly in kernel-level code or through higher-level bindings).
  * BCC (BPF Compiler Collection, a higher-level interface, primarily used with Python).

### 9. CO-RE, BTF: How BTF Works

* **CO-RE** (Compile Once, Run Everywhere) allows you to write eBPF programs that are agnostic to kernel version
  differences, making them portable.
* **BTF** (BPF Type Format) is a debugging and metadata format that describes types in eBPF programs. 
  BTF helps with kernel version agnosticism and enables introspection.
* Aya supports BTF via the `aya-obj` crate, allowing you to parse and manipulate BTF data and work with 
  BTF-based features like **CO-RE**.
* Kernel Configuration for BTF: BTF support requires the kernel to be compiled with the `CONFIG_BPF_SYSCALL` 
  option enabled.

  * Once you have a kernel with BTF support, you can extract BTF information from the running kernel 
    (`/sys/kernel/btf/`), or from ELF files.
  * Aya allows you to load BTF data and use it for CO-RE to ensure that eBPF programs work across different 
    kernel versions.

### 10. Misc: Parsing, Loading, LLVM, Async

* **LLVM**: For compiling eBPF programs, the `clang`/`llvm` toolchain is typically used. 
  Aya can interface with programs that are already compiled into BPF bytecode (using LLVM or `clang`).
* **Parsing**: Aya can help load and parse BPF programs from object files, and the `aya-obj` crate helps with 
  handling BTF files and other kernel data structures.
* **Async**: Aya provides async support, which is especially useful if you're writing tools that interact with 
  kernel data in a non-blocking way (e.g., monitoring BPF maps asynchronously or processing network packets 
  in an async runtime like `tokio`).

---

### Quick Checklist to Dive into Aya eBPF

Here’s a condensed view of what you should focus on when diving into Aya for eBPF:

1. Understand eBPF sub-systems and how they interact with the kernel (networking, tracing, perf, etc.).
2. Learn how to load, attach, and manage eBPF programs in Rust with Aya.
3. Get familiar with eBPF program types: XDP, TC, Tracepoints, etc.
4. Work with maps and **helper functions** that eBPF programs use to communicate with the kernel.
5. Use BTF and **CO-RE** for writing kernel-version-agnostic programs.
6. Asynchronous handling: Learn how to interact with the kernel asynchronously in a non-blocking way 
   using `tokio` or `async-std` with Aya.

##  Additional Sub-Topics:

### 1. eBPF Program Types

* Apart from **XDP** and **tracepoints**, there are other types of eBPF programs to be aware of:

  * **TC (Traffic Control)**: Used for controlling network traffic.
  * **Socket Filters**: Attach to sockets for packet filtering.
  * **Perf Events**: For performance monitoring and tracing.
  * **LSM (Linux Security Module)**: Hook into security-related actions.
  * **Cgroup BPF**: Work with control groups to enforce policies on containers.
  * **BPF Type Format (BTF)**: This could also have subtypes like **BTF-based maps** and **helpers**.

### 2. eBPF Maps Advanced Usage

* **Advanced map types** and interactions is worth understanding:

  * **Per-CPU Maps**: Allows for efficient handling of data in multi-core systems.
  * **Pinned Maps**: Maps that are pinned into the BPF filesystem, allowing for persistence and interaction between user space and kernel space.
  * **Array of Arrays**: Sometimes used in more advanced programs for multi-dimensional data storage.
  * **Map Access Control**: Permissions and security around who can access the maps (e.g., restricting access to only certain user space programs).

### 3. Error Handling and Debugging in eBPF

* **Debugging** eBPF programs can be tricky, and understanding the **error codes**, **debugging strategies** 
  is important.

  * **BPF Verifier**: This checks eBPF code for errors before it’s loaded into the kernel.
  * **Verifying Program Safety**: How to deal with failed verification (ex: what errors you can expect from
    the BPF verifier and how to fix them).
  * **eBPF Map Debugging**: Debugging interactions with BPF maps, and tools like `bpftool` for inspecting 
    live maps.

### 4. eBPF Performance Optimization

* Understanding **performance** is key for eBPF, especially when you're working with XDP or TC for 
  high-speed networking or packet filtering:

  * **Tail Calls**: Efficient function call handling inside eBPF programs (jumping between programs).
  * **Program Elision**: Optimizing how you load or bypass certain BPF programs when possible.
  * **Map Lookup Efficiency**: Considerations for how quickly you can look up data in maps.
  * **Instruction Limit**: The **eBPF instruction limit** enforces a maximum number of instructions that can 
    run in one program. This can become a bottleneck, so understanding how to optimize prog size is important.

### 5. Kernel Internals / eBPF Internals

* Understanding how eBPF works internally in the kernel can be helpful when troubleshooting or optimizing 
  programs. 
  This includes:

  * **eBPF Program Lifecycle**: From user space loading, to kernel execution, and back to user space.
  * **The eBPF Verifier**: The verifier ensures that eBPF programs are safe and won’t crash the kernel. 
    Understanding how it works helps in debugging.
  * **How eBPF interacts with other kernel components** like networking, security modules, and the cgroup 
    subsystem.
  * **Program Tail Calls and Linking**: How programs can call each other and how program chaining works 
    (especially in XDP, where you might want to chain different programs).

### 6. Security and Privileges with eBPF

* Working with eBPF can sometimes raise concerns about security, especially given how eBPF can access 
  kernel data:

  * **eBPF Security Model**: How eBPF programs are sandboxed, limitations of what they can access, and their
    security constraints (e.g., **CAP_BPF**, the ability to load eBPF programs).
  * **Privileged Operations**: Some operations may require elevated privileges (e.g., loading XDP programs, 
    creating maps in privileged namespaces).
  * **BPF Type Format (BTF) and Seccomp**: Understanding how **seccomp** (secure computing mode) works with 
    eBPF and how to use it for restricting eBPF program behaviors.

### 7. eBPF in Containers / Kubernetes

* If you're working with **containerized** applications, you'll encounter **cgroup-based eBPF programs** and 
  might need to interact with **Kubernetes** to perform network monitoring, security, and observability:

  * **Cgroup BPF**: Create eBPF programs that interact with cgroups (useful for container monitoring).
  * **Kubernetes & eBPF**: Using eBPF for networking (Cilium),observability, and security in Kubernetes env's.

### 8. Cross-Platform eBPF

* **Portability of eBPF**: With CO-RE (Compile Once, Run Everywhere) and the kernel version mismatches, it’s
  crucial to understand how to ensure eBPF programs run across multiple kernel versions.

  * **BTF-based CO-RE**: Using BTF to write portable code and ensure eBPF programs work even if the kernel 
    changes.
  * **Handling eBPF Program Versions**: When using tools like Aya, it’s important to write code that works 
    across different kernel versions and distributions.

### 9. Monitoring and Tracing Tools for eBPF

* **Tools** like `bpftool` and `bpftrace` are very useful for interacting with live eBPF programs.

  * **bpftool**: Allows you to inspect eBPF programs, maps, and attachments, both in user space and kernel 
    space.
  * **bpftrace**: High-level tracing language built on eBPF, useful for quick prototyping and debugging of 
    tracing programs.

### 10. eBPF in the Context of Linux Kernel Development

* Understanding **Linux kernel internals** will help you better understand how eBPF fits into the overall 
  kernel architecture. This includes:

  * **Kernel Module Development**: If you plan to integrate eBPF into kernel modules, this becomes relevant.
  * **eBPF vs. Kernel Modules**: The advantages of eBPF in terms of security (no need to load kernel modules), 
    flexibility, and safety.

### - Recap:
1. eBPF Program Types (XDP, TC, LSM, etc.)
2. Advanced Map Usage (Per-CPU maps, pinned maps, etc.)
3. eBPF Error Handling and Debugging
4. Performance Optimization (e.g., Tail Calls, Map Lookups)
5. Kernel Internals and eBPF Internals
6. Security and Privileges (CAP_BPF, Seccomp, etc.)
7. eBPF in Containers / Kubernetes (Cgroup-based BPF, Cilium)
8. Cross-Platform eBPF and Portability (BTF, CO-RE)
9. eBPF Monitoring and Tracing Tools (bpftool, bpftrace)
10. eBPF in Linux Kernel Development (eBPF vs. Kernel Modules)

