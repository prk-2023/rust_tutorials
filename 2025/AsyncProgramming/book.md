# Fundamentals of Asynchronous Programming

Many operations we ask the computer to do can take a while to finish. 
Its nice if we could do something else while we are waiting for those long-running process to complete. 
Modern computer offer 2 techniques for working on more then one operation at a time:
- Parallelism 
- Concurrency 

Once we start writing programs that involve parallel or concurrent operations, though, we quickly encounter 
new challenges inherent to asynchronous programming, where operations may not finish sequentially in the 
order they were started.

Ex: if we’re building a tool to manage file downloads, we should be able to write our program so that 
starting one download won’t lock up the UI, and users should be able to start multiple downloads at the 
same time. 
Many operating system APIs for interacting with the network are blocking, though; that is, they block the 
program’s progress until the data they’re processing is completely ready.

Note: This is how most function calls work, if you think about it. However, the term blocking is usually 
reserved for function calls that interact with files, the network, or other resources on the computer, 
because those are the cases where an individual program would benefit from the operation being non-blocking.

We could avoid blocking our main thread by spawning a dedicated thread to download each file. 
However, the overhead of those threads would eventually become a problem. 

It would be preferable if the call didn’t block in the first place. It would also be better if we could 
write in the same direct style we use in blocking code, similar to this:
```rust
    let data = fetch_data_from(url).await;
    println!("{data}");
```

That is exactly what Rust’s async (short for asynchronous) abstraction gives us.
This leaves us with :
- How to use Rust’s async and await syntax
- How to use the async model to solve some of the same challenges we looked at in Chapter 16 ( Concurrency )
- How multithreading and async provide complementary solutions, that you can combine in many cases

before that we need a clear picture of:

### Parallelism and Concurrency:

Asynchronous Programming is a paradigm that allows for non-blocking execution where operations ( like I/O,
networking requests, or file reading ) can run concurrently without blocking the main thread of execution.
This is crucial in modern programming for building efficient, scalable applications especially for systems
with many I/O bound tasks.

Key-Concepts:
1. **Concurrency** :Running multiple tasks "at once", but not necessarily in parallel. In Rust, this
   typically means executing multiple tasks that don’t block the main thread of the program.

2. **Parallelism**: Performing multiple tasks literally at the same time, often in different cores or threads.

In Rust, async programming is made possible through a combination of the `async` & `await` keywords, along 
with `Futures` and `Streams`.

#### `async` `await` and future

- **future**: a value that may not be ready now but will become ready at some point in the future. 
  (This same concept shows up in many languages, sometimes under other names such as *task* or *promise*.) 
  Rust provides a `Future` trait as a building block so that different async operations can be implemented 
  with different data structures but with a common interface. 

  In Rust, futures are types that implement the Future trait. 
  Each future holds its own information about the progress that has been made and what “ready” means.

- `async`: Functions 
  In Rust, functions that perform asynchronous operations are defined using the `async` keyword. 
  These functions return a `Future` a value that represents the eventual result of an asynchronous operation.

  * You can apply the `async` keyword to blocks and functions to specify that they can be interrupted and
    resumed. 

  * with async block or function you can use `await` keyword to await a future ( i.e wait for it become
    ready). Any point where you await a future within an async block or function is a potential spot for 
    that async block or function to pause and resume. 

  * The process of checking with the future to see if the value is available is called *"polling"*

  * When writing async Rust, we use the async and await keywords most of the time. Rust compiles them into 
    equivalent code using the Future trait, much as it compiles for loops into equivalent code using the 
    Iterator trait. 
    Because Rust provides the Future trait, though, you can also implement it for your own data types when 
    you need to. 

```rust 
async fn fetch_data() -> String {
    // Simulating async work
"Data fetched!".to_string()
}
```
* The `fetch_data()` function doesn't immediately return its value. Instead, it returns a `Future<String>`.
* The actual data will be fetched later, and we will "await" its result.

- `await`: Keyword. 
  To get the result of an `async` function, we use the `await` keyword. 
  It tells Rust to pause the current task until the `Future` is ready and then return the value.

```rust 
#[tokio::main] // Attribute to set up the async runtime (more on this later)
async fn main() {
    let data = fetch_data().await; // Await the result of the async function
    println!("{}", data); // Print the result
}
```
* `await` only works within an `async` context, i.e it can only be used inside `async` functions.
* The program will not block during the `await`

- `Future` : the `async` function returns a **Future**, which represents a value that will be computed at
  some point in the future.

  * A **Future** is a Rust trait that represents a value that will be available at some point in the future.
    Its like a **promis** or a **blueprint** for the work that needs to be done. Is sits there, inert, until
    something actively tries to run it.

  * Crutially, Rust's std lib does not contain code to repeatedly check on these `Future`'s and run them. 

  * `Future` is a trait, and it defines methods like `.poll()` that are used to determine if the future has
    been completed. Rust's runtime (like Tokio and async-std) handles polling the future to make progress on
    the computation.

  * Rust provides a `Future` trait as a building block so that different async operations can be implement
    with different data structures but with a common interface. 

  * => futures are types that implement the `Future` trait. Each future holds its own information about the
    progress that has been made and what "ready" means.


