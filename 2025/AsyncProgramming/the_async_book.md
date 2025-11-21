# Asynchronous Programming In Rust:


Reference: 
1. [Asynchronous Programming In Rust](https://rust-lang.github.io/async-book/)

2. [Tokio Fast, scalable, Reliable async runtime](https://tokio.rs/tokio/tutorial)

3. [async-std](https://book.async.rs/introduction) ( Deprecated in favor of `smol` )

4. [smol A small and fast async runtime](https://github.com/smol-rs/smol) 

## Intro:

Async programming is a way to achieve concurrency in a program without relying on traditional
multi-threading or creating a new OS thread for each concurrency task.

### - Concurrency: 
  It Refers to the ability of a program to manage multiple tasks at once, but necessarily
  executing them simultaneously. It's got to do more about structuring your program in such a way that it
  can handle many things at once, without the running them in parallel ( as done in threads based approach).

  Concurrency in async is achieved by allowing to switch between tasks during "idle" time, like waiting for
  I/O to complete, or waiting for response. ( file read, network requests ..). This is done by
  * using Non-Blocking calls and 
  * event loop.

Async programming helps you organize your task so that while one task is waiting, the program can do
something else ( processing another request or calculating other useful items as per the program.)

### - Parallelism: 
  Parallelism is about executing multiple tasks simultaneously which often requires multiple threads or even
  multiple CPU cores. 

### - Concurrency Without Threads

Async programs do not directly achieve parallelism because, it doesn't create new threads for each task.
Instead it **multitasks** within a single thread ( or few threads ). 

The switching is done at specific points where tasks are waiting for something, like waiting for I/O 
operations to finish (reading from a file, sending/receiving data over the network, etc.).

- **Event Loop**: 
        In Async Programming there is a *event loop* ( like `tokio` runtime , `smol` light weight
  executor ). The event loop checks whether any tasks are ready to be executed and run them. It Schedules
  tasks that are waiting (such as I/O bound tasks) and switch between them without blocking the main
  program. When One task is waiting ( ex: network data) the event loop moves on to run the next ready task.

- **Futures and Tasks**: 
    They are basic building blocks in async programming **future** and **task**.
    * **$Futures$** : represent a computation that will eventually produce a result, but it's not ready yet.
    * **$Tasks$** are units of work that are executed asynchronously. You "run" a task, and the async runtime
      manages when it runs and when it yields control, allowing other tasks to run.
    
- Non-Blocking I/O: ( Core advantages of Async )
    Instead of blocking a thread while waiting for data from a disk or network, async functions will yield 
    control back to the event loop, allowing other tasks to run. 
    This enables high concurrency without needing multiple threads.

### - Why avoid threads:

1. **Overhead of threads**: 
    For each thread, OS consumes memory and resource, and switching between threads involved some overhead
    as OS must save the state of one thread and load the state of another. This is expensive in terms of
    performance.

2. **Scaling with Async**: 
    Async programming allows for many concurrent tasks without overhead of managing multiple OS threads.
    Instead of spawning a new thread for each task, you have a small pool of threads that effectively manage
    multiple async tasks. This leads to better scalability and lower resource consumptions ( especially when
    you are handling many network requests )

#### - Example Async I/O vs Thread-based I/O:

1. Thread-based I/O (Blocking): 
    Imagine handling 100 HTTP requests on a server. If each request is handled by a separate thread, 
    you'll have 100 threads running concurrently. Each thread will block while waiting for I/O (e.g., 
    reading from a file, accessing a database), and only one thread can run on each CPU core at a time. 
    If you have more threads than CPU cores, your threads will spend time waiting for the OS to schedule them.

2. Async I/O (Non-blocking):
    With async programming, the same 100 HTTP requests can be handled by just one or a few threads. 
    When a request is waiting for I/O (like database access), it yields control back to the event loop, 
    allowing other requests to be processed in the meantime. 
    The runtime efficiently switches between tasks, using very little overhead. 
    This allows the program handle many more requests with the same amount of resources.

### - Summary 

* There are many models of execution (Sequential, thread, process and asynchronous ):
    1. Threads are an abstraction provided ( and scheduled ) by the OS. They usually involve pre-emptive
       multitasking, are parallel by default, and have fairly high overhead of management and context
       switching.

    2. Asynchronous Programming is managed by user-space runtime. Multi-tasking is cooperative. It has lower
       overhead then threads, but feels a bit different to programming with threads since it uses different
       programming primitives (`async` , `await` and futures, rather than firt-class threads )

* Concurrency and parallelism are different but closely related concepts.
    1. Concurrency is about the ordering of computation (operations are concurrent if their order of
       execution cannot be observed )
    2. Parallelism is about computing on multiple processors ( operations are parallel if they are literally
       happening at the same time).

* Both OS threads and Asyn programming provide concurrency and parallelism; async programming can also offer
  constructs for flexible or fine-gain concurrency which are not part of most of operating systems' threads
  API.

## Rust approach:

- Rust uses async/await and futures to let your program run many tasks at the same time without blocking.
  When you write an `async fn`, Rust turns it into a **small state machine** that can pause and continue 
  later. 

-  An async runtime like Tokio, smol, or async-std runs these **futures**.  
  It keeps an executor (similar to an event loop) that checks which tasks are ready and polls them until 
  they’re done.

- The future makes progress only when its poll method is invoked, typically by an executor provided by 
  runtimes such as Tokio, smol, or async-std.
  
- Executors manage **task queues**, **register wakers**, and drive futures to completion using cooperative
  scheduling. Internally, they may use reactor components (ex: epoll, kqueue, IOCP) to recv readiness events.

- Rust’s ownership, borrowing, and `Send`/`Sync` traits ensure memory safety and data-race freedom, 
  although logical race conditions and deadlocks remain possible if synchronization primitives 
  (Ex: `Arc<Mutex<T>>` are misused. )


### - Key components of Asynchronous Rust:

To work with async programming there are few components that need to understand:

1. `async/await` : These are syntax elements used to define and execute asynchronous functions.

    * `async`: used to mark a function/block of code as asynchronous.

    * `await`: is used to pause the execution of a task until it has completed. 

2. `Future`: This is a value that represents a computation that might not have finished yet. When we write a
   asynchronous function, they return a `Future`. `await` is used to resolve this `Future` and get the
   result once it's ready.

3. **Executor** : This is a runtime that actually drives the execution of the `Future` s. Executors are
   typically provided by frameworks like `tokio` or `async-std`. They manage when and how the tasks are
   running.

4. **Task** : An asynchronous operation is often represented as a task that is executed by an executor. This
  is how Rust's async runtime schedules tasks. 
  To distinguish concurrency from that offered by threads, sequence of execution in async concurrency is
  called "task", and its to note that the way task is executed, scheduled, and represented in memory is very
  different to a thread but for a high level intuition, it can be useful to think of task as just like
  threads, but managed entirely within the program rather than by the OS.

In an async system, there a scheduler which decides which task to run next (it's part of the program, not 
part of the OS). However, the scheduler cannot pre-empt a task. Instead a task must voluntarily give up 
control and allow another task to be scheduled. 
Because tasks must cooperate (by giving up control), this is called cooperative multitasking.


### When to use Rust async:

Not all problems require asynchronous programming. 
It’s most beneficial when you have tasks that involve waiting for external resources, like network calls, 
file I/O, or user input. 
If your program is CPU-bound (i.e., performing lots of computations), then async programming might not 
provide significant benefits and could actually add complexity.

#### - When to use async:

- **I/O bound task** : These involve interacting with external systems (ex:web servers, db, file systems).

- **Scalable Servers** : If you're writing a server that needs to handle many requests concurrently (like a
  web server or network service).

- **Non-blocking operations** : If you need to perform background tasks (e.g., periodic checks or background
  jobs ) without blocking the main application flow.

Also Async programming offers the programmer fine-grain control over how tasks are executed ( level of
parallelism and concurrency, control flow, scheduling and so forth ). => Async programming can be expressive
as well as ergonomic for many uses. 
Async also has a concept called cancellation and supports many different flavours of concurrency (expressed
using constructs including `spawn` and its variations `join`, `select`, `for_each_concurrent` ..) These
allow composable and reusable implementations of concepts like timeouts, pauses and throttling.

In Short Async programming allows you to achieve concurrency without the overhead of managing multiple
threads, making the program more efficient and optimal usage of CPU, achieving high level of concurrency in
I/O tasks.

Note: Rust Async is developed for a while and its still not "finished" part of the language. 
Async Rust (at least the parts available in the stable compiler and standard libraries) is reliable and 
performant. However there are some missing parts and rough edges. For most of the missing parts there are
workarounds. ( ex: working with async iterator ( also known as **streams** ) is some times hard to
undersand.) 

---

#### - A Basic Example

A basic async function in Rust. ( example has no concurrency and does not really take advantage of being
async )
```bash
cargo add tokio features=full
```
```rust
// Define async function
async fn greet() {
    println!("Hello, world!");
}

#[tokio::main] //boilerplate which lets us write `async fn main` 
async fn main() {
    // Call an async function and lets us wait it result.
    greet().await;
}
```
- To define a function async: use the key word `async` 
- call await on the async function in main. ( rust does not do anything to a async function unless its
  `await` ed )

This example is simple, but it shows the basic structure of using async/await in Rust. 
You could add more asynchronous operations inside the `greet` function (e.g., network calls, I/O), and Rust
will handle them efficiently without blocking other tasks.

---

####  - How Does Asynchronous Code Work in Rust?

Behind the scenes, Rust compiles async functions into state machines. When you write an async function, 
the compiler rewrites it into code that is executed in steps. Each time the function hits an `await`, the 
state machine is paused, and the rest of the function is executed once the awaited task finishes. 
This allows Rust to perform other tasks while waiting.

For example, when you call an async function, the Rust compiler creates a `Future`, which represents a task 
that may not have completed yet. The `await` expression tells Rust to pause and wait for the result of that 
task before continuing the execution.

---

Enslavement  model 

#### - A Practical Example: Async HTTP Request

Let’s extend our example to something more practical — performing an asynchronous HTTP request. 
Here's a simplified example using `reqwest` and `tokio`.

1. Add the dependencies to your `Cargo.toml`:

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
reqwest = "0.11"
```

2. Write the program:

```rust
use tokio;

#[tokio::main]
async fn main() {
    let response = reqwest::get("https://www.rust-lang.org")
        .await
        .unwrap();
    
    println!("Response: {}", response.status());
}
```
```bash
$ cargo run 
Response: 200 OK
```

In this example:

* `tokio::main` is a procedural macro that sets up the Tokio runtime for us. 
  It marks the entry point of our async program.

* `reqwest::get().await` performs an async HTTP GET request to the specified URL. 
  The `await` pauses execution until the request is complete.

By using async, we can make the request without blocking the main thread, allowing our program to handle 
other tasks efficiently.


## Understanding `async` and `await` in Rust

---

###  `async` and `await`

These two are fundamental to asynchronous programming.

#### - The `async` keyword

The `async` keyword is used to define an **asynchronous function**. When you mark a function with `async`, 
it means that the function will be executed asynchronously, allowing the program to continue executing other
tasks while waiting for this function to complete.

```rust
async fn example() {
    println!("This is an async function!");
}
```

- An asynchronous function doesn’t immediately return its result. 
- Instead, it returns a **future** — a type that represents a value that will eventually be computed. 
- So when you call an async function, you don’t get the result right away. Instead, you get a **future** 
  that can be awaited.

#### - The `await` keyword

The `await` keyword is used inside an `async` function to pause execution until the result of an 
asynchronous operation is ready. 

When you `await` on a future, Rust will suspend the current function's execution and let other tasks run 
until the awaited future completes.

```rust
async fn example() {
    let future_value = async {
        42
    };

    // Await the result of the future.
    let result = future_value.await;
    println!("The result is: {}", result);
}
```

In the example above:

1. `future_value` is an asynchronous block that computes `42`.
2. `await` keyword pauses the fun’s execution until the future returned by `future_value` is resolved.
3. Once the future is complete, it returns the value `42`, which we can use.

#### - Asynchronous Function Return Types

When you define an async function, its return type is always a **future**, even if the function seems to 
return a simple value. 

In the example above, `async { 42 }` is actually returning a `Future<Output = i32>`. 
This is an important detail because the return type of an async function is not just the return value, but 
the type of the future that will eventually contain the value.

To make this clearer, let’s look at this in terms of function signatures:

```rust
async fn get_number() -> i32 {
    42
}
```

In this case, `get_number` actually returns a `impl Future<Output = i32>`. 
This means we cannot directly use the result; we need to `await` it.

#### - Using `await` outside of async functions

You can only use `await` inside an `async` function or an async block. 
If you try to use `await` in the main function or outside of an async context, the compiler will raise an 
error. 

This is because the Rust runtime needs a context for managing asynchronous operations, and by default, 
the `main` function is synchronous.

However, starting from **Rust 1.39**, you can use `#[tokio::main]` or `#[async_std::main]` 
(from async runtimes like Tokio or Async-std) to allow asynchronous execution in your main function.

```rust
#[tokio::main]
async fn main() {
    let result = get_number().await;
    println!("The number is: {}", result);
}
```

In this example:

* `#[tokio::main]` is an attribute macro that sets up an async runtime for the `main` function.
* Inside `main`, you can now use `.await` to wait for futures.

### - How does `async` work under the hood?

At a high level, `async` functions are compiled into state machines. 
This means that when you write an async function, you’re not writing code that runs immediately, but 
instead code that is transformed into a state machine that can yield control back to the caller when it 
needs to wait for something.

When you call an `async` function, Rust generates a state machine that keeps track of the execution progress
and the states of the future. When you `.await` a future, the state machine is suspended, allowing other 
tasks to run in the meantime. 
Once the awaited operation completes, the state machine picks up where it left off.

This allows Rust to achieve **zero-cost abstractions** for asynchronous programming, meaning it doesn’t add 
any overhead beyond what’s necessary for asynchronous execution.

### - The `async`/`await` and `Future` types

Now that we’ve covered the basics, let’s talk about the `Future` type in more detail. 
A future is a value that will be computed at some point in the future. 
You can think of it as a placeholder for a value that’s not available yet but will be at some later point.

* `Future` is a trait, and `async` functions implicitly return types that implement this trait.
* Futures in Rust are **pollable**. This means that you can periodically check if a future is ready, or if 
  it’s still in progress.

```rust
use std::future::Future;

async fn fetch_data() -> String {
    "data".to_string()
}

fn poll_future() {
    let future = fetch_data();
    let mut fut = Box::pin(future);
    // Poll the future, checking if it's ready
    if let Poll::Ready(data) = fut.as_mut().poll() {
        println!("Got the data: {}", data);
    }
}
```

In the above example:

1. The `fetch_data()` async function returns a future.
2. We “poll” the future to check if the result is ready.

Normally, `.poll()` is used by async runtimes like **Tokio** or **async-std**, but you can also use it 
manually to gain deeper control over the execution of futures.

---

### - Summary of `async`/`await` in Rust:

* **`async`**: Defines an asynchronous function or block that will return a **future**.

* **`await`**: Pauses the current execution until the future is resolved.

* **Future**: A value that represents the result of an asynchronous operation that hasn’t completed yet.

* **State Machine**: The compiler transforms async functions into state machines to manage suspension and 
  resumption of execution.


## More on async and await:


### - Intro and recap
- `async` on a function is annotation
- `await` is a operator used in expression. 

Rust’s `async` / `await` system gives:
    - non-blocking I/O 
    - efficient task scheduling 
    - safe concurrency (no data races)
    - code that looks like normal synchronous Code 

- What async functions are, how `.await` works and How to run async tasks with async runtime:

- Future: 
    A *future* is an object representing a value that will be available later.

    - A future implements the `Future` trait.
    - A future is **lazy**: nothing happens until it is *polled*.
    - Polling is done by an **executor** inside an async runtime.

=> 

```rust
let f = some_async_function(); // <--- does NOT run it!
```

This only **creates** a future.
To *run* the future, you must `.await` it or spawn it.


- What `async fn` Really Does

```rust
    async fn fetch_number() -> u32 {
        42
    }
```

This does **not** create a function that returns `u32`.

Instead it becomes:

```rust
    fn fetch_number() -> impl Future<Output = u32>
```

The compiler transforms the body of the function into a **state machine** that the runtime can drive by polling.

So calling:

```rust
let fut = fetch_number();
```

Creates a future.

Running it:

```rust
let value = fetch_number().await;
```

Polls it until it finishes and returns the final result.

- What `.await` Really Does

`.await`:

1. **Polls the future**
2. If the future is **ready**, it returns the result immediately
3. If the future is **not ready**, it yields back to the runtime:

   * Your task pauses
   * Other tasks get to run

Important: Rust uses **cooperative multitasking**.
A future *must* `.await` something to allow other tasks to run.

- Runtime and its role:

Rust's standard library **does not** ship with an async runtime.

You must use one such as:

- **Tokio** (most popular for production)
- **async-std**
- **smol**
- **futures executor** (basic)

These run times provide:

- **executor** (polling future tasks)
- **reactor** (I/O, timers)
- **scheduler** (task switching)

They are essential to actually run async code.

### - Example Using Tokio
Minimal example using Tokio:

```rust
#[tokio::main]
async fn main() {
    say_hello().await;
}

async fn say_hello() {
    println!("Hello from async Rust!");
}

```

- `#[tokio::main]` creates and starts a Tokio runtime
- `main` becomes asynchronous
- Calling `say_hello().await` *runs* the future

### - Asynchronous I/O Example

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = mini_redis::client::connect("127.0.0.1:6379").await?;

    client.set("foo", "bar".into()).await?;
    let result = client.get("foo").await?;

    println!("Got: {:?}", result);

    Ok(())
}
```

Several key ideas show up here:

* `.await` is used for **network I/O**
* operations return `Future`s
* `?` works normally inside async functions

### - Spawning Concurrent Tasks

You can run multiple async functions at once using `tokio::spawn`.

```rust
use tokio::time::{sleep, Duration};

async fn task_a() {
    sleep(Duration::from_millis(500)).await;
    println!("Task A done");
}

async fn task_b() {
    sleep(Duration::from_millis(300)).await;
    println!("Task B done");
}

#[tokio::main]
async fn main() {
    tokio::spawn(task_a());
    tokio::spawn(task_b());

    sleep(Duration::from_secs(1)).await; // wait to let tasks finish
}
```

Output:

```
Task B done
Task A done
```

Notice:

* The tasks run concurrently
* Ordering reflects timing, not code order


### - Waiting for Tasks with `JoinHandle`

Spawning returns a handle that you can `.await`.

```rust
#[tokio::main]
async fn main() {
    let h1 = tokio::spawn(task_a());
    let h2 = tokio::spawn(task_b());

    h1.await.unwrap();
    h2.await.unwrap();

    println!("Both tasks completed.");
}
```

This lets you:

* wait for background work
* get return values
* handle errors from spawned tasks

### - Common Mistakes

#### Mistake 1: Forgetting to `.await`

```rust
do_work();   // does nothing
```

✔ Fix:

```rust
do_work().await;
```

---

#### Mistake 2: Blocking inside async code

Never call blocking functions like:

* `std::thread::sleep`
* expensive CPU loops
* synchronous file/network operations

They block the executor thread.

✔ Fix: use async versions (`tokio::time::sleep`, async I/O APIs)

---

####  Mistake 3: Expecting implicit parallelism

Async does **not** mean parallel by default.

* Async → concurrent (shared thread(s))
* Parallel → many threads running simultaneously

To run work in parallel, use:

```rust
tokio::task::spawn_blocking(...)
```

or a multi-threaded runtime.

### Exercise 1: Simple async function

Write an `async fn` that returns a random number and `.await` it in `main`.

```rust
use rand::Rng;

async fn gen_rand() -> u32 { 
    //Simulate async work (optional)
    // tokio::time::sleep(std::time::Duration::from_millis(1000)).await 

    //let mut rng = rand::thread_rng();
    let mut rng = rand::rng();
    // rng.gen_range(100..1000)  // 100 milli sec to 1 sec
    rng.random_range(100..1000)  // 100 milli sec to 1 sec
}

#[tokio::main]
async fn main (){
    let n = gen_rand().await;
    println!("Random num: {}", n);
}
```

### Exercise 2: Concurrent timers

Spawn three tasks that sleep for different durations and print in completion order.

```rust 
use rand::Rng;
use tokio::time::{sleep, Duration};

async fn random_number() -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(0..100)
}

#[tokio::main]
async fn main() {
    // Use your async function
    let n = random_number().await;
    println!("Random number: {}", n);

    // Spawn three tasks with different durations
    let handle1 = tokio::spawn(async {
        sleep(Duration::from_secs(3)).await;
        "task 1 finished after 3s"
    });

    let handle2 = tokio::spawn(async {
        sleep(Duration::from_secs(1)).await;
        "task 2 finished after 1s"
    });

    let handle3 = tokio::spawn(async {
        sleep(Duration::from_secs(2)).await;
        "task 3 finished after 2s"
    });

    // Collect them and print in completion order
    let mut tasks = vec![handle1, handle2, handle3];

    while !tasks.is_empty() {
        // `select!` waits for whichever task finishes first
        let (completed, _, remaining) = futures::future::select_all(tasks).await;
        println!("{}", completed.unwrap());
        tasks = remaining;
    }
}
```

### Exercise 3: Join results

Spawn two async tasks that return numbers. Await their JoinHandles and print their sum.

### Exercise 4: Error propagation

Write an async function that returns a `Result` and use `?` to handle errors.
### Exer
