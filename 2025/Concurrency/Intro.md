# Concurrency:

## Prerequisites for Concurrency in Rust

To grasp Rust's approach to concurrency, you must first understand how the language ensures thread safety
using core traits and specific shared-state mechanisms.

### 1. Concurrency Safety Traits: `Send` and `Sync`

These are crucial **marker traits** (traits without methods) that the Rust compiler uses to enforce thread
safety rules. Understanding them is key to writing safe concurrent code.

* **`Send` Trait:** Indicates that it is **safe to transfer ownership** of a type from one thread to
  another.
    - If a type implements `Send`, a variable of that type can be moved across thread boundaries.
    - Almost all primitive types (`i32`, `bool`) and simple containers (`Vec<T>`, `String`) are `Send`.
    - Types that contain raw pointers or manage unmanaged memory are often **not** `Send` by default, as
      moving them could break assumptions about where that memory is valid.

* **`Sync` Trait:** Indicates that it is **safe to share a reference** ($\text{\&T}$) to a type across
  multiple threads.
    - If a type implements `Sync`, multiple threads can safely hold immutable references to that data
      simultaneously.
    - The rule is simple: **A type $\text{T}$ is $\text{Sync}$ if and only if $\text{\&T}$ is $\text{Send}$**.

This means if a shared reference can be safely sent to another thread, the type itself is safe to be shared.

### 2. Thread-Safe Shared State Mechanisms

When multiple threads need to access and potentially modify the **same piece of data**, standard Rust
references are insufficient. You need mechanisms designed specifically for synchronization.

#### A. Atomic Operations

For simple, fundamental types (like integers and booleans), you use **atomic types** ($\text{AtomicUsize}$,
$\text{AtomicBool}$).

* **Function:** These types provide methods that guarantee operations like reading, writing, or incrementing
  happen in a single, **uninterruptible** step by the CPU. This prevents data races on simple values without
  needing heavy locking mechanisms.
* **Performance:** They are the fastest way to manage simple shared counters or flags across threads.

#### B. Mutual Exclusion (`Mutex<T>`)

The $\text{Mutex}$ (Mutual Exclusion) type is the primary tool for sharing *mutable* state safely between
threads.

* **Safety:** A $\text{Mutex}$ guarantees that **only one thread** can access the inner data ($\text{T}$) at
  any given time.
* **Locking:** To access the data, a thread must first call $\text{lock()}$. If another thread holds the
  lock, the current thread will **block** (pause) until the lock is released.
* **Guard:** The $\text{lock()}$ method returns a special smart pointer called a **lock guard**. This guard
  automatically implements $\text{Deref}$ and $\text{DerefMut}$ to provide access to the inner data. When
  the lock guard goes out of scope, the lock is automatically released (RAII).

---

### 3. Asynchronous Basics (Precursor to `async/await`)

Concurrency can also be achieved without traditional multi-threading using asynchronous programming.

* **Futures:** In Rust, an asynchronous operation is represented by a **`Future`**—a trait that represents a
  value that *might* be ready in the future. It's essentially an IOU for a result.
* **The $\text{await}$ Keyword:** When a task calls $\text{await}$ on a Future, it **pauses execution** of
  the current task until the result is ready. The critical distinction from a traditional lock is that this
  pause **does not block the thread**. Instead, the thread is free to switch to and work on other pending
  tasks.
* **Executor:** Futures must be run by an **Executor** (or Runtime), which is responsible for polling the
  Futures, managing their state, and waking them up when their needed resources (like IO) are available.

Understanding these concepts—safe data movement (`Send`), safe data sharing (`Sync`), controlling mutable
access ($\text{Mutex}$), and non-blocking waiting ($\text{Future}/\text{await}$)—will give you the solid
foundation needed for the next chapter. Concurrency and asynchronous programming in Rust rely heavily on
several fundamental concepts that you've touched on with smart pointers. Understanding these deeper concepts
will make the subsequent chapters on multi-threading and asynchronous code much clearer.

Here are the key topics relevant to concurrency, along focusing on the concepts that provide safety
guarantees.

---

## I. Traits for Concurrency Safety

Rust's core concurrency safety is enforced through two primary marker traits. These traits have no methods
but are used by the compiler to guarantee that data can be safely sent between threads or accessed across
thread boundaries.

### 1. The `Send` Trait

The `Send` trait indicates that it's **safe to transfer ownership** of a type's value from one thread to
another.

* **When is a type `Send`?** Most primitive types (like integers, booleans) and standard library types (like
  `String`, `Vec<T>`) are `Send`.
* **The Rule:** If a type `T` is composed entirely of types that are `Send`, then `T` is also automatically
  considered `Send`.
* **Exception:** Types that manage external resources (like raw pointers or certain non-thread-safe buffers)
  may not be `Send` because transferring them could lead to memory corruption or crashes on the receiving
  thread.

### 2. The `Sync` Trait

The `Sync` trait indicates that it's **safe for a type to be referenced** across thread boundaries (i.e.,
it's safe to use a shared reference, $\text{\&T}$, in multiple threads).

