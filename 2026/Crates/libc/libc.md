# `libc` crate: Raw FFI bindings to platform's System libraries:


Ref:
    - https://docs.rs/libc/0.2.180/libc/
    - https://github.com/rust-lang/libc

Related  and useful crates:
- `bindgen`: The crate automatically generates Rust *FFI* bindings to `C` and `C++` libraries.
- `linux-raw-sys`: Generates bindings for Linux's user-space API.

## Introduction:

`libc` Crate is a raw *FFI* binding to platform libraries like `libc`.

This crate is a building block in Rust ecosystem. It provides **raw bindings** to standard C library (
often called `libc`) on various operating systems like Linux, macOS, and microsoft. 

In short this crate facilitates Rust code to talk directly to the Operating System's lowest level
infrastructure. 

- Rust's `std` library is powerful and designed to be safe and cross-platform, but some time programs
  need to :
  - Access a specific OS feature not available in `std` ( Eg: Specialized signal handling).
  - Call a C library that expects specific C type ( like `char *` or `int`).
  - Loding eBPF Bytecode: Rust program must make use of `syscall` (specifically `sys_bpf`).
    The `libc` crate provides the definitions for these syscall numbers and raw-structures needed to
    talk to the kernel.
    - Memory Mapping in eBPF: While using `Perf Buffers` or `Ring Buffers` to send data from kernel to
    Rust app, the user-space program might use `libc::mmap` to map that kernel memory into its own
    address space.
    - Shared Types: eBPF maps are the standard way to share data between eBPF kernel program and the
    user-space program. Sharing may demand a `struct` definition between kernel(eBPF) and userspace
    (Rust). And `libc` crate types ( like `libc::c_int`) in your user-space code to ensure that data
    alignment matches exactly what the kernel expects. 

Note:
---
As with above example for eBPF and libc, its to note that eBPF programs are complied to a specific `bpf` instruction set, not standard `x86` or `ARM`:
- Kernel has no standard library and has no access to `libc` and `libc` is a user-space library that
  provides a wrapper around system calls. 
- eBPF programs are extremely small and verifiable, they use a restricted set of "Helper Functions"
  provided directly by the kernel (ex: `bpf_map_lookup_elem`) and not Standard C functions like `printf` or `memset, malloc...`
---

### Key Characteristics: 

- **Unsafe** : Almost every function in `libc` is marked as `unsafe`, because Rust compiler can not
  guarantee `C` languages memory safety. 

- **No Ovehead**: These are direct FFI calls => there is no performance penalty for using them. 

- **Type Definition**: Provides aliases like `libc::c_int` to ensure your integers match the size of a `C` `int` on a specific target architecture.



## Usage:

Step 1: => Cargo.toml :
```toml 
[dependencies]
libc = "0.2"
```
Or run `cargo add libc` inside the project folder.
```bash
$  cargo add libc
    Updating crates.io index
      Adding libc v0.2.180 to dependencies
             Features:
             + std
             - align
             - const-extern-fn
             - extra_traits
             - rustc-dep-of-std
             - rustc-std-workspace-core
             - use_std
    Updating crates.io index
     Locking 1 package to latest Rust 1.92.0 compatible version
$ cat Cargo.toml
[package]
name = "libc_ex1"
version = "0.1.0"
edition = "2024"

[dependencies]
libc = "0.2.180"
```
    

Step 2: Calling a C function: 
To use `libc` wrap the call in `unsafe` block:
```rust 
use libc;

fn main () {
    // getpid() is a standard C function to get process ID of the program 
    let pid = unsafe { libc::getpid() };
    println!("Pid of this program: {}", pid);
}
```

Step 3. Passing data to libc:
While passing data to `libc`, do not use Rust std types `String` or `Vec`, instead use `libc` types to
ensure memory layout matches what C expects:
```rust 
use libc::{c_char, printf};
use std::ffi::CString;

fn main () {
    let message = CString::new("Hello from Rust via C printf!").expect("CString::new failed");
    unsafe {
        // printf(const char *format, ....)
        // .as_ptr() gives us the raw pointer C requires 
        printf(message.as_ptr());
    }
}
```

#### Common Type Mappings:
The `libc` crate ensures your code is portable by mapping Rust types to the correct `C` equivalents for
your machine:

| C Type | `libc` Type | Description |
| --- | --- | --- |
| `int` | `libc::c_int` | Usually a 32-bit signed integer. |
| `char` | `libc::c_char` | Platform dependent (signed or unsigned). |
| `void*` | `*mut libc::c_void` | A raw pointer to an untyped memory block. |
| `size_t` | `libc::size_t` | Unsigned integer for object sizes. |

Note:
---

- When to avoid `libc`: 
1. When you are doing general purpose Rust programming stay with `std`. 
2. When you require platform specific feature, consider using `nix` crate ( for unix like systems )
   instead of `libc`, `nix` wraps `libc` calls  inside safe wrappers so you don't have to write
   `unsafe` blocks manually.

