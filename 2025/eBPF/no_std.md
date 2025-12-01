# no_std

## Introduction: 
Rust `no_std` attribute is a **crate-level flag** (`#![no_std]`) that tells the Rust Compiler *not to
automatically link the standard library (`std`).

W.R.T eBPF where the program get execute within the Linux kernel, The environment is fundamentally
**bare-metal** and does not provide the services expected by the standard Rust library (`std`) 

By default every Rust program links against the full `std` library, which provides a rich set of features
like:

1. Networking and File I/O : 
    Functionality for interacting with the OS environment.

2. Threading: 
    Concurrency Primitives

3. Dynamic Memory Allocation (Heap) : 
    Structures like `Vec`, `String` and `HashMap`

4. Runtime Support: 
    Setting up stack overflow protection, processing command line arguments, and calling the main function.

The problem is that the `std` library assumes your program is running on a full-featured OS like Linux,Unix
or m-soft, macOS.

However many environment do not have an OS or typical services:

1. Embedded Systems/Microcontrollers: 
    Tiny devices ( with limited memory where every byte counts and there is on OS. )
2. OS Kernels and Bootloaders: 
    Code that runs before an OS is fully initialized or which is the OS.
3. WebAssembly: (WASM) 
    Environment that may not have traditional OS APIs.

`no_std` allows Rust to be used for bare-metal programming.

### What does #![no_std] have:
Access to minimal, platform-agnostic parts of Rust:

- The `core` library: This is the foundational lib of Rust. It contains the absolute minimum necessary for
  the language to function, including:
    * Primitive types (`i32`, `f64`, `bool`, `slices &[]`).
    * Fundamental traits (`Copy`, `Clone`, `Debug`). 
    * Essential type systems like `Option` and `Result`.
    * Basic iteration and error handling logic.
    * Module paths start with `core::` instead of `std::` .

- The `alloc` crate (Optional): 
    This crate contains the parts of the standard library that rely on dynamic memory allocation(the
    heap),but without any other OS-specific features.
    * It provides dynamic data structures like `Vec<T>`, `String`, `Box<T>`, and `HashMap` (via the `alloc`
      feature).
    * To use `alloc` in a no_std environment, you must explicitly provide and register a global memory
      allocator yourself, as there is no OS to do it for you.

#### Requirements for `no_std` Binaries:

When creating a complete executable using `no_std` you are responsible for providing things the `std`
library usually handles:

1. `#![no_main]` : You often need this attribute to disable the default Rust run times entry point, allowing
   you to define your own low-level entry function (like an extern "C" `fn _start`).

2. **Panic Handler**: You must define a function marked with `#[panic_handler]` to specify what happens when
   your program encounters an unrecoverable error (ex: printing a message to a serial port or simply looping
   indefinitely).

3. **Memory Layout**: If using `alloc`, you must define a global memory allocator using `#[global_allocator]`


### - Example A minimal "Hello, world!" 

A minimal example showing the necessary setup for a `no_std` bare-metal application.
( This is code is for bare-metal targets ( like ARM micro controllers)) 
The code shows the four essential component the programmer has to provide when giving up `std` library:

- main.rs:

```rust
// 1. Disable the standard library
#![no_std]

// 2. Disable the default 'main' function entry point
#![no_main]

use core::panic::PanicInfo;

// This is the actual entry point of the application.
// We use a custom entry point attribute provided by a helper crate (like 'cortex-m-rt'
// for ARM systems, or 'bootloader-rs' for an OS kernel).
// For simplicity, we'll define a simple infinite loop here.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // This is the "main" logic.
    // Notice we can still use fundamental types and core functions.

    let message: [u8; 13] = [
        b'H', b'e', b'l', b'l', b'o', b',', b' ', b'w', b'o', b'r', b'l', b'd', b'!',
    ];

    // In a real bare-metal program, you would interact with hardware registers
    // here, perhaps to send this message over a serial (UART) connection.
    // Since we have no standard I/O, we can only simulate the action.
    // println! is UNAVAILABLE.

    // A simple, non-returning loop is standard for embedded/kernel applications.
    loop {
        // Do nothing (wait for an interrupt or a watchdog timer reset)
    }
}


// 3. The Panic Handler
// Since there's no OS to catch a panic, we must define this function.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // In a real application, you might:
    // 1. Log the panic info to a debug port.
    // 2. Turn on an LED to signal an error state.
    // 3. Reset the system.
    
    // For this minimal example, we just enter an infinite loop.
    loop {}
}
```
- `#![no_std]` & `#![no_main]` : these two attributes are the core of the **bare-metal programming
  contract** 
    * `#![no_std]` prevents the compiler from linking the large, OS-dependent `std` library. You are left
      with `core` library.
    * `#![no_main]` disables the automatic insertion of the standard Rust runtime, which normally sets up
      the stack, calls the main function and handles cleanup.

