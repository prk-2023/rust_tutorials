# Rust prelude:

The prelude is a set of modules that are automatically imported into every Rust program, making their
contents available without the need for explicit imports using the `use` keyword.


The prelude includes a number of fundamental types and traits, such as:

* `Option`
* `Result`
* `Vec`
* `String`
* `HashMap`
* `HashSet`
* `Box`
* `Rc`
* `Arc`
* `std::fmt::Display`
* `std::fmt::Debug`
* `std::cmp::PartialEq`
* `std::cmp::Eq`
* `std::hash::Hash`
* `std::iter::Iterator`
* `std::iter::IntoIterator`

These types and traits are considered essential to the Rust language and are used extensively throughout
the standard library.

By including them in the prelude, Rust makes it easy to write programs without having to worry about
importing them explicitly.

Complete list of modules and types included in the prelude in the Rust documentation:
[The Rust Prelude](https://doc.rust-lang.org/std/prelude/index.html).

Note that while the prelude includes many useful types and traits, it's still possible to import
additional modules and types using the `use` keyword if needed.

When programming for embedded Rust, it's common to exclude the prelude to reduce the size of the generated
binary and improve performance.

By default, the Rust compiler includes the prelude in every program.
However, you can control this behavior using the `no_std` attribute.

To exclude the prelude, you can add the following attribute to your crate's `lib.rs` file:
```rust
#![no_std]
```
This attribute tells the Rust compiler not to include the prelude in your program.

When you exclude the prelude, you'll need to import the types and traits you need explicitly using the
`use` keyword. For example:


```rust
use core::option::Option;
use core::result::Result;
```
Note that the `core` library is a subset of the `std` library that's designed for use in embedded systems.
It includes many of the same types and traits as the `std` library, but without the overhead of the `std`
library's dependencies.

Alternatively, you can use the `#![no_std]` attribute in combination with the
`#![feature(core_intrinsics)]` attribute to include only the core intrinsics, which are a set of
low-level functions that provide basic operations like arithmetic and memory management.

To include the prelude, you can simply omit the `#![no_std]` attribute.
However, keep in mind that this will include the entire `std` library, which may not be suitable for
embedded systems.

Here's an example of how you might structure your `lib.rs` file to exclude the prelude:
```rust
#![no_std]

use core::option::Option;
use core::result::Result;

// Your code here
```
And here's an example of how you might structure your `lib.rs` file to include the prelude:
```rust
// No attributes needed

// Your code here
```
Note that the `#![no_std]` attribute only affects the current crate, so if you're using a library that
includes the prelude, you'll still need to import the types and traits you need explicitly.

In summary:

* To exclude the prelude, use the `#![no_std]` attribute and import types and traits explicitly.
* To include the prelude, omit the `#![no_std]` attribute.
* Use the `core` library to access types and traits that are similar to those in the `std` library, but
  without the overhead of the `std` library's dependencies.

---
**Programs Packed with Rust binary**
---

When a Rust program is compiled, several other programs are included in the binary:

1.  **`std`**: 
    The Rust standard library is included in the binary. 
    However, the `std` library is designed to be highly modular, so only the parts of the library that 
    are actually used by the program are included in the binary.

2.  **`libc`**: 
    The C standard library is included in the binary, as Rust uses it for certain operations such as file 
    I/O and networking.

3.  **`libm`**: 
    The math library is included in the binary, as Rust uses it for certain mathematical operations.

4.  **`libgcc`**: 
    The GCC runtime library is included in the binary, as Rust uses it for certain operations such as 
    exception handling.

5.  **`libpthread`**: 
    The POSIX threads library is included in the binary, as Rust uses it for certain operations such as 
    threading.

6.  **`libdl`**: 
    The dynamic linker library is included in the binary, as Rust uses it for certain operations such as 
    dynamic linking.

7.  **`librt`**: The real-time library is included in the binary, as Rust uses it for certain operations 
    such as timing and scheduling.


---
The `#![no_std]` environment in Rust is used when you need to work in a **low-level** environment that 
doesn't have access to the Rust standard library (`std`). 

This is often the case in scenarios where you’re programming for embedded systems, operating systems, or 
other constrained environments, where the full standard library isn't available or practical due to the 
size, memory, or hardware limitations.

### When You Might Use `#![no_std]`:
Here are some typical scenarios where you would use a `#![no_std]` environment:

---

### 1. **Embedded Systems (Microcontrollers, IoT Devices)**
   - Many **embedded systems** run on microcontrollers (MCUs) or **single-board computers** that have
     limited resources (e.g., memory, processing power). These systems typically don't have an operating 
     system or full filesystem, so they can't use the Rust standard library, which depends on features like 
     dynamic memory allocation, file IO, and thread management.

   - In these cases, you would use `#![no_std]` and rely on `alloc` (which provides heap-allocated types) 
   and hardware-specific crates.
   
   **Example:**
   ```rust
   #![no_std]
   
   // Use embedded crates
   extern crate embedded_hal; // For hardware abstraction layers (HAL)

   fn main() {
       // Example: Simple program for an embedded system
       let led_on = true;
       if led_on {
           // Turn on LED
       }
   }
   ```

   - **Common Embedded Crates**: `embedded-hal`, `cortex-m`, `nrf52840-hal`, `stm32f4`, etc., are commonly
     used in no-std environments for controlling hardware.

---

### 2. **Operating System (OS) Development**
    - **OS development** requires direct control over hardware without relying on an OS (like Linux) or a
      runtime (like the standard library). 
      In this case, you write a custom OS that must be self-contained and not rely on standard library 
      features like threading, file I/O, and heap management that are part of `std`.

   - `#![no_std]` is essential because you’re writing code that will run directly on hardware, often
     starting with **bare-metal** programming or building a **kernel** for a custom operating system.

   **Example:**
   ```rust
   #![no_std]  // No standard library, we're writing a bare-metal OS
   
   // OS-related functionality
   pub fn kernel_main() -> ! {
       // Kernel setup code (e.g., setting up memory, I/O)
       loop {}
   }
   ```

   - **Common OS Development Crates**: `x86_64`, `bootimage`, `uart_16550`, `no_std_compat`, `bare-metal`.

---

### 3. **Real-Time Systems (RTOS)**
   - In **real-time operating systems** (RTOS) or **real-time applications**, where strict timing and
     resource constraints are critical, you often need to avoid the overhead of the Rust standard library.

   - RTOS environments typically need **predictable execution times** without the complexity of dynamic
     memory allocation or file I/O, both of which are provided by `std`.
   
   **Example:**
   ```rust
   #![no_std]
   
   // Real-time system logic, no heap or std functionality
   fn main() {
       // Periodic task execution without dynamic allocation
       loop {
           // Critical real-time code
       }
   }
   ```

   - **Common RTOS Crates**: `freertos-rust`, `rtic` (Real-Time Interrupt-driven Concurrency).

---

### 4. **WebAssembly (Wasm) with No-Std**
   - Although WebAssembly (Wasm) often relies on JavaScript or browser environments, you might use 
     `#![no_std]` if you want to target **bare-metal WebAssembly** environments or run your program in 
     **non-browser** contexts where minimal Rust runtime is desired 
     (e.g., Wasm runtimes or serverless environments).

   - In these cases, you might not need the full standard library and only need basic functionality with 
     some minimal external dependencies (e.g., memory, networking).

   **Example:**
   ```rust
   #![no_std]

   // WebAssembly logic with no dependencies on std library
   pub fn run() {
       // Low-level WebAssembly operations, memory access, etc.
   }
   ```

   - **Common WebAssembly Crates**: `wasm-bindgen`, `wasm-memory`, `wee_alloc`.

---

### 5. **Low-Level or Embedded Networking Applications**
   - Some networking protocols, especially in constrained environments like **low-power wide-area networks 
     (LPWAN)** or **Bluetooth Low Energy (BLE)** devices, might require custom networking code where the
     full functionality of `std` (e.g., TCP/IP stack, file I/O) is unavailable or unnecessary.
   - You might still need some memory allocation features provided by `alloc`, but `std` is generally 
     avoided to reduce size.

   **Example:**
   ```rust
   #![no_std]
   // Use crates for network protocols (e.g., Zigbee, LoRa, BLE)
   
   fn send_data() {
       // Send data over a custom network protocol
   }
   ```

---

### 6. **Bare-Metal Firmware Development**
   - In **bare-metal firmware** programming, you directly program the microcontroller without relying on 
     any OS or library. The firmware often needs to run in an environment with extremely limited resources,
     so avoiding `std` is essential.

   - You would often write the firmware in a minimal way, handling everything from low-level hardware 
     initialization to peripheral management.

   **Example:**
   ```rust
   #![no_std]
   #![no_main]
   
   // Minimal firmware code for a microcontroller
   fn _start() -> ! {
       // Firmware startup code (e.g., configuring peripherals)
       loop {
           // Main loop
       }
   }
   ```

   - **Common Bare-Metal Crates**: `cortex-m`, `riscv`, `stm32`, `no_std_compat`.

---

### 7. **Custom Memory Allocators (in No-Std Environments)**
   - When you are building software that involves custom memory allocation (e.g., in embedded systems, 
     OS kernels, or special-purpose hardware), you might need to implement a **custom allocator**.
     This requires a `#![no_std]` environment since the standard Rust allocator 
     (and `std`'s memory management features) won’t be available.

   **Example:**
   ```rust
   #![no_std]
   extern crate alloc; // Use alloc crate for heap allocation

   use alloc::vec::Vec;

   fn main() {
       let mut data: Vec<u8> = Vec::new();
       data.push(42);
   }
   ```

---

### 8. **Test Environments or Special Rust Toolchains**
   - Some **toolchains** or **testing environments** might specifically require `#![no_std]` to verify the
   behavior of code in minimal environments, or if the goal is to compile for a platform that lacks the
   Rust standard library. This is common in **cross-compilation** scenarios 
   (e.g., targeting ARM or RISC-V platforms from x86 systems).

   **Example:**
   ```rust
   #![no_std]
   
   // This crate is written for testing or verification purposes on constrained hardware
   fn test_functionality() {
       // Minimal function to verify behavior on a constrained platform
   }
   ```

---

### Conclusion:

The **`#![no_std]`** attribute is primarily used when you need to:

1. **Work with embedded systems** where resources (memory, processing power) are limited.
2. **Develop custom operating systems or real-time systems** that require direct hardware interaction 
   without the overhead of the standard library.
3. **Build bare-metal or low-level firmware** that doesn’t rely on any operating system or dynamic memory 
   allocation.
4. **Create WebAssembly (Wasm) or special-purpose applications** where you want minimal overhead.
5. **Write software that uses custom allocators or works in constrained environments** where the standard 
   library is not appropriate.

In these cases, you rely on `alloc` for heap-allocated types (like `Vec` and `String`), but avoid `std` to 
reduce size and dependencies.
