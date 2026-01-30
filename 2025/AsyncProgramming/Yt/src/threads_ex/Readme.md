# Threads and types of threads:

Generally we have two types of threads while programming:
- Created and managed by the Operating system  ( OS threads )
- User-level threads ( green threads )

Understanding these two types of threads is essential as Rust language handles them in different ways. 

* **OS Threads (Kernel-level):** 
    Managed by the OS. 
    They have their own stack and are scheduled by the OS kernel. 
    In Rust, these are **"1:1"** threads (one Rust thread = one OS thread).

* **User-level Threads (Green Threads):** 
    Managed by a runtime library (like `Tokio` or `async-std`) rather than the OS. 
    Rust achieves this through **Async/Await**.
    Many user-level tasks can run on a single OS thread, often called **"M:N"** scheduling.

## 1. OS-Level Threads (`std::thread`)

Rust’s std library provides `std::thread` for spawning OS threads. 
These are great for CPU-intensive tasks.

### Example

```rust
use std::thread;
use std::time::Duration;

fn main() {
    // Create an OS thread
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Hi from the OS thread! Count: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Do work in the main thread
    println!("Hi from the main thread!");

    // Wait for the thread to finish
    handle.join().unwrap();
}

```

## 2. User-level Threads (Async/Tasks)

Rust does not have a built-in "user-level thread" scheduler in the standard library.
Instead, we use **Tasks** within an async runtime like **Tokio**. 
These tasks are extremely lightweight—you can spawn hundreds of thousands of them 
without crashing your RAM.

### Example (using Tokio)

To run this, you would add `tokio = { version = "1", features = ["full"] }` to `Cargo.toml`.

```rust
#[tokio::main]
async fn main() {
    // Create a user-level "task" (Green thread)
    let handle = tokio::spawn(async {
        println!("Hi from a lightweight Tokio task!");
        // This yields the thread instead of blocking it
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        println!("Task finished.");
    });

    println!("Main function is still running...");

    // Wait for the task to complete
    handle.await.unwrap();
}

```

## Key Differences at a Glance

| Feature | OS Threads (`std::thread`) | User-level Tasks (`tokio::spawn`) |
| --- | --- | --- |
| **Memory** | Large stack (~2MB default) | Very small (bytes to KBs) |
| **Switching Cost** | High (Context switch via Kernel) | Low (User-space switch) |
| **Best For** | Parallel CPU-bound math/logic | I/O-bound work (Web servers, DBs) |
| **Scheduling** | Preemptive (OS interrupts) | Cooperative (Yields at `.await`) |

> **A quick note on "Green Threads":** Rust actually used to have green threads built into the language (pre-1.0), but they were removed to ensure the "zero-cost abstraction" philosophy. Now, we use the `Future` trait and async runtimes to get that same high-concurrency benefit only when we actually need it.


Care should also be taken while sharing data between these threads safely, example use of 
Arcs and Mutexes
