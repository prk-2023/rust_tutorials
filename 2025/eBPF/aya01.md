# Aya 

## Aya's Architectural Distinction

Aya is fundamentally different from traditional eBPF tooling because of its pure **Rust** implementation 
and minimal dependencies, leading to better operability and developer experience.

* **No `libbpf` or `bcc` Reliance:** 
    Aya does not depend on the established C libraries like `libbpf` (the official in-kernel library) or 
    the Python-based `bcc` framework. This eliminates the need for C toolchains, kernel headers, and complex 
    build environments, making it faster and easier to deploy.
    
* **Pure Rust Implementation:** 
    The library is built entirely in Rust, which is used for both the **user-space application** and the 
    **eBPF program** itself (via the `aya-ebpf` crate). This allows for easier code sharing and a unified 
    development experience.

* **Direct Kernel Interaction:** 
    Instead of relying on an intermediary C library, the user-space portion of Aya directly interacts with 
    the Linux kernel's eBPF subsystem. This is achieved using the **`libc` crate** to execute the raw 
    **`bpf()` system call** for all essential operations, including:

    * Loading eBPF object files (programs and maps).
    * Creating and managing eBPF maps.
    * Attaching eBPF programs to various hook points (e.g., kprobes, tracepoints, XDP).

This design allows Aya to offer a **"Compile Once, Run Everywhere" (CO-RE)** solution, especially when 
linked with **musl**, as the resulting binary is self-contained with minimal dependencies.

### Comparison of eBPF User-Space Tooling

| Feature | Aya (Rust) | `libbpf` (C) / `libbpf-rs` (Rust bindings) | `bcc` (Python/C++) |
| :--- | :--- | :--- | :--- |
| **Language** | Pure Rust | C / Rust (bindings) | Python (Frontend) / C++ (Backend) |
| **Dependency on `libbpf`** | **No** (Uses `libc` for direct syscalls) | Yes (It *is* `libbpf` or a wrapper) | No (Uses LLVM/Clang for JIT) |
| **Toolchain Requirement** | Only Rust | Requires C toolchain | Requires Python, LLVM/Clang |
| **Deployment** | Single, self-contained binary (especially with musl) | Requires `libbpf` shared library on target | Requires runtime dependencies (Python, etc.) |