- The custom entry point (`_start`) : 
```rust
#[no_mangle]
pub extern "C" fn _start() -> !
```
    * when you use `#![no_main]`, you must provide the function where execution begins. By convention, this
      named `_start`. 

    * `#[no_mangle]` tells the compiler not to "mangle" the function name, ensuring the linker can find it
      by its simple `_start` name.

    * `extern "C"` specifies the C calling convention, which is what the hardware boot process or initial
      linker script expects. 

    * The function returns `!` ( a divergent function), meaning it **never returns** ( it typically loops
      forever)

- **The panic Handler** :
    ```rust 
    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! { ... }
    ```
    In a standard Rust application, a panic (like any array index out of counds) is caught by the `std`
    library, which cleans up the stack, prints an error message, and exits the process.

    In a `no_std` environment You are the final authority. The `#[panic_handler]` attribute marks the
    function that the compiler will jump to when a panic occurs. If you don't provide this the compilation
    will fail. 

=> Minimal setup demonstrates that `no_std` is not just about excluding the library; it's about **taking
ownership** of the low-level runtime environment, memory management, and error handling.
    

### - `no_std` for eBPF:

`no_std` is mandatory in eBPF programs:
- No OS : 
    The eBPF program runs inside the kernel, not on top of it. 
    Therefore, there is no underlying OS to provide file I/O, networking sockets, or threading functions
    that std relies upon.

- NO Dyanamic Allocation (Heap) : 
    Standard `std` structures like `Vec` or `String` require **dynamic memory allocation** (the heap).
    The eBPF runtime strictly forbids dynamic memory allocation because it is non-deterministic and can 
    lead to memory exhaustion or unpredictable behavior inside the kernel.

- No Standard Runtime:
    The `std` library requires a specific runtime to initialize things like stack overflow protection and 
    global data structures. 
    The eBPF environment does not support this runtime; it only allows a simple, direct entry point.

- Verifier Restrictions	:
    Before execution, the eBPF Verifier checks the program for safety. 
    It ensures the program terminates, doesn't access invalid memory, and doesn't call unauthorized funs. 
    The complex, non-deterministic operations in `std` would cause the Verifier to reject the program.

=> When writing eBPF in Rust with `no_std` you are restricted to:
    - The `core` library. 
    - Data structures that live entirely on the **stack** ( ex: arrays,vecotors, simple structures)
    - eBPF-specific "Helper Functions" ( which act like system calls for the eBPF runtime) and eBPF Maps (
      for communication with user-space)

For Typical eBPF project in Rust you would use a dedicated crate that provides the necessary bindings and
helper for the eBPF environment, such as the `aya` or `redbpf` crates, which are themselves built on top of
`no_std`.

## Rust eBPF programs:

Rust eBPF program using the `aya-ebpf` crate. 

Flow is split into two main components :

1. **eBPF Program** Kernels user-space 
2. **User space program** Loader.

Aya simplifies this process of by providing the necessary macros and a unified development experience using 
Rust for both the kernel and user space components.

### Kernel Space eBPF program:

Code runs inside the kernel's eBPF Virtual Machine:

Steps: Requirement/Code Snippet,Purpose/Rationale

