# Async programming:


## Introduction:

**Asynchronous programming**  allows you to write programs that can perform tasks concurrently without blocking
or waiting for each task to finish before moving to the next one. This is particularly useful when you are
dealing with operations that take time, such as reading from a file, fetching data from website, or waiting
for user input.

In traditional **synchronous programming**, the program executes each line of code 1-by-1. This flow gets
stuck or waits when there is a time-consuming task in the pipe, say like downloading a file, the program has
to wait until that task is finished before moving on to the next one.
This makes the programs slow and inefficient, especially if there are many tasks that could be done in
parallel. 

Key-Concepts:

- synchronous programming Model : Blocking, programs execute one task at a time and each task has to
  complete before the next one starts. Ex: download_file(), process_file(), save_to_db(),...

- Asynchronous programming Model : Non-Blocking, program does not have to wait for one-task to finish before
  starting another. Instead of blocking the entire program, it can run other tasks while waiting for slow
  operations like network requests or file I/O to complete. Ex: download_file_async(),
  process_file_async()..

- Asynchronous programming is useful:
    * Handle many tasks at once ( serving multiple web request in a web server )
    * Improve efficiency by not wasting time waiting for things like network requests, user inputs, slow
      db..
    * This make the code more responsive, especially in programs with UI or real-time applications like chat
      apps or games.


### async with Python:

Async programming is done using module `asyncio` or async libraries like `aiohttp` for web requests. 


**Basic structure of Async Code** : To write async code in python you use:

    - `async` keyword to define an *asynchronous function*
    - `await` keyword to pause the execution of a function until the result of another asynchronous
      operation is ready.

    Ex:

```python
import asyncio 

# Define an asynchronous function:
async def say_hello():
    print("hello")
    await asyncio.sleep(1) # simulate a time-consuming operation.
    print("goodbye")

# run the async func 
asyncio.run(say_hello)

```
- The say_hello() function is asynchronous, meaning it doesn’t block the program.
- `await asyncio.sleep(1)`  is time-delay function that simulates waiting for 1 sec without blocking the
  rest of the program. While it waits, other tasks could run in the background if defined.

- `async`: marks a function as asynchronous. When you call this function, it doesn't immediately execute the
  code inside. Instead it returns a co-routine, which is like  a promise to do the work later.

- `await`: causes the program to pause the execution of the current function and wait for another
  asynchronous operation to finish before continuing.

Ex:
```python 
async def fetch_data():
    print("Fetching data...")
    await asyncio.sleep(2)  # Simulating a time-consuming operation
    print("Data fetched!")

async def process_data():
    print("Processing data...")
    await asyncio.sleep(1)  # Simulate processing
    print("Data processed!")

async def main():
    # Run tasks concurrently
    task1 = asyncio.create_task(fetch_data())
    task2 = asyncio.create_task(process_data())
    
    # Wait for both tasks to finish
    await task1
    await task2

asyncio.run(main())
```
- `asyncio.create_task()` allows both functions to run concurrently. The program does not wait for one task
  to finish before starting the other.

Common Usecases of Async-Programming:
---
1. **I/O bound tasks** : operations that involve waiting for data, like reading from a file or making HTTP
   requests.

2. Concurrency: Running many independent tasks simultaneously ( running a server that handles multiple
   requests at once)

3. Real-time applications: applications where you need to respond very quickly, ( chat, games...)

Benifits and complexities :
---
- Efficiency: As Asynchronous program allows the code to do multiple things at once without waiting for one
  task to finish before starting the next they are efficient.

- Non-blocking: Allows you to keep the program responsive, even while waiting for slow tasks to complete.

- Not for CPU-bound tasks: Async programming shines with I/O-bound tasks. If your task is CPU-heavy,
  traditional multithreading or multiprocessing may be better.

- Complexity: Asynchronous code can be harder to understand and debug especially for starters.

## Asynchronous programming with C++:

C++ does not have built-in async/await keywords like in python, instead it relies on other mechanism such as 
`std::async`, `std::future` and `std::thread` to handle concurrency and parallelism.