* **When is a type `Sync`?** If a type $\text{T}$ is $\text{Send}$, it is often also $\text{Sync}$. If
  $\text{\&T}$ is $\text{Send}$, then $\text{T}$ is $\text{Sync}$.
* **The Rule:** If a type $\text{T}$ is composed entirely of types that are $\text{Sync}$, then $\text{T}$
  is also automatically considered $\text{Sync}$.
* **The Importance:** This trait guarantees that multiple threads can safely read the data simultaneously
  without causing data races.

> **Analogy:** If a box of tools is $\text{Send}$, you can move the box to another workshop. If the box is
> $\text{Sync}$, you can let another worker share the tools while you're still using them (e.g., they can
> look at the wrench while you use the screwdriver).

---

## II. Atomic Types and Shared Mutability

When threads need to share and mutate data, regular references and even $\text{RefCell}<T>$ are insufficient
(as $\text{RefCell}<T>$ is not thread-safe). Concurrency relies on explicit tools to manage shared state.

### 1. $\text{Mutex}<T>$ (Mutual Exclusion)

The $\text{Mutex}<T>$ is the thread-safe equivalent of $\text{RefCell}<T>$ (for interior mutability)
combined with $\text{Rc}<T>$ (for shared ownership).

* **Purpose:** It ensures that only one thread can access the inner data ($\text{T}$) at any given time.
* **Locking:** To access the data inside, a thread must first **acquire a lock**. If another thread holds
  the lock, the current thread blocks (pauses) until the lock is released.
* **Access:** When the lock is acquired, the $\text{Mutex}$ returns a **smart pointer** (called a guard)
  that implements $\text{Deref}$ and $\text{DerefMut}$, allowing you to read or write to the inner data
  $\text{T}$. When the guard goes out of scope, the lock is automatically released (RAII).

### 2. $\text{Arc}<T>$ (Atomic Reference Counting)

$\text{Arc}<T>$ is the thread-safe version of $\text{Rc}<T>$.

* **Purpose:** Allows multiple threads to share ownership of data on the heap.
* **Atomic Operations:** The crucial difference is that $\text{Arc}<T>$ uses **atomic operations** to
  increment and decrement the reference count. Atomic operations are special CPU instructions that guarantee
  the count update is completed in one step, without any possibility of interruption by another thread. This
  prevents data races on the counter itself.
* **Usage:** You almost always combine $\text{Arc}<T>$ with $\text{Mutex}<T>$ to achieve **shared, mutable
  ownership** across threads: $\text{Arc}<\text{Mutex}<T>>$.

### 3. Atomic Types ($\text{AtomicU64}$, etc.)

For simple, single numeric values (like counters), using a full $\text{Mutex}<T>$ is overkill. **Atomic
types** provide direct, lock-free, thread-safe ways to read and write simple primitives.

* They provide methods like $\text{fetch\_add()}$ or $\text{compare\_and\_swap()}$ that execute in a
  guaranteed thread-safe manner, offering the highest performance for simple concurrent operations.

---

## III. Asynchronous Concepts (Pre-cursor to `async/await`)

While $\text{Mutex}<T>$ and $\text{Arc}<T>$ manage threads blocking/waiting, modern concurrency often uses
**asynchronous programming** to avoid blocking and improve efficiency.

### 1. Futures

A **Future** is a trait representing a value that might not be ready yet.

* **Analogy:** A Future is like an **IOU** (I Owe You). When you start a slow operation (like a network
  request), you immediately get a Future back. The actual result will be available later.
* **The $\text{poll}$ Method:** At its core, a Future has a $\text{poll}$ method that is called repeatedly
  by an executor.
* If the result is ready, it returns the value.
* If the result is not ready, it returns $\text{Pending}$ and arranges for the executor to be notified when
  it should try again (waking).

### 2. $\text{async}$ and $\text{await}$

These keywords are syntactic sugar that makes working with Futures feel like writing synchronous code.

* **$\text{async}$:** Wraps a function's body in a Future.
* **$\text{await}$:** Pauses the execution of the current function until the Future it's called on completes
  (the result is ready). **Crucially, it does not block the thread.** Instead, it yields control back to the
  executor, allowing the thread to work on other pending Futures.

Understanding these traits and tools is the key to safely managing state and executing code efficiently in
concurrent and asynchronous environments.

# Basics :

### 1. Thread: 

A unit of execution, its like separate path of execution in a program, allowing your program to do
multiple things at once ( in parallel or concurrently ). Threads are created by Operating System and managed
by CPU.

In Rust threads are created using `thread::spawn` function, which executes a closure ( a small function )
ina new thread. 

### 2. Spawn: 

- `thread::spawn` is used to create a new thread. When we call `Spawn` it starts the execution of a closure
  in a separate thread.

```rust 
use std::thread;

fn main() {
    let handle = thread::spwan(|| {
        //this code runs in new thread 
        println!("This is a separate thread.");
    });
    //Wait for the spawn thread to finish 
    handle.join().unwrap();
}
```
- Thread start running immediately after being spawned.
- `spawn` returns a `JoinHandle`, which is a handle to the spawned thread, used to wait for the thread's
  completion ( using `join` ).