1. Disable std => `#![no_std]` 
    The eBPF VM environment does not provide the OS services needed by the Rust Standard Library (std). 
    Only core is allowed.

2. Disable main => `#![no_main]`
    The program is event-driven and has no default OS entry point. 
    The actual entry point is defined by an *eBPF macro.*

3. Provide Panic Handler => 

    "Crucial for `no_std`. While a panic shouldn't happen, Rust requires a handler. The implementation must 
    be non-unwinding and minimal. `unreachable_unchecked()` or an infinite loop is the standard, safest 
    practice in eBPF to stop execution immediately."

    ```rust 
    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        // Safest way to abort execution in the eBPF VM context
        unsafe { core::hint::unreachable_unchecked() }
    }
    ```
    This specific implementation using `core::hint::unreachable_unchecked()` is the standard, minimal, and
    safest way to satisfy the compiler's requirement for panic handler while guaranteeing that the eBPF
    program does not attempt complex or illegal operations ( such as stack unwinding ) that would cause the
    kernel verifier to reject it. 

4. Define Entry Point => 
    `rust use aya_ebpf::macros::xdp; #[xdp] pub fn myapp(ctx: XdpContext) -> u32 { /* ... logic ... */ }`
    
    "A macro from `aya-ebpf` (like #[xdp], #[tracepoint], etc.) marks the function that the kernel should 
    call when the associated event (the ""hook"") occurs."

    ```rust 
    use aya_ebpf::macros::xdp;
    
    #[xdp]
    pub fn myapp(ctx: XdpContext) -> u32 {
        // This pattern is highly recommended for stability and verifier compatibility
        match try_myapp(ctx) {
            Ok(ret) => ret,
            Err(_) => xdp_action::XDP_ABORTED, // e.g., using a constant for a safe exit
        }
    }
    ```

5. Core Logic & Safety,

    `rust match try_myapp(ctx) { Ok(ret) => ret, Err(_) => xdp_action::XDP_ABORTED, }`

    "Best Practice: The entry function (e.g., myapp) typically delegates the core logic to a separate 
    function (e.g., try_myapp) that returns a Result. This allows you to handle recoverable errors 
    gracefully (e.g., returning an eBPF action like XDP_ABORTED or 0 for success) without causing a panic."

6. Build Target
    `cargo build --target bpfel-unknown-none`,

    The program must be compiled for the eBPF target architecture (**bpfel-unknown-none** for little-endian 
    kernels, which is common). The output is a .o object file containing the eBPF bytecode.

### User-Space loader:

This is a normal Rust application that runs on the host OS. Its job is to manage, load, and communicate with
the eBPF program in the kernel.

Steps :

1. Standard Setup :

    Normal `fn main()` and `use std::...`,This is a regular Rust application and uses the full `std` lib.

2. Load Bytecode:

    `rust let mut bpf = Ebpf::load_file(""myapp.bpf.o"")?;` 

    Uses the aya crate to read the compiled eBPF object file (`myapp.bpf.o`).

3. Initialize Logger => 

    `EbpfLogger::init(&mut bpf)?;` 

    Sets up communication to receive logging/debug messages (using `aya_log_ebpf::info!` in the kernel 
    program) from the kernel program.

4. Get Program Reference

    `let prog: &mut Xdp = bpf.program_mut(""myapp"")?.try_into()?;` 

    Retrieves a reference to the specific eBPF function (myapp) loaded from the object file.

5. Load & Attach 

    `prog.load()?; prog.attach(""eth0"", XdpFlags::default())?;`

    Loading sends the bytecode to the kernel's verifier. 
    Attaching links the program to its designated kernel hook point 
    (ex: the eth0 network interface for an XDP program).

6. Wait & Cleanup => 

    `signal::ctrl_c().await; drop(bpf);`

    The user-space program must stay running to keep the eBPF program attached. 
    When it exits (ex., on Ctrl+C), the `drop(bpf)` (or simply the scope ending) automatically detaches and
    unloads the eBPF program from the kernel.


