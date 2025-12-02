# CO-RE:

## What is CO-RE?

**CO-RE** stands for **Compile Once, Run Everywhere**, and it's a feature in the context of **eBPF 
(extended Berkeley Packet Filter)** programs. 

The main idea behind CO-RE is that you can write an eBPF program without needing the exact source code of 
the kernel you're running it on, and still have the program work on different kernels or versions of the 
kernel.

### The Problem Without CO-RE

Normally, if you're writing an eBPF program that interacts with the kernel (e.g., for tracing system calls 
or network traffic), you need to know certain details about the kernel's internal data structures and funs.

These details change from one kernel version to another, and they vary depending on how the kernel is built. 
This means:

1. You need to **recompile** the eBPF program every time you run it on a different kernel version.

2. You might need the **kernel source code** or **headers** to figure out how to interact with the kernel’s 
   data structures (like `task_struct` or `sock`), which can be a hassle.

### How CO-RE Solves This Problem

CO-RE allows you to **write the eBPF program once** without needing to know the kernel's internal details at
the time of writing. 
The key to this is **BPF Type Format (BTF)**, which contains metadata about the kernel's data structures and
function signatures. 

Here's how it works:

1. **Use BTF Metadata**: 
    Instead of depending on the source code, CO-RE relies on **BTF**, a data format that describes kernel 
    structures and functions in a machine-readable way. 
    This metadata is **separate** from the kernel source code and can be used to understand the layout of 
    structures (like `task_struct` or `skb`).

2. **Compile Once**: 
    When you compile your eBPF program, you don’t need to include the kernel source code. 
    You just need to know the kernel's BTF data (which is available on the system at `/sys/kernel/btf/`), 
    and the program is compiled to **work with the kernel’s data structures** based on this metadata.

3. **Run Everywhere**: 
    Once compiled, your eBPF program will be able to run on **any kernel** that provides the necessary BTF 
    metadata. This means that you don’t need to recompile the program when moving to a different machine or 
    a new kernel version. 
    As long as the kernel supports the same BTF format, the program will work, and **you won’t need the 
    kernel source code**.

### Why Doesn't CO-RE Need Kernel Source?

To put it simply:

* **Without CO-RE**: 
    You would need the **kernel source code** or **headers** because you need to know the exact layout of 
    kernel structures and functions when writing your eBPF program. 
    These details can change between kernel versions.

* **With CO-RE**: 
    You don't need the kernel source code. You use the **BTF metadata** (which is like a map of kernel 
    structures), and this map allows your program to understand the kernel’s data structures and functions 
    **dynamically**. 
    You compile once, and the program works on any kernel that provides the correct BTF metadata.

### Example with CO-RE

1. **Without CO-RE**: 
    You write an eBPF program that needs to access the `task_struct` structure. You must know exactly how 
    `task_struct` is laid out in memory for the kernel you're compiling against, so you would need the 
    kernel headers for that specific version.

2. **With CO-RE**: 
    You write the same eBPF program using BTF metadata. 
    The program doesn't need to know the exact memory layout of `task_struct` at compile time. 
    It simply uses the BTF data, and when you load the program into the kernel, the kernel uses BTF to map 
    the structure dynamically. 
    This allows your program to run on any kernel version as long as it has the appropriate BTF data.


### Platforms that do not support BTF

Some platforms ( embedded ) come with kernel that is not compiled with BTF support.
CO-RE will not work on target system if the kernel does not provide BTF (BPF Type Format) data even if the
eBPF prgram itself is compiled with CO-RE support. 

- CO-RE relies on BTF metadata to understand kernel structures ( like `task_struct`, `skb`.. etc) at
  runtime. This BTF data is what allows the eBPF program to adapt to different kernel versions without
  needing to be recompiled. 

- If the kernel does not expose BTF data ( because it was compiled without `CONFIG_DEBUG_INFO_BTF` or other
  necessary kernel config options), there will be **no BTF metadata** available for the eBPF program to use, 
  and thus the program will not be able to correctly access or interpret kernel structures.