- `std::async` : Introduced in C++11, allows you to run functions asynchronously. It automatically creates a
  thread and returns a `std::future` object that can be used to retrieve the result once the task completes. 

- `std::future` : A mechanism to retrive the result of asynchronous operation. you can use `.get()` to wait
  for the result when needed.

- `std::thread` : provides low-level threading support, allowing you to create new threads to run functions
  concurrently. However managing threads manually is more complex and error-prone then using `std::async`

In C++ the asynchronous model is typically more focused on managing threads and tasks directly rather than
using a high-level async/await model like in python. ( more caution about thread safety, synchronization and
potential race conditions)

Ex: Simple example that demonstrates how to execute tasks asynchronously in C++ using `std::async`
```c 
// example that demonstrates how to execute tasks asynchronously in C++ using std::async
#include <iostream>
#include <future>
#include <chrono>

int async_task() {
    std::this_thread::sleep_for(std::chrono::seconds(2)); // Simulate a time-consuming task
    return 42; // Return some result
}

int main() {
    // Launch async_task() asynchronously
    std::future<int> result = std::async(std::launch::async, async_task);

    std::cout << "Doing some other work while waiting..." << std::endl;

    // Get the result from async_task (this will block until the task finishes)
    int value = result.get();

    std::cout << "The result from async_task is: " << value << std::endl;

    return 0;
}
```
- `std::async` runs the `async_task()` function asynchronously.
- `std::future<int>` holds the result of the asynchronous function, which can be retrieved by calling 
  `.get()`. This call will block until the result is ready.
- Meanwhile, the program can perform other tasks without being blocked.


## Rust: asynchronous programming:

Rust has a unique and very powerful approach that strikes balance between performance and safety,
distinguishing itself from languages like C++ and Python. 

Rust's async programming model is built around `futures`, `async/await`, and a highly optimized execution
model based on polling. 

Rust achieves non-blocking behavior without using a traditional thread-based model like C++ or 
Python’s event loop.

**Key features*: 
- `async/await` syntax: Rust introduced the `async` and `await` keywords in version 1.39 (2019), allowing 
  developers to write asynchronous code in a more intuitive and readable way, similar to Python.

- `Futures`: In Rust, an `async` function returns a `future`, which is a value that represents a computation
  that may not have completed yet. This allows for asynchronous execution without blocking the thread, 
  similar to how Python’s `asyncio` and C++'s `std::future` work.

- *Zero-cost abstraction*: Unlike Python (which has a GIL) or C++ (where managing threads can be expensive),
  Rust’s *async* system is extremely lightweight. It avoids using a thread pool for every *async task*. 
  Instead, it uses *futures* that are poll-driven, which means tasks don't run concurrently unless 
  explicitly awaited, and they can be suspended and resumed by the runtime.

- No Garbage Collection: Rust avoids runtime garbage collection. Instead, it uses ownership and borrowing
  for memory safety, which means that even in asynchronous code, Rust can ensure memory safety without 
  needing a garbage collector.

- Concurrency model: Rust's async model can run tasks concurrently **within a single thread** using **async 
  runtimes** like **Tokio** or **async-std**. This is highly efficient for I/O-bound tasks, as multiple 
  tasks can execute concurrently without creating new threads for each one.

- Performance: Rust has a major performance advantage over Python in async programming. Python uses a single
  thread (due to the Global Interpreter Lock or GIL) for asynchronous tasks, which makes it more limited in 
  CPU-bound scenarios. Rust, on the other hand, is zero-cost and highly efficient, allowing asynchronous 
  tasks to run with minimal overhead, potentially making it faster than Python in most scenarios.

- Simplicity: C++ is a powerful but more low-level language when it comes to concurrency and async prog. 
  You often need to manage threads manually or use libraries like `std::async`, `std::thread`, or 
  third-party libraries like `Boost.Asio` for asynchronous operations.

- Safety: Rust takes the edge with guaranteed memory safety. While C++ allows for more control over the
  system, it comes with the risk of race conditions, undefined behavior, and memory management bugs 
  (e.g., use-after-free, dangling pointers). Rust’s strict compiler checks prevent these issues, even in 
  async code.

