# Aya:
```mermaid
graph TB
    subgraph "User Space Applications"
        APP[Rust User Application]
        CLI[Command Line Tools]
        METRICS[Prometheus/Grafana]
    end

    subgraph "Aya Framework (Rust)"
        AYA_CORE[Aya Core Library]
        AYA_LOADER[BPF Loader]
        AYA_GEN[Code Generation]
        AYA_BIND[BPF Bindings]
        
        subgraph "Aya Components"
            PROG_MGR[Program Manager]
            MAP_MGR[Map Manager]
            LOGGER[Logger]
            PERF[Perf Event Handler]
        end
    end

    subgraph "Rust eBPF Programs"
        XDP[XDP Program]
        TC[TC Classifier]
        TRACE[Tracepoint]
        KPROBE[Kprobe/Kretprobe]
        SOCK[Socket Filter]
    end

    subgraph "Kernel Space"
        BPF_VM[eBPF Virtual Machine]
        JIT[JIT Compiler]
        VERIFIER[Verifier]
        
        subgraph "BPF Maps"
            HASH[HashMap]
            ARRAY[Array Map]
            PERCPU[Per-CPU Map]
            LRU[LRU Map]
            RING[Ring Buffer]
        end
        
        subgraph "Hooks"
            XDP_HOOK[XDP Hook]
            TC_HOOK[TC Hook]
            TRACE_HOOK[Tracepoint Hook]
            KPROBE_HOOK[Kprobe Hook]
        end
    end

    subgraph "External Systems"
        NETWORK[Network Packets]
        KERNEL[Kernel Functions]
        SYSTEM[System Calls]
    end

    %% Compilation & Loading Flow
    APP -->|Builds| AYA_GEN
    AYA_GEN -->|Generates| RUST_BPF[Rust BPF Code]
    RUST_BPF -->|Compiles| LLVM[LLVM to BPF]
    LLVM -->|BPF Bytecode| AYA_LOADER
    AYA_LOADER -->|Loads & Verifies| VERIFIER
    VERIFIER -->|JIT Compiles| JIT
    JIT -->|Native Code| BPF_VM

    %% Program Attachment
    AYA_CORE -->|Attach| XDP_HOOK
    AYA_CORE -->|Attach| TC_HOOK
    AYA_CORE -->|Attach| TRACE_HOOK
    AYA_CORE -->|Attach| KPROBE_HOOK

    XDP -->|Runs on| XDP_HOOK
    TC -->|Runs on| TC_HOOK
    TRACE -->|Runs on| TRACE_HOOK
    KPROBE -->|Runs on| KPROBE_HOOK
    SOCK -->|Runs on| NETWORK

    %% Data Flow
    XDP_HOOK -->|Processes| NETWORK
    TC_HOOK -->|Processes| NETWORK
    TRACE_HOOK -->|Traces| KERNEL
    KPROBE_HOOK -->|Probes| SYSTEM

    %% Map Operations
    BPF_VM -->|Accesses| HASH
    BPF_VM -->|Accesses| ARRAY
    BPF_VM -->|Accesses| PERCPU
    BPF_VM -->|Accesses| LRU
    BPF_VM -->|Writes Events| RING

    %% User Space Communication
    RING -->|Perf Events| PERF
    PERF -->|Data Stream| AYA_CORE
    AYA_CORE -->|Metrics| METRICS
    AYA_CORE -->|Control| CLI
    
    %% Map Management
    MAP_MGR -->|Manages| HASH
    MAP_MGR -->|Manages| ARRAY
    PROG_MGR -->|Controls| XDP
    PROG_MGR -->|Controls| TC
    
    %% Logging
    BPF_VM -->|Debug Logs| LOGGER
    LOGGER -->|Log Output| APP

    classDef rust fill:#f0a3a3,stroke:#333,stroke-width:2px
    classDef kernel fill:#a3c4f0,stroke:#333,stroke-width:2px
    classDef aya fill:#a3f0b0,stroke:#333,stroke-width:2px
    classDef external fill:#f0e0a3,stroke:#333,stroke-width:2px
    
    class APP,CLI,METRICS rust
    class BPF_VM,JIT,VERIFIER,HASH,ARRAY,PERCPU,LRU,RING,XDP_HOOK,TC_HOOK,TRACE_HOOK,KPROBE_HOOK kernel
    class AYA_CORE,AYA_LOADER,AYA_GEN,AYA_BIND,PROG_MGR,MAP_MGR,LOGGER,PERF aya
    class NETWORK,KERNEL,SYSTEM external
```

## What is Aya:

Aya is a collection of crates designed to work with `eBPF` programs. It's a powerful framework that provides
high-level abstraction and interfaces for working with `eBPF`, enabling developers to *write*, *load* and
*interact* with `eBPF` programs directly from Rust. 

At its core the main Aya crate provides all that is required for developing `eBPF` based programs for Linux
kernel.

It facilitates the development of `eBPF` programs with its family of crates `aya_xyz`. 
Its totally independent of many standard `eBPF` related libraries such as `libbpf`, `BCC`, and implemented
entirely in Rust, using only `libc` crate to execute system calls (`bpf()`).

Key Features:

1. **BPF Type Format (BTF) support**:
    Transparently enabled when supported by the Target kernel, allowing eBPF programs compiled against one
    kernel version to run on different kernel versions without recompilation. 