=> Without BTF: If the target kernel has no BTF support, there’s no way for the eBPF program to adjust to 
   the kernel’s internal structure, and CO-RE cannot function as intended.

### In Summary

* **CO-RE** makes eBPF programs **portable** across different kernel versions and setups, allowing you to 
  **compile once** and run everywhere.

* It does this by using **BTF (BPF Type Format)** metadata, which describes kernel data structures and
  functions without needing the kernel source code.

* This is useful because you don’t need to worry about kernel-specific details when writing eBPF programs. 
  You can write them once, and they’ll work on any compatible kernel with the right BTF information.


# Understanding /sys/kernel/btf/ 

The `/sys/kernel/btf/` directory typically contains multiple files,
```text
root@fedora:/sys/kernel/btf# ls
    acpi_cpufreq                kvm_intel                radeon
    amdgpu                      loop                     realtek
    amdxcp                      lpc_ich                  rfkill
    ata_generic                 lz4_compress             serio_raw
    cec                         lz4hc_compress           sha1_ssse3
    coretemp                    nf_conntrack             sha512_ssse3
    drm_buddy                   nf_conntrack_broadcast   snd
    drm_display_helper          nf_conntrack_netbios_ns  snd_hda_codec
    drm_exec                    nf_defrag_ipv4           snd_hda_codec_generic
    drm_panel_backlight_quirks  nf_defrag_ipv6           snd_hda_codec_hdmi
    drm_suballoc_helper         nf_nat                   snd_hda_codec_realtek
    drm_ttm_helper              nfnetlink                snd_hda_core
    fuse                        nf_reject_ipv4           snd_hda_intel
    gpio_ich                    nf_reject_ipv6           snd_hda_scodec_component
    gpu_sched                   nf_tables                snd_hrtimer
    hwmon_vid                   nft_chain_nat            snd_hwdep
    i2c_algo_bit                nft_ct                   snd_intel_dspcfg
    i2c_dev                     nft_fib                  snd_intel_sdw_acpi
    i2c_i801                    nft_fib_inet             snd_pcm
    i2c_smbus                   nft_fib_ipv4             snd_seq
    i7core_edac                 nft_fib_ipv6             snd_seq_device
    intel_cstate                nft_reject               snd_seq_dummy
    intel_pmc_bxt               nft_reject_inet          snd_timer
    intel_powerclamp            parport                  soundcore
    intel_uncore                parport_pc               sunrpc
    irqbypass                   pata_acpi                ttm
    it87                        pata_jmicron             uinput
    iTCO_vendor_support         pcspkr                   video
    iTCO_wdt                    ppdev                    vmlinux
    joydev                      qrtr                     wmi
    kvm                         r8169                    zram
```
along with the `vmlinux` BTF file. 

### **Understanding `/sys/kernel/btf/`**

The `/sys/kernel/btf/` directory is used to store BTF (BPF Type Format) data files. 

BTF is a format that describes the kernel's data structures, function prototypes, and other type information, 
which can be used by BPF programs for things like tracing, performance analysis, and debugging.

The kernel exposes its internal data structures (like `task_struct`, `skb`, etc.) in a way that BPF programs 
can use them efficiently. 

The BTF files contain this metadata, and it allows eBPF programs to work on different kernel versions 
without needing the exact source code or kernel headers.

### **Files in `/sys/kernel/btf/`**

1. **`/sys/kernel/btf/vmlinux`**:

   * This is the **primary BTF file** containing all the metadata for the kernel's internal data structures,
     types, and functions. The `vmlinux` file is essentially a BTF dump of the kernel, and it contains 
     detailed information about how different types in the kernel are structured.

   * **Purpose**: 
     This file allows tools like `bpftool` and other eBPF programs to access the BTF information and 
     interact with the kernel types dynamically.

