# FFI: Bridge between Rust and other languages.

Foreign Function Interface (FFI) in Rust is the bridge that allows Rust code to interact with other
languages, primarily C and C++. For **systems, embedded, and automotive programming**, FFI is not just a
feature, but it's a requirement for talking to hardware abstraction layers (HALs), real-time OS (RTOS)
and legacy safety-critical libraries.  

## 1. Core Mechanisms: The "Foreign" Bridge

To interface with C (Which is most common in embedded/automotive), Rust requires three core elements:

* `extern "C"`: 
    Tells the Rust compiler to use the C Application Binary Interface (ABI). This ensures that
    function arguments and return values are passed in the exact registers and stack locations that C
    expects. 

* `#[no_mangle]`: 
    Prevents Rust from renaming your functions during compilation (mangling). Without this, a C
    linker wouldn't be able to find a function named `reset_ecu` because Rust might have renamed it
    to `_ZN9reset_ecu7h12345E`.

* `unsafe`: 
    Every call to a foreign function is wrapped in an `unsafe` block. Since the Rust compiler cannot verify the memory safety of C code, the programmer must manually guarantee that the call is safe.

---

## 2. Data Layout and Types

In embedded systems, memory layout is everything. 
You cannot pass a Rust `String` or `Vec` to C.

### Type Mapping

Use the `core::ffi` (for `no_std`) or `std::ffi` types to ensure bit-compatibility:

| C Type | Rust FFI Equivalent |
| :--- | :--- |
| `int` | `core::ffi::c_int` (usually `i32`) |
| `uint32_t` | `u32` |
| `char *` | `*mut core::ffi::c_char` |
| `void *` | `*mut core::ffi::c_void` |

### Struct Alignment (`#[repr(C)]`)

To ensure a Rust struct matches a C struct exactly (including padding), you must use the C representation
attribute:

```rust
#[repr(C)]
pub struct SensorData {
    pub id: u32,
    pub value: f32,
}

```

---

## 3. Automation Tools for Systems

Manually writing bindings is error-prone, especially for massive automotive SDKs (like AUTOSAR or STM32 HALs).

* **`bindgen`**: Automatically scans C header files (`.h`) and generates the equivalent Rust `extern`
  blocks and structs.
* **`cbindgen`**: The reverse—generates C headers from your Rust code so your C-based RTOS can call Rust
  functions.
* **`cc` crate**: A build-time dependency used in `build.rs` to compile C/C++ source files and link them
  into your Rust binary automatically.

---

## 4. Automotive & Embedded Special Requirements

In automotive (ISO 26262) and embedded contexts, FFI has stricter guardrails:

### No Standard Library (`no_std`)

Most embedded FFI must work without an OS. You must use `core::ffi` instead of `std::ffi`.

```rust
#![no_std]
use core::ffi::c_int;

extern "C" {
    fn hw_init(port: c_int) -> c_int;
}

```

### Opaque Pointers (Encapsulation)

To handle complex C objects without exposing their internals to Rust, use opaque pointers:

```rust
// In Rust, define an empty enum to represent the C type
#[repr(C)]
pub struct CanDriver { _private: [u8; 0] }

extern "C" {
    fn can_send(driver: *mut CanDriver, id: u32);
}

```

### Panic Handling

Crucial: A Rust `panic!` must never cross the FFI boundary into C. Doing so causes "Undefined
Behavior" (UB), which in an automotive setting could lead to a system crash. Always use `catch_unwind` 
( if available) or strict error codes.

---

## 5. Summary Table for Systems Engineers

| Requirement | Tool / Feature | Why? |
| --- | --- | --- |
| **Hardware Access** | `extern "C"` | Call vendor-provided C drivers/HALs. |
| **Memory Safety** | Safe Wrappers | Hide `unsafe` FFI calls behind a safe Rust API. |
| **Build Integration** | `build.rs` + `cc` | Link legacy C code into the Cargo build process. |
| **Certification** | Ferrocene | Use a qualified Rust compiler for ASIL-D automotive standards. |


## Rust build.rs script
Next: Example of a `build.rs` script that compiles a C driver and links it to a Rust project:

### Example of a build.rs script to compile C driver and link to Rust project:

To bridge Rust with C code in a systems environment, you need a **Build Script** (`build.rs`). 

This script runs before your code compiles, acting as the "glue" that compiles C source files and tells the Rust linker where to find the resulting binary blobs.

#### 1. The Project Structure

For an embedded project, your directory should look like this:

```text
my_embedded_project/
├── Cargo.toml
├── build.rs          # The build script
├── src/
│   └── main.rs
└── c_src/            # Your legacy/vendor C code
    ├── driver.c
    └── driver.h

```

---

#### 2. The Build Script (`build.rs`)

You will use the `cc` crate (added to `[build-dependencies]` in `Cargo.toml`) to automate the compilation.

```rust
// build.rs
fn main() {
    // 1. Compile the C code into a static library named "libdriver.a"
    cc::Build::new()
        .file("c_src/driver.c")
        .include("c_src")
        // Optional: Add safety flags for automotive/embedded
        .flag("-Wall")
        .flag("-Wextra")
        .compile("driver");

    // 2. Tell Cargo to rerun this script only if the C files change
    println!("cargo:rerun-if-changed=c_src/driver.c");
    println!("cargo:rerun-if-changed=c_src/driver.h");
}

```

---

#### 3. The Rust Implementation (`src/main.rs`)

Once the build script is ready, you define the "Safe Wrapper." 
In automotive systems, you never want the end-user (or other modules) to touch `unsafe` code directly.

```rust
// src/main.rs
#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Define the raw interface to C
extern "C" {
    /// Initialises a hardware peripheral. Returns 0 on success.
    fn hw_peripheral_init(id: u32) -> i32;
}

// THE SAFE WRAPPER
// This is what makes Rust valuable in systems programming.
pub fn init_peripheral(id: u32) -> Result<(), &'static str> {
    let result = unsafe { hw_peripheral_init(id) };
    
    if result == 0 {
        Ok(())
    } else {
        Err("Hardware failed to initialize")
    }
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    let _ = init_peripheral(1);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

```

---

#### 4. Key Considerations for Automotive (ISO 26262)

When moving from standard FFI to safety-critical FFI, keep these three rules in mind:

1. Static Analysis:
    Use tools like `bindgen` to ensure your Rust structs and C structs have the exact same memory offset.

2. Stack Overflow:
    C functions called via FFI do not share Rust's stack-guard information. You must manually ensure
    the C function won't blow the stack of your Rust thread/task.

3. No Exceptions: If your C code uses a library that can "throw" or "longjmp," it will likely crash 
crash the Rust runtime. Stick to simple return-code based C libraries.

-------------------


## `bindgen` : create automatic bindings

Next: Generate a `bindgen` configuration that automatically creates these bindings from a header file?

Using `bindgen` is the industry standard for automotive and embedded systems because it eliminates the "human error" of manually typing out C structs and function signatures in Rust.

### 1. The Setup

First, add the necessary tools to your `Cargo.toml`. Since `bindgen` is a build-time tool, it goes under `[build-dependencies]`.

```toml
[package]
name = "embedded-driver"
version = "0.1.0"
edition = "2021"

[dependencies]
# No-std compatible types
core = { version = "1.0", optional = true }

[build-dependencies]
bindgen = "0.69" 

```

---

### 2. The Automated `build.rs`

This script does two things: it compiles your C code (using `cc`) and then generates Rust bindings (using `bindgen`).

```rust
use std::env;
use std::path::PathBuf;

fn main() {
    // 1. Tell Cargo to link the hardware abstraction library
    println!("cargo:rustc-link-lib=static=hw_driver");
    println!("cargo:rerun-if-changed=wrapper.h");

    // 2. Configure bindgen
    let bindings = bindgen::Builder::default()
        // The header we want to generate bindings for
        .header("c_src/wrapper.h")
        // Ensure it works in embedded (no_std)
        .use_core()
        // Prevent generating layout tests (often fails in cross-compilation)
        .layout_tests(false)
        // Automotive standard: Ensure C enums are handled as Rust enums or constants
        .default_enum_style(bindgen::EnumVariation::Rust { non_exhaustive: true })
        .generate()
        .expect("Unable to generate bindings");

    // 3. Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}

```

---

### 3. The "Wrapper" Header

In FFI, we often create a `wrapper.h` file. This acts as a gateway, including all the specific vendor headers (like `stm32_hal.h` or `autosar_can.h`) you want to expose to Rust.

```c
// c_src/wrapper.h
#include "driver.h"
#include "sensor_api.h"

```

---

### 4. Using the Generated Code

In your `src/lib.rs` or `src/main.rs`, you include the generated code using the `include!` macro. This pulls in the thousands of lines of code `bindgen` just wrote for you.