- Concurrency Model: In C++, you often need to manually manage concurrency with threads, mutexes, or 
  task pools. While C++ gives maximum control, it also requires more careful management. 
  Rust’s async model, is more high-level and ergonomic. Rust's async tasks are much lighter than C++ 
  threads and can run concurrently within a single thread, thanks to its efficient polling model.

### Rust’s Async Execution Model (Key Differences)

Rust’s async model works by **polling futures**. Here’s how it works at a high level:

- When you call an `async` func, it returns a `future` object that represents a value that isn't available 
  yet.

- The future is then **polled** by the async runtime (like **Tokio** or **async-std**) to check if it’s 
  ready to execute.

- If the future is not ready, the runtime yields control back to allow other tasks to run.

- This polling approach is highly efficient because it doesn’t require creating new threads for each task.
  Instead, multiple tasks can be interleaved on a single thread without blocking.

=> Rust doesn’t use an *event loop like Python* or a *thread-based approach like C++*. Instead, it builds 
on its own task-based model, leveraging **futures** and **executors** to handle concurrency efficiently.

### Rust Async Ecosystem:

- **Tokio** : A popular asynchronous runtime for Rust, optimized for I/O-bound tasks, particularly network
  applications. It's used in high-performance web servers, database clients, and other real-time services.

- **async-std** : A simpler alternative to *Tokio*, offering async APIs similar to Rust’s standard library 
  but with asynchronous versions of common tasks like I/O, file handling, etc.

- **async-io and smol**: Lighter runtimes for smaller apps that don’t require the full power of Tokio.

----------------------------------------------------------------------------------------------------

# Pre-requisites:

## Pre-requisites before learning the Fundamentals of Async Programming in Rust:

Async programming in Rust builds on several foundational concepts. 
If you’re comfortable with the below, the async chapter will feel natural and logical instead of confusing.

#### 1. Solid Understanding of Ownership & Borrowing (MOST IMPORTANT)

Async Rust is deeply tied to Rust’s ownership model because:

* async tasks may suspend and resume later
* values must remain valid across await points
* references must not outlive what they borrow

You must understand: Ownership, Borrowing rules, Lifetimes (at least the basics), Move semantics
Why `move` is often required in async closures and tasks

#### 2. Experience With Closures

Async code uses closures frequently:

* spawning tasks (`tokio::spawn(async move { ... })`)
* passing around `async` blocks
* using higher-order combinators (streams, futures)

You should know:
✓ How closures capture variables
✓ `move` closures
✓ Capturing by reference vs value

#### 3. Concurrency Basics (Threads & Message Passing)

Before async concurrency, Rust introduces *threaded* concurrency.
Understanding these helps you grasp *why async exists* and *what problems it solves*.

From the last chapter, you should understand:
`thread::spawn`,  Channels (mpsc), Shared-state concurrency (`Mutex`, `Arc`), Synchronization patterns
Why threads are expensive → motivation for async

#### 4. Understanding of Traits

Async Rust heavily uses traits like:

- `Future`
- `Send` and `Sync`
- `Unpin`
- `Sized`
- async traits from crates

You don’t need to be an expert, but you should know:
✓ What traits are
✓ What trait bounds do (`T: Send`)
✓ What it means for a type to “implement a trait”

#### 5. Basic Error Handling

Async functions return lots of `Result<T, E>` types.

You should understand:
✓ `Result`
✓ `?` operator
✓ Propagating errors

#### 6. Enums and Pattern Matching

Async runtimes and futures are implemented using enums, so understanding:
✓ `enum`
✓ Pattern match (`match`)
✓ Destructuring
will make it much easier to understand the `Future` trait and Pinning.

#### 7. Smart Pointers (Arc, Box, Pin)

Async Rust frequently uses:

* `Arc<T>` for shared state between tasks
* `Box<dyn Future>` for heap-allocated futures
* **Pinning** (`Pin<T>`) is foundational for async machinery

Before async, you just need to know:
✓ What `Box<T>` is
✓ What `Arc<T>` is
✓ That `Pin` prevents moving values in memory (you will learn full details in this chapter)

#### 8. Understanding Zero-Cost Abstractions