2. **Other BTF Files in `/sys/kernel/btf/`**:

   * Along with `vmlinux`, you might also see other files in this directory. These files typically represent 
     the **BTF data for specific modules or other kernel parts**. 
     Some common examples:

     * **`/sys/kernel/btf/other_module`**: 
     If you're using kernel modules that have their own BTF data, you might see separate BTF files for each
     module. These files contain metadata specific to that module’s types and structures.

     * **`/sys/kernel/btf/` (with other specific names)**: 
     In some setups, there might be other files representing different parts of the kernel or specialized 
     BTF data for specific features.

     * **`/sys/kernel/btf/kconfig`**: 
     Sometimes you may find specific BTF metadata related to configuration options for the kernel. 
     This can be useful for tracing and debugging kernel builds.

### **Purpose of Multiple BTF Files**

Each BTF file corresponds to specific kernel or module information. The reason for multiple BTF files is that:

* **Modularization**: 
    The kernel can have many modules, and each module can generate its own BTF data. These modules might not 
    have all the same types or structures, so having individual BTF files allows BPF programs to query only 
    the relevant metadata for the part of the kernel or module they are interacting with.

* **Efficiency**: 
    Instead of loading the entire kernel BTF metadata for every module or feature, the system can keep 
    separate files, making it more efficient to access only the relevant metadata when needed.

### **What You Can Do with `/sys/kernel/btf/` Files**

You can interact with the BTF files in `/sys/kernel/btf/` using tools like `bpftool` to inspect and work 
with the metadata. For example:

1. **Inspect the `vmlinux` BTF file**:
   You can use `bpftool` to dump the BTF data from `vmlinux`, which gives you information about kernel types 
   and structures.

   ```bash
   bpftool btf dump file /sys/kernel/btf/vmlinux
   ```

   This command will dump the entire BTF data from `vmlinux` and show details about the kernel’s internal 
   structures, functions, and types.

2. **List BTF Types**:
   To get a list of available types from the BTF data, you can use:

   ```bash
   bpftool btf list file /sys/kernel/btf/vmlinux
   ```

   This command will list all the types available in the kernel's BTF data, including types like 
   `task_struct`, `skb`, and other kernel structures.

3. **Inspect a Specific Type**:
   If you're interested in a specific type (e.g., `task_struct`), you can dump that particular type by its 
   index (which you can get from the `bpftool btf list` output):

   ```bash
   bpftool btf dump file /sys/kernel/btf/vmlinux type <index_of_task_struct>
   ```

### **How to Identify Which BTF File to Use**

* **`vmlinux`**: 
    This is the main kernel's BTF file. You will use this most often if you're interacting with the core 
    kernel types (like `task_struct`, `skb`, etc.).

* **Other Files**: 
    If you're using kernel modules, you may need to use the corresponding BTF file for that module. 
    For example, if you are dealing with BPF programs that trace a specific module, you might have to 
    specify the BTF file for that module.

### **Example Scenario: Using Multiple BTF Files**

Suppose you have a system with a kernel and several modules. 
The `/sys/kernel/btf/` directory might look something like this:

```
/sys/kernel/btf/
├── vmlinux
├── net_module
├── fs_module
└── custom_module
```

* **`vmlinux`**: Contains the core kernel BTF data.
* **`net_module`**: Contains BTF data for the networking module.
* **`fs_module`**: Contains BTF data for the file system module.
* **`custom_module`**: Contains BTF data for a custom kernel module.

If you're writing an eBPF program that interacts with networking data structures, you'd use the `net_module`
BTF file, while if you're working with the file system, you’d use `fs_module`.

---

### **Summary**

* **`/sys/kernel/btf/`** contains BTF data files that describe the kernel's types and data structures, 
  making it easier for eBPF programs to interact with the kernel without needing kernel source code.

* The **`vmlinux`** file contains the core kernel BTF data, while other files may exist for specific kernel 
  modules or features.