2. **Function calls and global data support**: 
    Enables `eBPF` programs to make function calls and use global variables with initializers. 

3. **Async support**:
    Compatible with Async executor `tokio` and `async_std` ( `smol`)

4. **Self-Contained:**
    No dependency on kernel headers or C tool-chains, this makes deployment easier and builds faster.

Reference: `README.md`, `aya/README.md`

The main entry point for user-space interactions is struct/type `Ebpf` ( for version 0.13.1, older version 
used to be `Bpf` ). This object is responsible for loading the eBPF Object file (bpf.o bytecode program), 
parsing the program and maps it contains, loading them into the kernel, and providing methods to interact
with them.

The `aya crate` has dedicated module for this `aya::programs` which contain structs and methods for all the
different `eBPF` program types ( ex: Kprob/Kretprob, Xdp, Tracepoings ) and for loading and attaching them
to the kernel hooks.
The `aya::maps` Contain types and methods for interacting with various `BPF map` types ( ex: HashMap, Array,
PerfEventArray, RingBuf, ) to exchange data between user-space and kernel space.

### Project Architecture: 

Aya consists of several interconnected components organized into 3 main groups:

![Aya Project Architecture](./img/01-Overview.png)

Sources: `Cargo.toml`, `aya/Cargo.toml`, `aya-obj/Cargo.toml`, `aya-log/Cargo.toml`, `aya-tool/Cargo.toml`

#### User Space Components:

- `aya` : The Core library providing the *main API* for:
    - Loading, 
    - Manipulating 
    - Managing `eBPF` programs

- `aya-obj` : Library for parsing `eBPF` object files with support for `BTF` and relocations.

- `aya-log` : A logging framework for `eBPF` programs running in the kernel 

#### `eBPF` Components: 

- `ebpf/aya-ebpf` : Runtime library for writing `eBPF` programs in Rust. 

- `ebpf/aya-ebpf-bindings` : Rust bindings for kernel types  used in eBPF programs. 

- `ebpf/aya-log-ebpf` : Library for logging from `eBPF` programs

#### Infrastructure Components:

- `xtask` : Build and development tools for Aya project.

- `test` : Framework for testing Aya functionality.



### `eBPF` Program LifeCycle:

The diagram shows how an `eBPF` programs flows from loading to executing:

![eBPF Program LifeCycle](./img/02-eBPF-Program-Lifecycle.png)

1. User loads an `eBPF` program using `Ebpf()::load` or `Ebpf::load_file`

2. The object is parsed by `aya-obj` to extract:
    - maps
    - programs 
    - `BTF` information 

3. Maps are created and programs are loaded via `BPF` system call `bpf()`

4. Programs are attached to kernel hooks ( ex: kprob, network driver, tracepoints.. ) 

5. Data exchange occurs between user-space and kernel-space via `maps`

Source :
    `aya/Cargo.toml:19-31`, `aya-obj/Cargo.toml:19-25`, `README.md:76-96`

### Supported Program and Map Type:

Aya supports a rich variety of program types and map structures:

![Supported Programs and Map Types](./img/03-SupportedPrograms-MapTypes.png)

For detailed information about:

- Different Program Types check [Program Types](./03-Aya-Program-Types.md)

- Map Types and data structures [Maps and DataStructures](./04-Aya-Maps-and-Data-Structures.md)

Sources: `README.md:76-96`, `aya/README.md:74-92`


### Logging System:

Aya provides a logging system for `eBPF` programs that allows log messages to be sent from kernel-space to
user-space. 

![Logging System](./img/04-Logging-System.png) 

For more details on Logging systems refer to [Logging System](./05-Aya-Logging.md)

Sources:  `aya-log/Cargo.toml:1-33`,`aya-log-common/Cargo.toml:1-23`,`aya-log-ebpf-macros/Cargo.toml:1-25`,
`aya-log-parser/Cargo.toml:1-24`

### Development and Testing:

Aya includes development, testing and Code generation:

![Development and Testing](./img/05-Development-Testing.png) 

For more info on development and testing : [Development and Testing](./06-Aya-Development-and-testing.md) 

Sources: `xtask/Cargo.toml:1-33`, `test/integration-test/Cargo.toml:1-50`

### Usage Example:

Below example demonstrates how to use Aya to load and attach an eBPF program:

```rust 
use std::fs::File;
use aya::Ebpf;
use aya::programs::{Cgroups, Cgroups, CgroupSkbAttachTyps, CgroupAttachMode} 

// Load the BPF Code:
let mut ebpf = Ebpf::load_file("ebpf.o");

//get the ingress_filter` program compiled into `ebpf.o`.
let ingress: &mut CgroupSkb = ebpf.program_mut("ingress_filter")?.try_into()?;

// load the program into the kernel
ingress.load()?;

// attach the program to the root cgroup. `ingress_filter` will be called for all
// incoming packets.
let cgroup = File::open("/sys/fs/cgroup/unified")?;
ingress.attach(cgroup, CgroupSkbAttachType::Ingress, CgroupAttachMode::AllowOverride)?;
```

For more detailed API documentation, see [API Reference](./07-API-Reference.md)

Sources: `README.md:76-96`, `aya/README.md:74-92`