Rust async is low-level compared to languages like Python/JS.
You should understand:
✓ Rust does not have a built-in async runtime
✓ Async is implemented using zero-cost abstractions
✓ Async/await compiles into a state machine


## Introduction:

Rust provides robust support for asynchronous programming, unlike other languages Rust approach is quite
different, Rust prioritizes *zero-cost abstractions*.

### Why Async Programming:

Asynchronous programming is essential when you need to write concurrent programs that perform multiple tasks 
without blocking the execution of others. 

This is especially useful in I/O-bound tasks, such as handling HTTP requests, reading files, or making 
network calls, where you don’t want your program to sit idle waiting for an external process to complete.

In traditional multi-threading, each thread is dedicated to a task. 
While threads allow for concurrency, they are expensive in terms of memory and context switching, and you 
end up with a lot of overhead when you need to create thousands or millions of threads.

In contrast, asynchronous programming allows you to write concurrent code without the overhead of multiple 
threads. Instead of waiting for a task to complete, you can "pause" the task, allowing the program to 
execute other work, and resume the task when it's ready.

Rust’s async model is built with performance in mind, using its unique ownership and borrowing system to 
ensure that data races are avoided, even in concurrent scenarios.

This chapter on **async programming** introduces you to how **async/await** works in Rust, focusing on:

- **Futures**: The core building blocks of async operations in Rust.

- **The `async` and `await` keywords**: How they work together to simplify asynchronous programming.

- **Async runtimes**: Understanding the need for an external runtime (like `tokio` or `async-std`) to
  execute async code.

- **The `Future` trait**: What it is, how it works, and how you can define your own custom async types.

- **Error handling in async functions**: Dealing with errors when writing async code.

---

### What Makes Rust’s Async Model Unique?

Rust’s async system stands apart from many other languages due to a few key reasons:

1. **Zero-Cost Abstractions**:
   Rust’s async/await system is implemented with zero-cost abstractions, which means it won’t incur runtime
   overhead unless explicitly needed. The abstractions Rust provides are designed to be as efficient as the
   manual state machines you would write yourself, but without the complexity and boilerplate.

2. **Concurrency without Data Races**:
   Thanks to Rust’s ownership and borrowing rules, data races are impossible even in concurrent async code.
   The compiler will ensure that you don’t accidentally share mutable state between tasks unless it is safe 
   to do so.

3. **Customizable Futures and Executors**:
   Rust allows fine-grained control over the runtime environment for async tasks. 
   You can choose from several available async runtimes like `tokio` and `async-std`, or even write your own.

4. **Error Handling**:
   Async functions in Rust return `Result<T, E>` types just like synchronous functions. 
   However, dealing with errors in an asynchronous context requires some additional strategies, which this 
   chapter will explain.

---

### **Async Fundamentals: The Basics of `async` and `await`**

The two most important keywords in Rust’s async system are `async` and `await`. Here’s how they work:

* **`async`**: When applied to a function, this keyword transforms the function into an asynchronous func, 
  returning a `Future`. A `Future` is an object that represents a value that may not have been computed yet.

* **`await`**: This keyword is used to pause execution of an asynchronous function until the `Future` is 
  ready, meaning it’s been completed. Once the `Future` is ready, the function will continue executing.


```rust
// cargo add tokio --features=full 
use std::future::Future; // Not strictly needed here, but kept for context

async fn fetch_data() -> String {
    // Simulate fetching data (e.g., from a file or network)
    // In a real scenario, you'd perform I/O that requires 'await'
    // For a simple string return, this function doesn't strictly need to be async, 
    // but it demonstrates the async call flow.
    "Data fetched".to_string()
}

#[tokio::main]
async fn main() {
    let data = fetch_data().await;
    println!("{}", data);
}
```

In this simple example:
* The #[tokio::main] macro essentially converts `async fn main()` into a synchronous `fn main()` that 
  correctly sets up and runs the Tokio runtime, ensuring your await calls work.

* `fetch_data()` is an **async function**, returning a `Future`.

* `main()` is also async and calls `fetch_data().await` to "pause" until `fetch_data()` resolves and returns

  the data.

---

### **Key Concepts You’ll Need to Understand for Async**