- The closure inside `spawn` can contain any code, including other function calls. When it finishes
  execution, the thread terminates. 

### 3. Join:
- `join()` is used to wait for the thread to finish. When you call `join()` on a thread's handle, it blocks
  the current thread until the spawned thread finishes executing. 

```rust 
    let handle = thread::spawn ( || {
        //some computing
        println!("thread's busy at work!");
    });
    // this below line blocks until the thread completes its work.
    handle.join().unwrap();
```
- It ensures that the main thread ( or the thread that spawned the new one) doesn't exit before the spawned
  thread finishes. 

- `join` can return a `Result` and you generally call `.unwrap()` or handle the error in case the thread
  panics. 

### 4. Concurrency Vs Parallelism:

- Concurrency: 

Refers to the ability to handle multiple tasks at once, but not necessarily simultaneously. 
In Rust, with `std::thread`, threads are scheduled by the operating system to run on a single CPU core, one
after another, so they give the illusion of concurrency.

- Parallelism: 

Refers to tasks actually running simultaneously, often on different cores of the CPU. 
This happens when you spawn multiple threads that are executed on different CPU cores.

### 5. JoinHandle:

- A `JoinHandle` is the value returned by `thread::spawn`. 
  It represents the handle to a thread that was spawned and allows you to manage the thread’s lifecycle 
  ( such as waiting for it to complete using join() ).

```rust 
let handle = thread::spawn(|| {
    println!("I am a new thread!");
});
handle.join().unwrap();  // Waits for the spawned thread to finish
```

### 6. Panics:

A panic in Rust is an unexpected error (like accessing an out-of-bounds index in an array). 
When a thread panics, it doesn’t stop the main thread or other threads from running.
However, it can affect the join operation.

- If a thread panics, calling `.join()` will return a `Result::Err`, and you’ll have to handle it properly 
  ( usually with `.unwrap()` or `.expect()`).

```rust 
let handle = thread::spawn(|| {
    panic!("Oops! Something went wrong in this thread.");
});
let result = handle.join();
match result {
    Ok(_) => println!("Thread completed successfully."),
    Err(e) => println!("Thread failed with error: {:?}", e),
}
```

### 7. Thread-local storage:

In multi-threaded programs, threads can have their own local storage (e.g., data that’s only visible within 
that thread). Rust offers a feature called `thread::LocalKey` to store data that’s specific to a particular 
thread.

```rust
use std::thread;
use std::cell::RefCell;

thread_local! {
    static COUNTER: RefCell<i32> = RefCell::new(0);
}

fn main() {
    COUNTER.with(|counter| {
        *counter.borrow_mut() += 1;
        println!("Counter is: {}", *counter.borrow());
    });

    // Each thread can have its own COUNTER value
    let handle = thread::spawn(|| {
        COUNTER.with(|counter| {
            *counter.borrow_mut() += 1;
            println!("Thread-local counter is: {}", *counter.borrow());
        });
    });

    handle.join().unwrap();
}
```
- `thread_local!` is the macro used to define variables that are local to each thread.

### 8. Mutex: (mutual exclusion)

- A **mutex** is synchronization primitive used to ensure that only one thread can access some data at a
  time. This is important when you're working with shared mutable state across threads.

In Rust you can use `std::sync::Mutex` to lock data between threads. It prevents other thread from accessing
the data while one thread is working with it.

```rust 
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn( move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
    println!("Counter: {:?}", *counter.lock().unwrap());
}
```
- `Arc`: Atomic Reference Counting. It's used for **shared ownership** of data across threads. 

- `Mutex`: Ensures exclusive access to the data inside the `Mutex`. Only one thread can access that data at
  a time.

### 9. Atomic Operations:

For low-level concurrency control, Rust provides atomic types (ex:`AtomicBool` and `AtomicUsize`) from the
`std::sync::atomic` module. These types allow you to perform lock-free operations on shared data.

Atomic operations are generally more performant but more error-prone and complex then mutexes.


### Summary of Key Thread Functions and Terms:

* **`thread::spawn`**: Creates a new thread to run a closure concurrently.
* **`join()`**: Waits for a thread to finish execution.
* **`JoinHandle`**: The handle returned by `spawn`, used to join the thread.
* **Concurrency**: Handling multiple tasks at once (not necessarily simultaneously).
* **Parallelism**: Tasks running simultaneously on multiple cores.
* **Mutex**: A synchronization primitive for managing access to shared data.
* **Arc**: A reference-counted pointer for shared ownership across threads.
* **Panics**: An error in a thread that causes it to terminate early.
* **Atomic operations**: Low-level, lock-free operations on shared data.

---

Rust’s concurrency model emphasizes **safety** (thanks to ownership rules) and **efficiency** 
(due to zero-cost abstractions). 
Threads, combined with Rust’s unique memory model, help ensure that your multithreaded programs are both 
safe and performant.