* These files are used by tools like `bpftool` to interact with the kernel's BTF data and enable features 
  like **Compile Once, Run Everywhere (CO-RE)** for eBPF programs.

---

# **bpftool**

To query and work with BTF (BPF Type Format) files, there are several tools and programs available that 
allow you to extract useful information about kernel data structures, types, and functions. 

These tools are particularly valuable for profiling, debugging, and tracing the kernel or user-space 
programs using eBPF.

Below is an overview of the tools, how they are used, and how they can help with profiling and debugging:

## 1. **bpftool**

`bpftool` is the primary command-line utility used to interact with BPF (including BTF) data. 

It can be used to inspect, dump, and manipulate BTF files, and also to inspect loaded BPF programs and maps.

### Common Use Cases:

* **Listing BTF types**: 
    To list all the types available in the BTF metadata (from the kernel's `vmlinux` BTF file).

  ```bash
  bpftool btf list file /sys/kernel/btf/vmlinux
  ```

* **Dumping BTF data**: 
    You can dump detailed information about the BTF data for a specific type or the whole kernel. 
    For example, to dump the `task_struct` type, you would use:

  ```bash
  bpftool btf dump file /sys/kernel/btf/vmlinux | grep task_struct
  ```

* **Exploring BTF for debugging**: 
    You can query BTF data to understand kernel structures and resolve kernel types in your eBPF programs.

  ```bash
  bpftool btf dump file /sys/kernel/btf/vmlinux type <type_index>
  ```

### For Profiling and Debugging:

* **Type Information**: 
    Use `bpftool` to query kernel types (e.g., `task_struct`, `sock`, etc.) to help in writing eBPF programs 
    that target specific data structures.

* **Dynamic BTF Queries**: 
    If your eBPF program interacts with kernel types, `bpftool` helps to ensure that it is using the correct
    and up-to-date type information for different kernel versions.

### 2. **BPFTrace**

`bpftrace` is a high-level tracing tool that allows you to write dynamic tracing scripts using BPF. 
It can be used to trace kernel functions and events with minimal effort, and supports CO-RE 
(Compile Once, Run Everywhere) for portable eBPF programs.

### Common Use Cases:

* **Tracing Kernel Events**: 
    You can trace system calls, function entry/exit, and other kernel events dynamically.

  ```bash
  bpftrace -e 'tracepoint:sched:sched_process_fork { printf("PID: %d\n", pid); }'
  ```

* **Custom Tracing Scripts**: 
    Write more advanced tracing programs that can trace multiple kernel functions based on BTF types, e.g., 
    tracing `do_fork` to capture process creation.

  ```bash
  bpftrace -e 'kprobe:do_fork { printf("Process created: %d\n", pid); }'
  ```

### For Profiling and Debugging:

* **Function Call Tracing**: 
    Trace kernel function calls and inspect function arguments and return values, utilizing BTF to 
    dynamically resolve data structures.

* **Event Monitoring**: 
    Monitor system performance and kernel events, using BTF types to identify relevant data structures and 
    function signatures dynamically.

### 3. **BCC (BPF Compiler Collection)**

`BCC` is a collection of tools and libraries for writing BPF programs. 
It includes higher-level tracing tools that are useful for debugging and profiling. 
`BCC` uses BPF and can work with BTF data for more advanced analysis.

#### Common Use Cases:

* **Using BCC Tools**: 
    Tools like `trace` (to trace function calls) and `execsnoop` (to trace process executions) can leverage 
    BTF data for tracing kernel functions.

  ```bash
  bcc-tools/trace 'p:syscalls:sys_enter_fork'   # Trace fork system calls
  ```

* **Custom BCC Programs**: 
    Writing custom BCC programs using Python or C++ that rely on BTF to resolve kernel data structures 
    dynamically.

#### For Profiling and Debugging:

* **System Call Tracing**: 
    Use BCC to trace kernel system calls and functions, collecting metrics like function call latency, 
    arguments, and return values.

* **Performance Analysis**: 
    Profile kernel subsystems or track process execution patterns, helping debug performance bottlenecks.

### 4. **LLVM Tools (`llvm-objdump`, `llvm-demangle`)**

The LLVM toolchain also includes utilities that can help analyze kernel symbols and BTF data:

* **`llvm-objdump`**: 
    Use `objdump` to inspect kernel binaries (`vmlinux`), including symbol information. 
    You can use this alongside BTF to analyze kernel symbols and types.

  ```bash
  llvm-objdump -S /path/to/vmlinux
  ```

* **`llvm-demangle`**: 
    Used for de-mangling C++ symbols, if you are dealing with kernel C++ code or modules, to make them more 
    human-readable.

  ```bash
  llvm-demangle <mangled_symbol_name>
  ```

### 5. **BPF Type Format (BTF) Viewer**

This is a tool used for visualizing and exploring the contents of a BTF file.
While it’s not a common standalone tool, there are several scripts and utilities built on top of the 
BTF format that can be used to visualize the data for debugging purposes.

For example, using `bpftool` to extract the full BTF data and then visualizing it in a human-readable 
format can help clarify the relationships between types.

### How to Use BTF for Profiling and Debugging:

1. **Profiling with eBPF**:

   * Use `bpftool` or `bpftrace` to probe and trace kernel functions and events.
   * Utilize BTF to ensure that you are interacting with the correct types, especially when working with 
     complex kernel structures.

2. **Dynamic Type Resolution**:

   * By using BTF, you can write an eBPF program that dynamically resolves kernel data structures based on 
     the kernel version, without needing to hard-code the types. This is especially useful when targeting 
     kernel types like `task_struct`, `sock`, etc., that may change between kernel versions.

3. **Kernel Debugging**:

   * If you're debugging a kernel issue or investigating a kernel crash, you can use BTF to help trace the 
     specific functions and data structures involved in the crash or issue.

   * Use `bpftool` to query the kernel’s internal data types and figure out where your eBPF program 
     interacts with those structures.

   * `bpftrace` can also be used to trace events and functions to better understand what might be causing 
     an issue in the kernel or in user-space interactions.

4. **System Profiling**:

   * Use `bpftrace` or `BCC` to monitor performance and system behavior. With CO-RE support, these programs 
     can work across different kernel versions by resolving types dynamically via BTF.

   * Monitor system events like file system operations, process creation, network traffic, and more, using 
     the dynamic type resolution provided by BTF.

### Example Workflow:

1. **Writing a CO-RE eBPF Program**:
   Write an eBPF program that interacts with kernel data structures (e.g., `task_struct` or `skb`). 
   The program is compiled with CO-RE support, meaning it doesn’t need the kernel source code, just the BTF 
   data.

2. **Querying BTF Data**:
   Use `bpftool` or `bpftrace` to inspect the relevant types and structures available in the BTF file 
   (`/sys/kernel/btf/vmlinux`).

3. **Loading the Program**:
   Load your eBPF program into the kernel. The kernel will use the BTF data to resolve the types dynamically 
   at runtime.

4. **Debugging/Profiling**:

   * Use `bpftool` or `bpftrace` to monitor the program’s behavior, trace specific kernel events, and check 
     the program’s output for any issues.
   * If you encounter any issues, use the BTF data to help identify potential type mismatches or errors in 
     how the program interacts with kernel structures.

### Summary

* **Tools** like `bpftool`, `bpftrace`, `BCC`, and LLVM tools are used to query BTF files and help write, 
  debug, and profile eBPF programs.

* **BTF** makes it easier to write eBPF programs that work across different kernel versions without 
  requiring kernel source code.

* These tools can be used for **profiling**, **debugging**, and **system tracing**, making them invaluable 
  for eBPF-based monitoring, performance analysis, and debugging tasks.

---

# How to make eBPF program CO-RE Compatible:

Lets go step by step and clarify how CO-RE (Compile Once, Run Everywhere) support works, especially in the 
context of eBPF programs using `libbpf` or Rust's `aya` crate.

## 1. **Steps to Make eBPF Programs CO-RE Compatible**

To make an eBPF program CO-RE compatible, the key is **avoiding hardcoded kernel type details** at 
compile-time and using the **BTF (BPF Type Format)** metadata instead. 
By using BTF, your program can adjust to different kernel versions dynamically. 
This ensures the eBPF program can work on multiple kernel versions without needing to be recompiled.

Here's a breakdown of the steps:

### **Steps for Making eBPF Program CO-RE Compatible**:

1. **Use BTF Data**:
   The eBPF program must utilize **BTF (BPF Type Format)** to dynamically resolve kernel data structures at 
   runtime. Instead of using hardcoded kernel offsets or structure layouts, the program queries the kernel 
   for the data type layouts using BTF.

   * **BTF Metadata**: 
   The kernel exposes its internal types and functions through BTF files (located in `/sys/kernel/btf/`). 
   These metadata files describe structures like `task_struct`, `skb`, etc., in a way that eBPF programs 
   can use without knowing the exact kernel version at compile time.

2. **Avoid Hardcoding Kernel Types**:

   * Do not directly reference kernel data structures like `task_struct`, `skb`, etc., using their memory 
     layout. Instead, refer to them via **BTF type information**.

   * Use tools like `bpftool` to query the BTF data and understand the types used by your eBPF program.

3. **Use `libbpf` (or Other eBPF Libraries)**:
   If you're writing your program in C or a language that relies on `libbpf` (the standard eBPF library), 
   `libbpf` can load BTF data at runtime and resolve the necessary types dynamically. 
   **libbpf** has built-in support for CO-RE.

   * When your eBPF program uses **`libbpf`**, it can automatically query the kernel BTF data, adjusting 
     the program behavior depending on the kernel version and structure layout.

4. **Target Kernel Must Expose BTF**:
   For CO-RE to work, the **target kernel must expose BTF** data. 
   This is typically available in `/sys/kernel/btf/vmlinux`. 
   If the kernel is compiled without BTF support (i.e:missing `CONFIG_DEBUG_INFO_BTF`),then CO-RE wont work.

   * The **target machine's kernel must also have BTF support** enabled, which means it must have been 
     compiled with the `CONFIG_BPF` and `CONFIG_DEBUG_INFO_BTF` options.

5. **Compile with BTF Support**:
   The eBPF program should be compiled with **CO-RE support**, meaning the code should be written to be 
   independent of kernel source details (types, structures). 
   This is typically done by using BTF data at runtime rather than compiling the program to a specific 
   kernel version.

---

## 2. **Using `libbpf` for CO-RE**:

### **What is `libbpf`?**

`libbpf` is the **C library** that provides all the necessary functions for working with BPF in the Linux 
kernel. 

This includes loading eBPF programs, managing BPF maps, and handling BTF data.

* **CO-RE in `libbpf`**: `libbpf` automatically resolves kernel structures at runtime by loading the BTF data 
  from the `/sys/kernel/btf/vmlinux` file (or other files in `/sys/kernel/btf/`). 
  This makes it **CO-RE compatible** because the program adapts to the kernel dynamically.

### **How to Use `libbpf` for CO-RE:**

1. **Compile eBPF Program with CO-RE Flags**:
   When compiling the eBPF program, you need to **link against `libbpf`** and ensure the program uses 
   **BTF data**. 
   Typically, this is done via:

   * **eBPF Program**: 
    When loading the eBPF program in C, use `libbpf` to load the program and resolve types dynamically 
    using the BTF.

   * **CO-RE Support in C**: 
    You would need to use BTF data to resolve structures and function types at runtime. 
    This involves using **BPF Type Resolution APIs** provided by `libbpf`.

2. **Ensure BTF Availability**:

   * Your kernel must have BTF data enabled (`CONFIG_DEBUG_INFO_BTF`).
   * Use `bpftool` to check whether the kernel exposes BTF data at `/sys/kernel/btf/`.

---

## 3. **Using `aya` for Rust with CO-RE (Independent of `libbpf`)**:

### **What is `aya`?**

`aya` is a Rust crate that provides bindings to interact with eBPF in Linux. 
Unlike `libbpf`, which is the C library, `aya` is **written in Rust** and offers a more Rust-friendly 
interface for working with eBPF programs.

* **CO-RE Support in `aya`**: `aya` does not depend on `libbpf`, which means it is already independent of 
  C libraries.
  It is also designed to be **CO-RE compatible** if you follow the same principles of resolving kernel 
  types dynamically via **BTF**.

### **How to Make Rust Programs CO-RE Compatible Using `aya`:**

1. **Use `aya` with BTF**:

   * When writing eBPF programs in Rust using `aya`, the key to CO-RE compatibility is leveraging 
     **BTF metadata**. Just like with `libbpf`, the Rust program must query the kernel BTF data at runtime 
     to resolve types dynamically.

2. **Cross-compile with Musl for Minimal Dependencies**:

   * One of the advantages of using Rust is the ability to **compile the program statically** with the 
     **Musl libc**, which means **fewer runtime dependencies** compared to dynamically linked programs.

   * When you compile a Rust-based eBPF program with the **Musl** target (as opposed to the default glibc), 
     the program will be more portable, with fewer external dependencies. 
     This makes it truly **CO-RE**—the program can run on any target system with the appropriate kernel 
     BTF data, without depending on external libraries like `libbpf`.

   Example command to compile with Musl:

   ```bash
   cargo build --target x86_64-unknown-linux-musl --release
   ```

3. **Ensure BTF Availability on Target Kernel**:

   * Similar to using `libbpf`, the target system must expose the correct BTF metadata (usually at 
     `/sys/kernel/btf/vmlinux`). 
     If the target kernel lacks BTF data, then **CO-RE** won't work regardless of whether you're using 
     `libbpf` or `aya`.

---

## 4. **Comparison of `libbpf` and `aya` for CO-RE**:

| Feature               | `libbpf` (C)                                          | `aya` (Rust)                                                      |
| --------------------- | ----------------------------------------------------- | ----------------------------------------------------------------- |
| **CO-RE Support**     | Built-in, resolves kernel types dynamically using BTF | Built-in, works independently of `libbpf` with Rust-friendly APIs |
| **Dependencies**      | Requires `libbpf` and `libc` (dynamic linking)        | Minimal dependencies (e.g., Musl or glibc)                        |
| **Language**          | C (uses `libbpf`)                                     | Rust (no external dependencies like `libbpf`)                     |
| **BTF Support**       | Must query BTF data (via `libbpf`)                    | Direct support for BTF, no dependency on `libbpf`                 |
| **Cross-compilation** | Requires manual configuration for cross-compiling     | Supports cross-compiling easily (e.g., Musl)                      |

---

## 5. **Summary:**

* To **make an eBPF program CO-RE compatible**, you need to:

  1. Use **BTF data** to dynamically resolve kernel types at runtime.
  2. Ensure that the **target kernel exposes BTF** data.
  3. Use **`libbpf`** (C) or **`aya`** (Rust) to load and interact with eBPF programs in a CO-RE-compatible way.

* **`libbpf`** provides CO-RE support and works in the C ecosystem but requires `libbpf` to be installed and
  linked.

* **`aya`** provides CO-RE support in Rust and is independent of `libbpf`. 
  If compiled with **Musl**, Rust programs become **truly CO-RE** with minimal external dependencies.

Both approaches allow you to write eBPF programs that are portable across different kernel versions, 
provided the target kernel has BTF data available.