1. **Futures**:

   - Futures are types that represent values that are not yet available but will be at some point in the 
     future.
   - The **`Future`** trait is central to async programming in Rust. A `Future` defines how to wait for its
     result and when it should be executed.

2. **Async Runtimes**:

   - To **run async code**, Rust requires an async runtime (like `tokio` or `async-std`).
   - These runtimes provide the event loop necessary for executing tasks. Without them, Rust cannot execute
     `async` functions directly.

3. **The `await` Operator**:

   - The `await` operator is used to pause execution until the result of a `Future` is ready. 
     Rust ensures that the function doesn't block the entire thread, only the specific task.

4. **Pinning and Unpin**:

   - In async Rust, you’ll often deal with types that need to be **pinned**. 
     Pinned types can't be moved in memory, which is important when working with `async` operations.

---

### **What Makes Rust’s Async Different from Other Languages?**

Rust’s approach to **asynchronous programming** is both **unique** and **powerful**. 
Many other languages (e.g., JS, Py) offer async programming, but Rust’s system is designed around its 
**ownership model**, ensuring that you can write **concurrent code** safely without runtime checks or 
garbage collection. Here’s what makes it different:

* **Memory Safety Without Garbage Collection**: 
  Rust’s ownership system guarantees that memory is freed when no longer needed. 
  This makes async Rust programs very efficient.

* **Concurrency Without the Overhead**: 
  Rust’s async is designed to have minimal runtime overhead, unlike the heavy garbage collection or complex 
  scheduling seen in other languages.

* **Strict Compiler Guarantees**: 
  The Rust compiler ensures that you cannot accidentally share mutable state between threads without proper 
  synchronization, avoiding data races. This is enforced at compile time.

---

### **What You Will Build**

In this chapter, you will:

1. Write simple async functions that fetch data.
2. Learn how to handle errors in async code.
3. Understand how async runtimes like `tokio` help run async functions in real applications.
4. Explore the `Future` trait and how you can use it to write your own async types.

After finishing the chapter, you should have a solid foundation for writing async code in Rust, and you’ll 
be ready to use async in real-world scenarios like **networking**, **web servers**, and **I/O-bound operations**.

---

## **Getting Started**

Before you dive into async programming, ensure that:

* You have a working knowledge of Rust's basic features (ownership, borrowing, structs, enums, etc.).
* You have the `async-std` or `tokio` crate installed to run your async programs.

Once ur ready, we’ll begin with **async basics**: how to write and run async functions, handle `Futures`, 
and use async runtimes.

---

### **Next Steps**

* **Read through the chapter** to understand the core concepts.
* **Write your own simple async function** to start experimenting with the `async` and `await` keywords.
* **Set up your project** with an async runtime (`tokio` or `async-std`) and run your first async code.

----------------------------------------------------------------------------------------------------

# Rust's Out of the box Async features  ( With out external Crates ):


Rust’s standard library **does not include a full async runtime**, but it *does* provide the core language 
features needed for asynchronous programming. 

Without using **any external crates**, here’s what Rust gives you **out of the box**:


## 1. `async` / `await` keywords

You can write asynchronous functions and block on futures using `async` and `await`.

```rust
async fn do_something() -> u32 {
    42
}
```

But:
=> You **cannot run** this async function without some executor (the std library does *not* include one).


## 2. The `Future` trait (in `core` / `std`)

Rust defines the core trait behind all *async* work:

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

Every `async fn` returns a type implementing `Future`.

## 3. `Poll`, `Context`, and `Waker`

These are the low-level building blocks:

* `Poll::Pending` / `Poll::Ready`
* `Context` provides a reference to a `Waker`
* `Waker` tells the executor when a task is ready to be polled again

These allow you to build your own async executor manually.

## 4. `Pin`

Async tasks often contain self-referential state, so Rust requires `Pin` to ensure memory safety.

```rust
Pin<&mut T>
```

The async machinery depends heavily on this.

## 5. Basic concurrency primitives

Rust gives you some concurrency tools that can be used in simple async executors:

### Threading

```rust
std::thread::spawn(|| { ... });
```

### Channels (multi-producer, single-consumer)