Ex:
```rust 
use std::future::Future;
use std::pin::Pin;

async fn hello_world() -> String {
    "Hello, world!".to_string()
}

fn main() {
    let future = hello_world(); // This returns a Future<String>
    
    // To resolve the future, we would typically need an async runtime like Tokio.
    // You can't just poll it directly in the main function.
}
```


- `async` Block and `.await`:
  You can also create an `async` block that returns a Future without needing a named `async fn`

```rust 
let future = async {
    // Some asynchronous work here 
    "Hello from async block"
};
let result = future.await() // await the result 
println!("{}", result);
```
* The async block is useful of inline async code, especially in places where you dont want to define a
  separate async function. 

### Asynchronous Runtime in Rust:

1. **Rust’s Standard Library and Futures:**

   * Rust’s std lib doesn’t contain an executor or runtime for managing asynchronous tasks. It provides the
     foundational `Future` and `async/await` syntax, but **the actual task scheduling and execution** 
     require an external asynchronous runtime like **Tokio** or **async-std**.

2. **Executor's Role:**

   * The **Executor** is the component responsible for polling and driving `Future` values to completion. 
     Futures are lazily evaluated and do nothing unless they are **polled** by an executor. 
     When you call `.await`, you’re signaling the executor that the task has to yield until its awaited 
     value is ready. Once the value is ready, the executor can resume the task.

3. **Thread Management and Task Switching:**

   * When a `Future` is paused (via `.await`), the executor **doesn't immediately switch the current thread**
     to another `Future` per se. It places the task into a **pending queue**, and the **executor** then 
     chooses which task to poll next from this queue. The thread may continue running other tasks, but it 
     doesn’t switch threads arbitrarily. 
     If there's idle time or no immediate tasks to execute, the thread might wait until there’s work to do.

   * The key idea is that **the executor maximizes thread efficiency** by running many tasks on a small 
     number of threads. This allows many async tasks (potentially thousands) to be scheduled without needing
     an equal number of threads.

4. **Rust Doesn’t Include an Async Runtime:**

   * The standard library doesn’t provide an async runtime. This is why libraries like **Tokio** or 
     **async-std** ( now Deprecated in favor of `smol`) are needed. They provide the functionality to 
     execute async code efficiently by managing the scheduling of `Futures` and performing necessary I/O 
     operations.

5. **Executor’s Job:**

   * The executor’s job is to **efficiently manage** and **schedule many thousands of paused tasks** on a 
     small number of threads. These tasks can be in various states of progress, with some waiting on I/O, 
     and others actively running.

6. **Tokio/async-std/smol - Async Runtimes:**

   * **Tokio** and **amol** are two popular asynchronous runtimes that manage the scheduling, polling, and 
     execution of async tasks. They are both highly optimized for handling a large number of I/O-bound tasks
     concurrently. 
     They take care of all the complexities related to managing `Future`s, scheduling them on threads, and 
     waiting for external events like I/O readiness.

7. **Components of an Async Runtime (Tokio/async-std):**

   * The async runtime typically includes the following components:

     * **Scheduler/Executor**: This decides **which `Future` to run next**. It continuously polls `Future`s 
       to determine if they are ready to make progress. 
       If a `Future` has completed, the executor can move to the next task in the queue.

     * **Reactor/Proactor**: This handles **external events** like network, file I/O, or timers. 
       When a task is awaiting I/O (e.g., waiting for data from a network), the reactor listens for that 
       event and signals the executor when the task can resume. 
       This part is responsible for being notified when external resources (like a socket or file) are ready
       to continue the task.

     * **Thread Pool**: The thread pool (in **Tokio**, it's part of the runtime) contains worker threads 
       that the executor uses to run tasks. The runtime can scale this pool based on system load or 
       configuration. 
       Worker threads don’t execute tasks directly—**the executor assigns tasks to threads as needed**. 
       In the case of async runtimes like **Tokio**, tasks that are CPU-bound (e.g., computation-heavy 
       operations) can be offloaded to worker threads in a pool, while I/O-bound tasks can run on the 
       executor’s threads.

Note: Thread Pool: isn't always an inherent part of the executor. Some async runtimes (like Tokio) use a 
      thread pool, but others (like `async-std`) may work differently. 

For example, `async-std` has a more lightweight model using a smaller set of threads and focusing more on 
the task scheduler.


Example Using Tokio:

```bash 
cargo add tokio --features=full

```
```rust 
use tokio::time::Duration;

async fn fetch_data() -> String {
    // Simulating async work
    "Data fetched!".to_string()
}

#[tokio::main]
async fn main() {
    let result = fetch_data().await;
    println!("Waiting for fetch_data() ");
    println!("{}", result);
}
```
- `#[tokio::main]` sets up the Tokio runtume and allows you to use `async` function in the `main` function.

- With Tokio we dont need to manually spawn threads or manage event-loop it does all for you.