```rust
#![no_std]

// Include the auto-generated bindings
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

/// A high-level safe function for a junior engineer to use
pub fn safe_sensor_read() -> u32 {
    let mut val: u32 = 0;
    unsafe {
        // 'raw_c_sensor_read' was automatically generated by bindgen
        raw_c_sensor_read(&mut val as *mut u32);
    }
    val
}

```

### Why this is essential for Automotive:

* Bit-Accuracy:** If a C struct has a 2-byte hole due to alignment, `bindgen` detects it and add
  `_unused` padding bytes automatically.

* Maintenance: If the silicon vendor updates their C HAL, you simply re-run the build. 
  You don't have to manually update 500 function signatures.
* Transparency: You can inspect the generated `bindings.rs` in your `target/` folder to verify how the
  compiler sees the C data.


## Handle C call-backs ( interrupts )

How to handle C callbacks (interrupts) where a C function needs to trigger a Rust func?

In embedded and automotive systems, handling **callbacks** (like an Interrupt Service Routine or a CAN

In embedded and automotive systems, handling **callbacks** ( like a Interrupt service routine or a CAN
bus event) is one of the most complex parts of FFI. You are essentially teaching the C runtime how to
"jump" back into the Rust universe. 

### 1. The Challenge: Mismatched Paradigms

C usually handles callbacks by passing a **function pointer** and a `void*` (context pointer). Rust needs
to ensure that when C calls that pointer:

1. The Rust function is compatible with the C ABI.
2. The Rust function doesn't "panic" (which would crash the hardware).
3. Any data passed back is handled safely.


### 2. Step 1: Defining the Extern "C" Callback

You must mark your Rust function as `extern "C"` so it follows the calling convention the C interrupt
controller expects.

```rust
/// This function is what C will actually execute.
/// We use #[no_mangle] so the C linker can see it by name if needed.
#[no_mangle]
pub extern "C" fn rust_can_callback(packet_id: u32, data_ptr: *const u8) {
    // Safety: We must ensure we don't panic here!
    // In an ISR, we usually move data into a thread-safe buffer.
    let _id = packet_id;
    // Process data...
}

```

### 3. Step 2: Registering the Callback with C

Usually, a C HAL will have a registration function like `register_callback(void (*fn)(u32, uint8_t*))`.

```rust
// The binding generated by bindgen or manual extern
extern "C" {
    fn register_hw_interrupt(callback: extern "C" fn(u32, *const u8));
}

pub fn init_system() {
    unsafe {
        // We pass the Rust function pointer directly
        register_hw_interrupt(rust_can_callback);
    }
}

```

---

### 4. Step 3: Handling Context (The `void*` Pattern)

In sophisticated systems, you don't want a global function; you want the callback to access a specific
objects. C handles this with a "user data" pointer.

The C Signature:

```c
void set_callback(void* user_data, void (*fn)(void* user_data, int event_id));

```

The Rust Implementation:

```rust
#[repr(C)]
struct MyDriver {
    status: u32,
}

// This wrapper bridges the C void* back to a Rust struct
extern "C" fn trampoline(user_data: *mut core::ffi::c_void, event_id: i32) {
    // CRITICAL: Cast the raw pointer back to our Rust type
    let driver = unsafe { &mut *(user_data as *mut MyDriver) };
    driver.status = event_id as u32;
}

pub fn setup() {
    let mut my_driver = MyDriver { status: 0 };
    unsafe {
        set_callback(
            &mut my_driver as *mut _ as *mut core::ffi::c_void, 
            trampoline
        );
    }
}

```

---

### 5. Essential Rules for Automotive Callbacks

* The Trampoline Pattern: As shown above, use a small `extern "C"` function (the trampoline) to convert
  raw C arguments into nice Rust types before calling your main logic. 

* No Spontaneous Allocation: In an ISR (Interrupt Service Routine) context, never use `Box` or `Vec`.
  Your callback must be `no_std` and *heapless*.

* Static Lifetimes: Ensure the data pointed to by `user_data` lives at least as long as the hardware
  interrupt active. If `MyDriver` is dropped while the interrupt is still armed, you get a  "Use-After-Free" 
  at the hardware level—the hardest bug to debug in automotive.

Next: Use `RefCell` or `Atomic` types to safely share data between these C interrupts & your main Rust loop.


## Use `RefCell` / `Atomic` types to safely share data between C interrupts and Rust main loop:

In systems and eBPF/automotive prog, data often lives in two worlds: the Main Loop (Rust) and the Interrupt
service routine/callback (C). Because an interrupt can "preempt" (pause) your main code at any microsecond,
you cannot use standard variables.

You must use **Synchronization Primitives** that are safe for `no_std` environments.

---

### 1. The Atomic Pattern (For Simple Flags/Counters)

If you only need to share a status flag (e.g., "Is the CAN message ready?") or a counter, 
**Atomics** are the fastest and safest way. 
They don't require a heavy Mutex and are hardware-supported to prevent data races.

```rust
use core::sync::atomic::{AtomicBool, Ordering};

// Global state visible to both Interrupts and Main Loop
static DATA_READY: AtomicBool = AtomicBool::new(false);

#[no_mangle]
pub extern "C" fn hardware_interrupt_handler() {
    // Set the flag to true using 'Relaxed' or 'Release' ordering
    DATA_READY.store(true, Ordering::Relaxed);
}

pub fn main_loop() {
    loop {
        if DATA_READY.load(Ordering::Relaxed) {
            // Process data...
            DATA_READY.store(false, Ordering::Relaxed);
        }
    }
}

```

### 2. The `Critical Section` Pattern (For Complex Structs)

If you need to share a whole `struct` or a buffer, you need a way to ensure the Main Loop isn't halfway through reading the data when an interrupt triggers. In `no_std`, we use a combination of `RefCell` and a `Mutex` from a crate like `critical-section`.

```rust
use core::cell::RefCell;
use critical_section::Mutex;

struct SharedData {
    temperature: i32,
    fault_code: u16,
}

// A global, thread-safe container for our struct
static SHARED_SENSOR: Mutex<RefCell<Option<SharedData>>> = Mutex::new(RefCell::new(None));

#[no_mangle]
pub extern "C" fn sensor_callback(temp: i32, fault: u16) {
    // Access the global data safely
    critical_section::with(|cs| {
        let mut sensor = SHARED_SENSOR.borrow(cs).borrow_mut();
        *sensor = Some(SharedData { temperature: temp, fault_code: fault });
    });
}

```

---

### 3. Comparison of Sharing Strategies

| Strategy | Best For... | Overhead | Safety |
| --- | --- | --- | --- |
| **Atomics** | Flags, Counters, State Enums | Near Zero | Hardware-guaranteed |
| **Critical Section** | Structs, Arrays, Buffers | Medium (Disables interrupts) | Software-guaranteed |
| **Lock-Free Queues** | Streaming data (e.g., UART/CAN logs) | Low | Complex to implement |

---

### 4. Automotive Safety: The "Memory Barrier"

In automotive ECUs (Electronic Control Units), the compiler or the CPU might try to reorder your instructions to save time.

* **Rust's `Ordering`:** When using Atomics, `Ordering::SeqCst` (Sequentially Consistent) is the "safest" but slowest. In safety-critical systems, we often use it to ensure the CPU doesn't process a sensor value *before* the "Data Ready" flag is actually set.
* **Volatile Access:** If you are reading/writing directly to a memory address controlled by C hardware, always use `core::ptr::read_volatile` and `write_volatile`.

---

### 5. Summary Checklist for FFI in Automotive/Systems

1. **ABI:** Is it `extern "C"`?
2. **Layout:** Is it `#[repr(C)]`?
3. **Panic:** Did I use a "trampoline" to catch panics before they hit C?
4. **Sync:** Am I using `Atomic` or `Mutex` for any data touched by an interrupt?
5. **Opaque:** Am I hiding C pointers behind a safe Rust struct?

Next: Handle "Strings" and "Arrays" specifically, as they are handled very differently in C (null-terminated) vs Rust (length-based)**

## Handle String and Arrays between two different worlds:

In **eBPF**, the two worlds are the **Kernel** (compiled from C or restricted Rust) and **Userspace**
(your Rust control application). Strings and arrays are the primary way they communicate through "Maps."

### 1. Strings: Nulls vs. Lengths

C strings are **pointers** ending in `\0`. Rust strings are **slices** with a pointer and an explicit length.

* Rust to C:
    You must add a null terminator. Use `std::ffi::CString` (if you have `std`) or a fixed-size bytes
    with a trailing `0` in `no_std`.

* C to Rust: You must "measure" the string or use a fixed buffer size.

```rust
// eBPF example: Setting a program name in a C-style map
let name = "packet_filter\0"; // Hardcoded null-terminator
unsafe {
    bpf_map_update_elem(map_fd, &key, name.as_ptr(), BPF_ANY);
}

```

### 2. Arrays: Contiguous Memory

Both C and Rust store arrays contiguously, making them highly compatible. However, eBPF usually requires 
**fixed-size arrays** because the Kernel cannot handle dynamic resizing.

* C Side: `char buffer[128];`
* Rust Side: `buffer: [u8; 128]`

To pass a Rust array to a C function, you simply pass the pointer to the first element.

---

### 3. The eBPF "Helper" Pattern

In eBPF, you often use helpers like `bpf_probe_read_str` to pull strings from Kernel space into your Rust probe.

```rust
// Inside an eBPF probe (Rust)
let mut task_name: [u8; 16] = [0; 16];
unsafe {
    // Helper to copy a null-terminated string from kernel to our buffer
    bpf_probe_read_str(
        task_name.as_mut_ptr() as *mut c_void,
        task_name.len() as u32,
        kernel_ptr
    );
}

```

### Summary Table

| Feature | C (eBPF Kernel) | Rust (Userspace/Aya) | How to Bridge |
| --- | --- | --- | --- |
| **String** | `char*` (Null-terminated) | `&str` / `String` (Length) | `CStr::from_ptr` or `CString` |
| **Array** | `int arr[10]` | `[i32; 10]` | Identical memory layout |
| **Safety** | Trust the verifier | Trust the `unsafe` block | Use `repr(C)` on all structs |


Next: Example Shared `struct` defined in Rust that is used in eBPF Maps keys:

## eBPF : Share Rust struct with eBPF maps keys:

In **eBPF**, the "Contract" between the Rust userspace and the C kernel probe is a shared memory 
structure, usually defined in a common module or crate.

### 1. The Shared Struct

You must use `#[repr(C)]` to ensure the Rust compiler doesn't reorder fields, which would cause the 
Kernel to read the wrong data.

```rust
#[repr(C)]
#[derive(Clone, Copy)]
pub struct PacketLog {
    pub ipv4_address: u32,
    pub port: u16,
    pub protocol: u8,
    pub _padding: u8, // Manual padding to align to 4-byte boundary
}

```

### 2. The Kernel Side (C / eBPF)

The Kernel uses this struct to write data into a **Babel Map** (Hash or Perf Event Array).

```c
struct packet_log {
    uint32_t ipv4_address;
    uint16_t port;
    uint8_t  protocol;
    uint8_t  _padding;
};

SEC("maps")
struct bpf_map_def SEC(".maps") packet_map = {
    .type = BPF_MAP_TYPE_HASH,
    .key_size = sizeof(uint32_t),
    .value_size = sizeof(struct packet_log),
    .max_entries = 1024,
};

```

The rust equivalent of the above program:
```rust 
#![no_std]
#![no_main]

use aya_ebpf::{macros::classifier, programs::TcContext};
use aya_log_ebpf::info;

// We use the exact same struct definition from our shared library
use my_shared_types::PacketLog;

#[classifier]
pub fn process_packet(ctx: TcContext) -> i32 {
    match try_process_packet(ctx) {
        Ok(ret) => ret,
        Err(_) => 1,
    }
}

fn try_process_packet(ctx: TcContext) -> Result<i32, ()> {
    // Rust's safety: We safely read the packet header
    let eth_proto = u16::from_be(unsafe { *ctx.template_pointer(offset_of_proto)? });
    
    info!(&ctx, "Received packet with protocol: {:x}", eth_proto);
    Ok(0) // TC_ACT_OK
}
```

### 3. The Userspace Side (Rust)

Using a library like **Aya**, you read that map. Because the memory layout is identical thanks to 
`#[repr(C)]`, you can "cast" the raw bytes directly into your Rust struct.

```rust
use aya::maps::HashMap;
use core::convert::TryFrom;

// Assume 'bpf' is our loaded eBPF object
let mut m_log: HashMap<_, u32, PacketLog> = HashMap::try_from(bpf.map("packet_map")?)?;

// Read the value for a specific IP (key)
let key = 0x0100007F; // 127.0.0.1
if let Ok(log_entry) = m_log.get(&key, 0) {
    println!("Captured packet on port: {}", log_entry.port);
}

```

### 4. Critical eBPF FFI Constraints

* Verifier Limits: 
    The eBPF verifier in the kernel will reject your program if your struct is too large (usually > 512 bytes 
    on the stack) or if you use pointers inside the struct (pointers are not stable between kernel and userspace).

* Endianness: 
    Most network data is **Big Endian**, while x86 CPUs are **Little Endian**. 
    You often need `u32::from_be()` when moving data between the C network buffer and Rust logic.

* Zero-Copy: 
    The beauty of this FFI approach is that it is "Zero-Copy." Rust is looking at the exact same 
    physical memory pages that the Kernel just wrote to.