---

## Complex example with `libc`:

Using eBPF and File descriptors, using `mmap` ( memory mapping ) :
A common low-level task where you map a file or a kernel buffer directly into your process's memory
address space:

Example: Memory Mapping a File with `libc`
```rust 
use libc::{mmap, munmap, open, fstat};
use libc::{PROT_READ, MAP_PRIVATE, O_RDONLY, fstat as libc_fstat};
use std::ptr;
use std::ffi::CString;

fn main () {
    let path = CString::new("test.txt").expect("CString Failed");

    unsafe {
        // 1. Open the file to get File descriptor (fd) 
        let fd = open(path.as_ptr(), O_RDONLY);
        if fd < 0 {
            panic!("Could not open file... Double check");
        }

        // 2. get file size ( needed for mmap )
        let mut stat: libc::stat = std::mem::zeroed();
        if libc::fstat(fd, &mut stat) < 0 {
            panic!("Could not get file stats");
        }
        let size = stat.st_size as usize;

        // 3. map the file into memory 
        // NULL: let the OS choose the address 
        // PROT_READ: we only want to read 
        // MAP_PRIVATE: Changes aren't written back 
        let data = mmap(
            prt::null_mut(),
            size,
            PROT_READ,
            MAP_PRIVATE, 
            fd,
            0
        );

        if data == libc::MAP_FAILED {
            panic!("mmap Failed...");
        }

        // 4. Access the data ( treating it like a slice )
        let slice = std::slice::from_raw_parts(data as *const u8, size);
        println!("File content from memory: {:?}", std::str::from_utf8(slice));

        // 5. Cleanup: Unmap and close
        munmap(data, size);
        libc::close(fd);
    }
}
```
- Systems Programming: In Shared memory: 
    You can use `mmap` with `MAP_SHARED` to let 2 different process talk to each other by writing to
    same memory block.

- eBPF interaction: 
    When Kernel write data to **Ring Buffer** user-space app uses similar `mmap` call to read that
    data directly from the kernel allocated page.

- **Zero Copy**: You avoid copying data from kernel buffer to a user-space buffer, critical for
  performance networking. ( Zero-Copy )

Note: - Key precautions to follow with using `libc`
---

1. **Alignment** : Some libc funs (those dealing with HW) require memory to be aligned to specific boundaries (ex: 4KB pages).
2. Error Handling: C funs usually return -1 or NULL on error. You must manually check these, whereas Rust's `std` would return a `Result`.
3. Memory Safety: Once you have a raw pointer from `mmap`, Rust can no longer track the lifetime.  You must ensure you don't use the pointer after calling `munmap`.

---

## More examples: 

1. Example: Handling Signals with `libc`:
Signals are simple way to interrupt a program execution: `libc` gives access to all signals (SIGHUP,
SIGUSR1, ...) and allow you to define complex handlers:


```rust 
use libc::{c_int, signal, SIGINT, SIG_DFL, SIG_IGN};
use std::sync::atomic::{AtomicBool, Ordering};

// A global flag to track if we received a signal
static GOT_SIGNAL: AtomicBool = AtomicBool::new(false);

// The actual C-compatible handler function
extern "C" fn handle_sigint(_sig: c_int) {
    GOT_SIGNAL.store(true, Ordering::SeqCst);
}

fn main() {
    unsafe {
        // Register our handler for SIGINT (Ctrl+C)
        // signal() returns the previous handler
        signal(SIGINT, handle_sigint as libc::sighandler_t);
    }

    println!("Waiting for Ctrl+C...");
    while !GOT_SIGNAL.load(Ordering::SeqCst) {
        // Do work...
    }
    println!("\nSignal received! Cleaning up...");
}
``` 
In system tools, you might need to reload a configuration file when receiving `SIGHUP`. 
`libc` is the only way to catch that specific signal. 


## Example Unix domain socket:

If your eBPF program is sending data to a local "collector" daemon, you likely want to use 
**Unix Domain Sockets**. 
They are faster than TCP/IP because they don't involve the network stack.

While Rust has `std::os::unix::net`, using `libc` directly reveals how the OS actually manages these connections 
via file descriptors and `sockaddr_un` structures.

### The "libc" Workflow for Sockets:

1. **`socket()`**: Create a new file descriptor for communication.
2. **`bind()`**: Assign a name (a file path) to the socket.
3. **`listen()` / `accept()**`: Wait for incoming connections.
4. **`send()` / `recv()**`: Transfer raw bytes.

---

## Important: The "extern" Keyword

You’ll notice the `extern "C"` in the signal example. 
This is critical when using `libc`. 
It tells the Rust compiler to use the **C Calling Convention**. 
Without it, the kernel or C library would try to jump to your function and find the arguments in the 
wrong CPU registers, leading to an immediate crash.