```rust
std::sync::mpsc::channel();
```

### Atomics

```rust
std::sync::atomic::*;
```

### Mutex / RwLock / Condvar

Useful if you’re building your own executor:

```rust
std::sync::{Mutex, Arc};
```

## What the standard library *does NOT* provide

Rust **does not include**:

🚫 An async runtime
🚫 An executor (like `tokio::spawn` or `async-std`)
🚫 Async I/O (async TCP, async file I/O, timers, etc.)
🚫 Async synchronization primitives (async Mutex, async channels, etc.)

To actually *run* async tasks, you must either:

* **Write your own mini‐executor**
* **Use a runtime crate like `tokio`, `async-std`, or `smol`**

## Minimal example: tiny executor without external crates

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

fn dummy_waker() -> Waker {
    fn no_op(_: *const ()) {}
    fn clone(_: *const ()) -> RawWaker { raw_waker() }

    fn raw_waker() -> RawWaker {
        RawWaker::new(std::ptr::null(), &RawWakerVTable::new(clone, no_op, no_op, no_op))
    }

    unsafe { Waker::from_raw(raw_waker()) }
}

fn block_on<F: Future>(mut future: F) -> F::Output {
    let waker = dummy_waker();
    let mut context = Context::from_waker(&waker);
    let mut future = unsafe { Pin::new_unchecked(&mut future) };

    loop {
        match future.as_mut().poll(&mut context) {
            Poll::Ready(val) => return val,
            Poll::Pending => continue,
        }
    }
}

async fn hello() -> i32 {
    10
}

fn main() {
    let result = block_on(hello());
    println!("Result: {}", result);
}
```

---

## Summary: What Rust offers (no crates)

| Feature                    | Provided? | Notes                                 |
| -------------------------- | --------- | ------------------------------------- |
| `async`/`await` syntax     | ✅         | Built into language                   |
| `Future` trait             | ✅         | In `core`/`std`                       |
| `Poll`, `Waker`, `Context` | ✅         | Needed to build executors             |
| `Pin`                      | ✅         | Safe async state                      |
| Async I/O                  | ❌         | Needs crates (tokio, async-std, etc.) |
| Task executor              | ❌         | You must write your own               |
| Async Mutex / channel      | ❌         | Only sync versions provided           |

---

# How to write a **simple single-thread async executor** (Example)


Examples **written entirely with the Rust standard library**, with **no external crates**, showing:

- **A simple single-thread async executor**
- **A minimal async TCP server using non-blocking sockets + custom futures**

These examples are intentionally simple and educational; production executors use far more sophisticated mechanisms.

---

## 1. A Simple Single-Thread Async Executor (No Crates)

This executor:

* Holds a queue of tasks
* Polls each task until completion
* Uses a dummy waker that immediately requeues the task
* Runs everything on one thread

### Complete Example: Minimal executor

```rust
use std::future::Future;
use std::task::{Context, Poll, Waker, RawWaker, RawWakerVTable};
use std::pin::Pin;
use std::collections::VecDeque;

// ---- Dummy waker for single-thread executor ----

fn dummy_raw_waker() -> RawWaker {
    fn clone(_: *const ()) -> RawWaker { dummy_raw_waker() }
    fn wake(_: *const ()) {}
    fn wake_by_ref(_: *const ()) {}
    fn drop(_: *const ()) {}
    RawWaker::new(std::ptr::null(), &RawWakerVTable::new(clone, wake, wake_by_ref, drop))
}

fn dummy_waker() -> Waker {
    unsafe { Waker::from_raw(dummy_raw_waker()) }
}

// ---- Task wrapper ----

struct Task {
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Task {
    fn new<F: Future<Output = ()> + 'static>(f: F) -> Self {
        Self {
            future: Box::pin(f),
        }
    }
}

// ---- Simple Executor ----

struct Executor {
    tasks: VecDeque<Task>,
}

impl Executor {
    fn new() -> Self {
        Self { tasks: VecDeque::new() }
    }

