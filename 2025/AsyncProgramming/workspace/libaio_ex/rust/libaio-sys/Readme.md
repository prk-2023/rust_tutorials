# Asynchronous I/O ( AIO ) in Rust

This is a "bare-metal" implementation of **Asynchronous I/O (AIO)** in Rust. 
It bypasses the standard library's synchronous file methods to talk directly to the Linux kernel's AIO
sub-system.


This requires to define a **FFI (Foreign Function Interface)** to call C functions from `libaio`. 

---

### 1. The Memory Layout (`repr(C)`)

The kernel expects data structures to be laid out exactly as they are in the C programming language.

* **`iocb` (I/O Control Block):** 
    This is the "command card." 
    You fill this out to tell the kernel: *"What do I want to do? (Read/Write), Which file? How many bytes? 
    At what offset?"*

* **`io_event`:** This is the "receipt." When the kernel finishes the task, it writes the result (success /
  failure and bytes transferred) into this structure.

### 2. Linking the System Library

```rust
#[link(name = "aio")]
unsafe extern "C" { ... }

```
This tells the Rust compiler to link against `libaio.so` on your Linux system. 

The functions inside—`io_setup`, `io_submit`, `io_getevents`, and `io_destroy` are key four pillars of the 
Linux AIO lifecycle.

---

### 3. The Execution Flow

#### **A. Setup the Context**

`io_setup` initializes a "context" in the kernel. 
Think of this as a private workspace or a queue that can hold up to 10 simultaneous I/O requests.

#### **B. Open with `O_DIRECT**`

The code opens the file using a special flag: `libc::O_DIRECT`.

* **Standard I/O:** 
    Usually, Linux copies data from your app to a "Page Cache" (RAM) and then to the disk later.

* **Direct I/O:** 
    This tells the kernel to bypass the cache and move data directly from your application's memory to the 
    disk controller. **This is required for Linux AIO to be truly asynchronous.**

#### **C. The Aligned Buffer**

```rust
let layout = std::alloc::Layout::from_size_align(4096, 4096).unwrap();

```

Because you are using `O_DIRECT`, the disk hardware requires the memory address to be "aligned" (usually to
4096 bytes, the size of a physical sector/page). 
A standard `Vec<u8>` might start at a random memory address, which would cause the I/O to fail with `EINVAL`.

#### **D. Submission & Waiting**

1. **`io_submit`:** 
    You hand the `iocb` to the kernel. 
    Unlike a normal `write()`, this returns **immediately**. 
    The CPU can go do other work while the disk head is physically moving.

2. **`io_getevents`:** 
    This is a blocking call that tells the code: *"Don't move on until at least 1 I/O operation has finished."*
    Once the disk finishes writing, the kernel populates the `events` array.

---

### 4. Why use this instead of standard `File::write`?

In a standard web server or database:

1. If you write a 1GB file normally, your thread "sleeps" and does nothing until the disk finishes.
2. With this AIO code, you could submit 100 different write requests across 100 different files and then go 
   process HTTP requests while the hardware handles the storage in the background.

### 5. Risks in this Code

* **Unsafe Blocks:** 
    Since you are passing raw memory pointers to the C-based kernel, Rust's "borrow checker" can't help you.
    If you were to free the `buf_ptr` while the kernel was still writing to it, you would crash the program
    or corrupt memory.

* **Cleanup:** 
    The code manually calls `io_destroy` and `dealloc`. 
    In idiomatic Rust, you would usually wrap these in a `Drop` implementation to ensure they are cleaned 
    up even if the code panics.