    fn spawn<F: Future<Output = ()> + 'static>(&mut self, future: F) {
        self.tasks.push_back(Task::new(future));
    }

    fn run(&mut self) {
        let waker = dummy_waker();
        let mut cx = Context::from_waker(&waker);

        while let Some(mut task) = self.tasks.pop_front() {
            match task.future.as_mut().poll(&mut cx) {
                Poll::Pending => {
                    // Not ready yet; requeue the task
                    self.tasks.push_back(task);
                }
                Poll::Ready(()) => {
                    // Task finished; drop it
                }
            }
        }
    }
}

// ---- Example async function ----

async fn example() {
    println!("Hello from async executor!");
}

// ---- Main ----

fn main() {
    let mut ex = Executor::new();
    ex.spawn(example());
    ex.run();
}
```

This is a fully functional single-thread async executor.
It does *not* support async I/O, timers, or waking — yet.
We add that in part 2.

---

## 2. Async TCP Server Using Only `std` (non-blocking sockets + custom Future)

Since Rust’s standard library has **no async networking**, we must:

* Put the TCP listener/socket into **non-blocking mode**
* Attempt reads/writes that may return `WouldBlock`
* Return `Poll::Pending` when blocked
* Implement a `Future` for each type of operation

This will demonstrate the core principles of async I/O without any crates.

---

### Step 1 — Non-blocking TCP Listener Future

This future completes when a new TCP connection arrives.

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::io::{self, ErrorKind};
use std::net::{TcpListener, TcpStream};

struct AcceptFuture<'a> {
    listener: &'a TcpListener,
}

impl<'a> Future for AcceptFuture<'a> {
    type Output = io::Result<TcpStream>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.listener.accept() {
            Ok((stream, _addr)) => Poll::Ready(Ok(stream)),
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}
```

---

### Step 2 — Non-blocking read future

```rust
struct ReadFuture<'a> {
    stream: &'a TcpStream,
    buf: &'a mut [u8],
}

impl<'a> Future for ReadFuture<'a> {
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.stream.read(self.buf) {
            Ok(n) => Poll::Ready(Ok(n)),
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}
```

---

### Step 3 — Non-blocking write future

```rust
struct WriteFuture<'a> {
    stream: &'a TcpStream,
    buf: &'a [u8],
}

impl<'a> Future for WriteFuture<'a> {
    type Output = io::Result<usize>;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.stream.write(self.buf) {
            Ok(n) => Poll::Ready(Ok(n)),
            Err(ref e) if e.kind() == ErrorKind::WouldBlock => Poll::Pending,
            Err(e) => Poll::Ready(Err(e)),
        }
    }
}
```

---

### Step 4 — Async connection handler

```rust
async fn handle_client(mut stream: TcpStream) {
    use std::io::Write;
    stream.write_all(b"Hello from async server!\n").unwrap();

    let mut buf = [0u8; 512];

    loop {
        let n = ReadFuture { stream: &stream, buf: &mut buf }.await.unwrap();
        if n == 0 {
            break;
        }

        // Echo back
        WriteFuture { stream: &stream, buf: &buf[..n] }.await.unwrap();
    }
}
```

---

### Step 5 — Async TCP server using our executor

```rust
async fn server() {
    let listener = TcpListener::bind("127.0.0.1:9000").unwrap();
    listener.set_nonblocking(true).unwrap();

    loop {
        if let Ok(stream) = AcceptFuture { listener: &listener }.await {
            stream.set_nonblocking(true).unwrap();
            // Spawn handler task
            EXECUTOR.with(|ex| {
                ex.borrow_mut().spawn(handle_client(stream));
            });
        }
    }
}
```

Note: `EXECUTOR` could be a thread-local reference to our single-thread executor.

### Final `main` function

```rust
fn main() {
    let mut ex = Executor::new();
    ex.spawn(server());
    ex.run();
}
```

This covers:
✔ Writing an async executor from scratch
✔ Implementing async I/O by hand
✔ How `Future`, `Poll`, `Waker`, and non-blocking sockets interact
✔ No external crates required

However — this is *not production-ready*.
It lacks:

* real waker notifications
* epoll/kqueue/iocp integration
* timers
* backpressure
* thread pool
* async cancellation

Those are exactly what big runtimes like **Tokio**, **async-std**, and **smol** provide.

