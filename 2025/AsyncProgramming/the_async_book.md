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
  can handle many things at once, without running them in parallel ( as done in threads based approach).

  Concurrency in *async* is achieved by allowing to switch between tasks during "idle" time, like waiting for
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

---
Note: Event-Loop: 
A programming construct that continuously monitors for and responds to events or messages. 
It is a core mechanism in event-driven programming, allowing a program to perform non-blocking  operations
and handle multiple tasks concurrently, even if it runs on a single execution thread.

Event-Loop operates as an infinite loop that manages and orchestrates tasks using several key components:-

Call Stacks(LIFO) , Event Queue/Callback Queue (FIFO), Web APIs and background processes.

---



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
    So In Rust Async IO are handled by the async runtime( the scheduler is also part  of the runtime )
    The task of doing IO requests IO from the runtime, the runtime requests IO from the OS but the OS 
    returns control to the runtime. 
    The runtime pauses the IO task and schedules other tasks to get work done. When the IO is done, the 
    runtime wakes up the IO task so it can continue execution with the result of the IO.

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
allow composeble and reusable implementations of concepts like timeouts, pauses and throttling.

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

- An asynchronous function does not immediately return its result. 
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
- `future::future::select_all` is being used to wait for tasks to finish in the order they complete.
- `tokio::spawn` is used to run async tasks concurrently.
- TODO: Can use `unwrap_or_else` or `Result` to handle error more robustly

### Exercise 3: Join results

Spawn two async tasks that return numbers. Await their JoinHandles and print their sum.
```rust  
use rand::Rng;
use tokio::spawn;

// Function to generate a random number
async fn random_number() -> Result<u32, String> {
    let mut rng = rand::thread_rng();
    // Simulating a potential error with a 10% chance of failure
    if rng.gen_bool(0.1) {
        Err("Failed to generate random number".to_string())
    } else {
        Ok(rng.gen_range(0..100)) // Returns a random number between 0 and 99
    }
}

#[tokio::main]
async fn main() {
    // Spawn two tasks that each return a random number with error handling
    let handle1 = spawn(async {
        random_number().await.unwrap_or_else(|e| {
            println!("Error in task 1: {}", e);
            0 // Return 0 in case of error
        })
    });

    let handle2 = spawn(async {
        random_number().await.unwrap_or_else(|e| {
            println!("Error in task 2: {}", e);
            0 // Return 0 in case of error
        })
    });

    // Await the tasks and get the results
    let result1 = handle1.await.unwrap(); // Unwrap the result of task 1
    let result2 = handle2.await.unwrap(); // Unwrap the result of task 2

    // Calculate the sum of the random numbers and print it
    let sum = result1 + result2;
    println!("Random number 1: {}", result1);
    println!("Random number 2: {}", result2);
    println!("The sum of the random numbers is: {}", sum);
}
```
- `tokio::spwan` to generates two async tasks ( each task get randown number )
- both tasks are spawned concurrently ==> they run in parallel, each task generates a randon number. 
- `await` await for the completion of each task using `.await` on the `JoinHandle` returned by `tokio::spawn` 
  This gives the result of the task.
- Inside the `spawn` tasks, we use `unwrap_or_else` to handle error:
  - If task completes successfully, random number is returned.
  - If task encounters error prints error and returns 0 as fallback value.
  - `unwrap_or_else` : method handles the Result by either unwrapping the `Ok` value or executing the 
    closure provided for `Err`. closure, print the error and return a default value of 0.
    This is better error handling.

### Exercise 4: Error propagation

Write an async function that returns a `Result` and use `?` to handle errors.

- `?` operator is used to propagate errors, it allows  you to return early from a function if an error 
   occurs, and it automatically converts an error from one type into another if necessary 
   (such as from Result<T, E> to Result<T, E2>).

```rust 
use rand::Rng;
use tokio::spawn;
use std::fmt;

// Custom error type
#[derive(Debug)]
enum RandomNumberError {
    GenerationError,
}

impl fmt::Display for RandomNumberError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to generate random number")
    }
}

// Async function that returns a Result and uses `?` to propagate errors
async fn random_number() -> Result<u32, RandomNumberError> {
    let mut rng = rand::thread_rng();
    
    // Simulate a failure with a 10% chance
    if rng.gen_bool(0.1) {
        return Err(RandomNumberError::GenerationError); // Return error if fails
    }
    
    // Return a random number between 0 and 99
    Ok(rng.gen_range(0..100))
}

#[tokio::main]
async fn main() {
    // Spawn two tasks that each return a random number, handling errors using `?`
    let handle1 = spawn(async {
        match random_number().await {
            Ok(num) => num,
            Err(e) => {
                println!("Error in task 1: {}", e);
                0 // Return default value 0 in case of error
            }
        }
    });

    let handle2 = spawn(async {
        match random_number().await {
            Ok(num) => num,
            Err(e) => {
                println!("Error in task 2: {}", e);
                0 // Return default value 0 in case of error
            }
        }
    });

    // Await the tasks and get the results
    let result1 = handle1.await.unwrap(); // Unwrap the result of task 1
    let result2 = handle2.await.unwrap(); // Unwrap the result of task 2

    // Calculate the sum of the random numbers and print it
    let sum = result1 + result2;
    println!("Random number 1: {}", result1);
    println!("Random number 2: {}", result2);
    println!("The sum of the random numbers is: {}", sum);
}
```
- We have defined a custom error type `RandomNumberError` which has a variant `GenerationError` to simulate
  an error during random number generation. We use error type for our `Result`.

- `random_number` function returns a Result<u32, RandomNumberError>

- If the random number generation fails we return an Err(RandomNumberError::GenerationError) and 0 on
  success.

- In `random_number` func, we use the `?` operator to propagate errors if the random number generation fails.
  However, in the current setup, we handle errors directly inside the spawn tasks using `match`.

---

# Rust async Concepts:

The **Rust async runtime** is a key when working with asynchronous code, as it handles the execution, 
scheduling, and management of async tasks. 

It provides the necessary functionality for efficiently performing I/O-bound operations without blocking the
thread, and allows Rust to scale with limited resources (such as a few CPU cores) by handling many tasks 
concurrently. 

### 1. What is async in Rust?

An async function allows the function to return a **future**, which represents a value that will be available
at some point in the future. 

Async functions are non-blocking, meaning they don't tie up a thread while waiting for something (like I/O 
operations) to complete. 
Instead, they return a `Future` that is polled by an executor (usually provided by a runtime).

Example:

```rust
async fn do_something() -> i32 {
    // Some async task
    42
}
```

The `do_something` function returns a `Future<i32>` instead of directly returning `i32`.

### 2. The Role of the Runtime

In Rust, async functions by themselves don’t actually perform any work unless they are executed within a 
runtime. 

The runtime's responsibility is to manage and execute tasks on the system, handle I/O, and ensure that the 
correct tasks are executed at the right time.

The runtime will:

* Schedule tasks: decides which async task should be executed next based on the task's state and resource 
                  availability (such as the CPU or I/O).

* Manage async I/O: Interacts with the OS I/O APIs to manage non-blocking I/O operations (like networking or
                  fs operations). This is key for scaling efficiently when dealing with many I/O-bound tasks.

* Determine task execution on OS threads**: The runtime manages which tasks are executed on which threads. 
                  Typically, multiple async tasks are handled by a smaller number of threads, and the 
                  runtime decides how to distribute them.

Examples of popular runtimes in Rust are:

* Tokio: A full-featured async runtime for Rust, which handles both I/O operations and tasks scheduling.
* async-std ( replaced with smol): A simpler, more lightweight async runtime inspired by the standard library’s synchronous APIs.

### 3. Low-Level Driving and Execution

The **executor** within the runtime is responsible for polling the futures, driving them to completion. 
In Rust, when an async task is awaited (using `.await`), the executor is the entity that decides when the 
task can be resumed. The runtime may use an event loop or other scheduling mechanisms to track and poll 
tasks. 

For example, Tokio uses an internal event loop to manage and schedule tasks asynchronously.

### 4. Scope of the Runtime

* Runtime crate: The term "runtime" can sometimes refer to the entire library or framework that includes the 
                 low-level scheduling and task management tools (such as the **Tokio** crate). 
                 This is more than just the executor—it's the full ecosystem of utilities for async programming.

* For example **Tokio**, the runtime includes not just the executor but also traits for asynchronous I/O 
  (`AsyncRead`, `AsyncWrite`), utilities for timers, networking, file system access, and more.

### 5. Components of a Runtime Crate

Rust async runtimes typically come with various utility traits and components for handling different 
asynchronous tasks:

* I/O traits: Rust provides the `AsyncRead` and `AsyncWrite` traits that are commonly used in I/O-bound 
              async operations. 
              The runtime handles polling these traits and performing the I/O operations non-blocking.

* Channels & Synchronization Primitives: Asynchronous communication between tasks can be managed using 
              async channels (like `tokio::sync::mpsc`) or other concurrency primitives such as mutexes and
              semaphores, designed for async environments.

* Timer functionality: Runtimes like Tokio provide utilities for asynchronous timers (`tokio::time`), which 
              are useful for scheduling events after a delay or for implementing timeouts.

* OS interaction: The runtime is responsible for interacting with the OS, such as managing signals 
             (ex: Ctrl+C handling) and managing processes.

* Monitoring/Observation tools: Some runtimes offer tools to monitor and log async tasks, track the execution 
             of futures, and offer observability.

### 6. Why Choose Different Runtimes?

Since Rust doesn’t enforce a particular async runtime, you get the freedom to choose one that best fits your
needs:

* Tokio: Popular and feature-rich runtime with extensive support for I/O, timers, and async features, making 
         it suitable for large-scale applications with complex async needs.

* async-std**: Lightweight alternative, mimicking the synchronous standard library API and offering a 
         simpler approach. ( depreciated )

* smol: Another lightweight async runtime that focuses on simplicity and minimalism.


Each runtime has trade-offs in terms of performance, ease of use, and features, so choosing one depends on 
your project's specific requirements (e.g., performance, ecosystem, features, community support).

### Summary of Async Task Runtime Responsibilities

* Scheduling tasks: 
    Deciding which async tasks to run, when to poll them, and where to run them (on specific threads).

* Managing I/O: 
    Handling async I/O operations efficiently by interacting with the OS, using non-blocking system calls.

* Low-level task management: 
    Ensuring tasks are polled, executed, and completed as resources become available (such as CPU or I/O).

* Utility functions and synchronization primitives: 
    Providing essential tools for async operations, such as async I/O traits, locks, timers, and 
    communication channels.

The runtime acts as the orchestrator of async tasks in Rust, ensuring efficient execution while allowing you
to write scalable and non-blocking code without needing to manage threads or low-level details yourself.

--- 

# Overview of `futures-rs` and its place in the Rust async ecosystem**:

---

### What is `futures-rs`?

`futures-rs` (the **`futures` crate**) is the *foundational library* for asynchronous programming in Rust.
It provides the **core abstractions**— `Future`, `Stream`, `Sink`, combinators, executors, and utility 
traits—that the entire Rust async ecosystem is built on.

Before `async/await` syntax was stabilized, *this crate defined the original future system*. 
Even today, it remains the **low-level glue** that connects runtimes, libraries, and async components.

---

## What `futures-rs` Provides

`futures` is **runtime-agnostic**. It does *not* include a full async runtime like Tokio. 
It provides the foundations for asynchronous programming in Rust.
Instead, it provides:

* **Core traits**:

  * `Future` (pre-std version)
  * `Stream` (async iterators)
  * `Sink`

* **Combinators** (map, join, select, etc.)
* **Utilities** for polling, pinning, channels, and tasks
* A **lightweight executor** (mostly for testing or single-thread cases)
* Adapters and compatibility layers

Because it is runtime-agnostic, `futures` works with any executor: **Tokio**, **async-std**, **smol**, etc.

This crate makes sense when you are thinking about implementing your own custom runtime.

---

## **How It Fits with Tokio and Other Runtimes**

Think of the ecosystem like this:

### **`futures-rs` → Foundations and traits**

Common async building blocks used everywhere.

### **Tokio / async-std / smol → Full async runtimes**

They build on top of the concepts from `futures-rs`, providing:

* task scheduling
* I/O (networking, fs)
* timers
* threads

Tokio re-implements its own versions of some traits (`AsyncRead`, `AsyncWrite`), but *still interoperates* 
with `futures` through compatibility layers (`tokio-util`, `futures-util`).

---

## **When to Use `futures-rs`**

You might use it when:

* building **runtime-independent libraries**
* needing fine-grained control over futures, streams, combinators
* writing abstractions that should run on any executor
* testing async components without a full runtime

Most application developers will primarily use **Tokio**, but library authors often depend on `futures-rs`.

---

## **Summary**

`futures-rs` is the **standard toolkit for low-level async building blocks in Rust**.
Runtimes like **Tokio**, **async-std**, and **smol** build on these concepts (though sometimes with custom 
versions), forming an ecosystem where `futures` provides *interfaces*, and runtimes provide the *execution*.

It overlaps with runtimes, but remains the **neutral core layer** that ensures interoperability and shared 
async vocabulary across the Rust ecosystem.

---


# `await` 


`await` is a keyword used to **wait for the completion of a future**. 

A **future** represents a computation that will eventually produce a result, but not immediately. 
When you call `.await` on a future, the runtime **polls** it to check if the result is ready. 
If the result is available, it’s returned immediately. If not, the current task yields control back to the 
scheduler, allowing other tasks to execute while waiting.

### How `await` Works

The syntax for `await` is straightforward: you call it on a future with `.await`, like so:

```rust
some_future.await
```

`await` is a **postfix operator**, meaning it can be used seamlessly in chains of method calls or when 
accessing fields:

```rust
let result = some_object.method().await;
```

### Example: Simple Async Functions

Here’s an example of two async functions:

```rust
// An async function that completes immediately
async fn add(a: u32, b: u32) -> u32 {
  a + b
}

// An async function that waits for 1 second before adding
async fn wait_to_add(a: u32, b: u32) -> u32 {
  sleep(1000).await; // Simulating a delay (e.g., I/O operation)
  a + b
}
```

* Calling `add(15, 3).await` immediately returns `18`.
* Calling `wait_to_add(15, 3).await` will eventually return `18`, but while it’s waiting (due to `sleep`),
  **other tasks can execute**.

This demonstrates how `await` makes async operations **non-blocking**: the program doesn’t stop to wait but
instead yields control, letting other tasks run while waiting for the result.

### Key Points on `await`

* **Async functions return futures**: Calling an async function like `add` or `wait_to_add` returns a
  `Future`, but the code inside the function doesn’t run until you use `.await`.

* **No result without `.await`**: If you don’t use `.await` on an async function, it won’t run. 
   It simply returns a future that is not executed. This is different from some languages where async 
   functions start executing immediately upon calling.

* **Cooperative multitasking**: When `await` yields control, it’s a form of **cooperative multitasking** 
  — meaning tasks must voluntarily give up control to allow others to run. This is important for achieving  
  concurrency without preemption (i.e., no forceful task switching).

### Task Scheduling and Execution

In **pure sequential code**, each function call runs to completion before the next one starts. 
In an **async context**, things work differently. When you call an async function, it doesn’t execute right
away. 

Instead, it returns a future that must be **polled** by the runtime. If the future is not ready, the current
task **yields control** to allow another task to run.

`await` is an operator that helps **drive** the future forward:

* If the future is ready, it returns the result.
* If not, it **suspends** the current task and allows another task to run.

You can only use `await` inside an **async context**, like inside an async function or a block that’s part
of an async task. Without an async runtime, there's no way to actually *execute* the future, so async tasks
can’t run independently.

---

#### Examples in Action

#### Basic `say_hello` Example

```rust
// An async function to print a message
async fn say_hello() {
    println!("hello, world!");
}

#[tokio::main] // Initializes the Tokio runtime for async main
async fn main() {
    say_hello().await; // The task runs when we await it
}
```

In the example above, `say_hello()` is an async function that doesn’t do any computation, but when we 
call `.await`, the task is run within the Tokio runtime. If we omit `.await`, the `say_hello` function 
never runs.

#### Realistic `client` Example (with I/O)

```rust
#[tokio::main]
async fn main() -> Result<()> {
    let mut client = client::connect("127.0.0.1:6379").await?;

    client.set("hello", "world".into()).await?;
    let result = client.get("hello").await?;

    println!("got value from the server; result={:?}", result);

    Ok(())
}
```

In this example, we're interacting with a Redis server. 
The `connect`, `set`, and `get` methods are async and must be **awaited** to execute. 
The program doesn't block while waiting for these I/O operations — other tasks could be run concurrently if
present.

#### Concurrency in Action

Here’s an example where we introduce a delay and see how the task can yield control:

```rust
use tokio::time::{sleep, Duration};

async fn say_hello() {
    print!("hello, ");
}

async fn say_world() {
    println!("world!");
}

#[tokio::main]
async fn main() {
    say_hello().await;
    sleep(Duration::from_millis(1000)).await; // Task is paused for 1 second
    say_world().await;
}
```

Here, when `say_hello().await` is called, the task prints `"hello"`. Then, we call `sleep().await` to 
simulate a 1-second delay. While this task is paused, the Tokio runtime could potentially schedule other 
tasks. After the 1-second pause, `say_world().await` is executed, printing `"world"`. 

The key takeaway is that `sleep().await` allows **other tasks to execute during the wait** — a feature of
**concurrent execution**.

---

### Conclusion

The `await` keyword in Rust is a powerful tool for async programming. It:

* Allows you to **wait for a result** from a future without blocking the entire thread.
* Works by **yielding control to the scheduler** when the result isn’t ready, enabling other tasks to run.
* Is used within **async functions**, and is essential for making async code **execute**.

It’s also important to note that **sequential async code** (like calling `await` one after another) doesn’t
introduce concurrency — tasks must be designed to run concurrently for that benefit.

As you build async applications, `await` allows you to write **efficient, non-blocking code**, while the
runtime handles scheduling and executing tasks in parallel or concurrency where needed.

--- 

# Spawning Tasks

In async programming, **tasks** represent units of work that the runtime schedules and executes. 

**Spawning a task** means creating a new unit of work that can run concurrently with others, allowing for 
**parallel execution** if the runtime supports it.

Rust’s **Tokio runtime** provides a function `tokio::spawn` to spawn an asynchronous task.  This is similar
to threads spawning : using `std::thread::spawn`

This allows you to **run async functions concurrently** on potentially different threads. 
Note that **spawn** is a function of the **Tokio runtime**, not part of Rust's standard library, 
since tasks are a runtime concept.

### Basic Example: Spawning Async Tasks

```rust
use tokio::{spawn, time::{sleep, Duration}};

async fn say_hello() {
    sleep(Duration::from_millis(100)).await;
    println!("hello");
}

async fn say_world() {
    sleep(Duration::from_millis(100)).await;
    println!("world!");
}

#[tokio::main]
async fn main() {
    spawn(say_hello());
    spawn(say_world());
    // Give time for tasks to run
    sleep(Duration::from_millis(1000)).await;
}
```

* The `spawn` function takes an async **future** and schedules it as a **task** to run concurrently.

* The tasks will be executed **concurrently**, meaning **both "hello" and "world!"** may print in random 
  order (a race condition).

* These tasks can **run on separate threads** depending on the runtime configuration 
  (Tokio’s default is multi-threaded).

### Key Concepts:

* **Futures**: An async function returns a future.

* **Tasks**: A future, once spawned using `tokio::spawn`, becomes a task managed by the runtime.

* **Threads**: Tasks may run on different OS threads, but the primary purpose is to achieve **concurrency**
  and potentially **parallelism** when scheduled on separate threads.

When you call `spawn`, you create concurrency.
Each task can run on its own, and **awaiting** them (like in the next section) can allow you to coordinate 
when each task completes.

---

# Joining Tasks

When you spawn a task, you can get its **result** by waiting for it to finish, which is known as **joining** 
the task. 

This is similar to joining threads in other languages, where you wait for the thread to finish execution.

`tokio::spawn` returns a **JoinHandle**, which represents the task. 

By awaiting the `JoinHandle`, you can **wait for the spawned task to finish** and retrieve its result.

### Basic Example: Joining Tasks

```rust
use tokio::{spawn, time::{sleep, Duration}};

async fn say_hello() {
    sleep(Duration::from_millis(100)).await;
    println!("hello");
}

async fn say_world() {
    sleep(Duration::from_millis(100)).await;
    println!("world");
}

#[tokio::main]
async fn main() {
    let handle1 = spawn(say_hello());
    let handle2 = spawn(say_world());

    // Wait for both tasks to finish
    let _ = handle1.await;
    let _ = handle2.await;

    println!("!");
}
```
1. 
- `async fn main()`: This declares the main function as asynchronous.
  The `async` functions return a Future.

- `#[tokio::main]`: A macro provided by the Tokio. It wraps the main function and sets up the necessary 
  asynchronous runtime (the Tokio scheduler). This allows you to call `.await` within main and ensures that 
  the asynchronous tasks are executed.

2. 
    `let handle1 = spawn(say_hello());` and `let handle2 = spawn(say_world());`
    
- The spawn() function takes an async block or Future and immediately schedules it to run on the executor 
  (the Tokio runtime) concurrently with other tasks.

- `say_hello()` and `say_world()` are other `async` functions defined that are executed right away.

- `spawn()` returns a `JoinHandle`  (handle1 and handle2). This handle is used later to wait for the spawned
  task to complete and potentially retrieve its return value.

3. 
        `sleep(Duration::from_millis(90)).await;`

- `sleep()` is Tokio's asynchronous equivalent of pausing. It doesn't block the entire thread; instead, it 
  tells the Tokio runtime to pause this specific task (main) for 100 milliseconds, allowing the runtime to
  switch context and execute other pending tasks (say_hello and say_world) during this time.

- The `.await` keyword is used to pause the execution of the main function until the sleep Future is
  complete.

4. Waiting for Tasks to Complete

    `let _ = handle2.await;` and `let _ = handle1.await;`

-    `.await` on a `JoinHandle` pauses the current task (main) until the corresponding spawned task 
    (ex: handle2) completes.

- The tasks will likely have already finished during the 90ms sleep, but these lines ensure that the main 
  function waits for the tasks to truly finish before the program exits.

-  `let _ =` ignores the potential return value (and the `Result` wrapper) of the tasks.

The execution order here is sequential for the wait operation:  `main` waits for `handle2` to finish, 
then waits for `handle1` to finish. However, the tasks themselves (say_hello and say_world) are executing 
concurrently in the background.

#### Key Points:

* JoinHandle: 
    A `JoinHandle` is a **future** that represents the spawned task. 
    By awaiting it, you wait for the task to finish and obtain its result.

* Concurrency: 
    Even though the tasks run concurrently, `await` on the `JoinHandle` ensures that `main` doesn’t exit 
    until all tasks complete.

* Sequential Behavior: 
    If you remove the `await` on `JoinHandle`, the tasks will still be spawned, but you might lose control 
    over their completion order.

---

# `JoinHandle`

A `JoinHandle` is a struct that allows you to **track** the result of a spawned task. 
When you spawn a task using `tokio::spawn`, it returns a `JoinHandle` which you can await to wait for the 
task's completion.

### What is a `JoinHandle`?

* A `JoinHandle` is a **future** that you can **await** to get the result of the spawned task.
* It can hold the task’s result (`Result<T, E>`), which you can inspect after the task completes.
* If the spawned task **panics** or is **aborted**, the `JoinHandle` will return an error.

### Example: Using `JoinHandle` to Wait for Task Completion

```rust
use tokio::{spawn, time::{sleep, Duration}};

async fn say_hello() {
    sleep(Duration::from_millis(100)).await;
    println!("hello");
}

async fn say_world() {
    sleep(Duration::from_millis(100)).await;
    println!("world");
}

#[tokio::main]
async fn main() {
    let handle1 = spawn(say_hello());
    let handle2 = spawn(say_world());
    
    let _ = handle1.await; // Wait for task 1 to finish
    let _ = handle2.await; // Wait for task 2 to finish

    println!("!");
}
```

* Each `spawn` returns a `JoinHandle`, which is then awaited.
* Result Handling: If the spawned task returns a result (e.g., `JoinHandle<String>`), `await` will 
  return that result. In the example, the tasks return `()`, so `JoinHandle<()>` is used.
* Error Handling: If the task panics or is aborted, the `JoinHandle` returns an error. 
  You can unwrap it or handle it based on your needs.

### Key Concepts:

* Generic Type: `JoinHandle<T>` is generic, where `T` is the type of value returned by the spawned task 
               (e.g., `JoinHandle<()>` for tasks with no result).

* Result Handling: Awaiting a `JoinHandle` returns a `Result<T, JoinError>`. You can use `unwrap` or handle
               errors in a more controlled way.

* Panic Propagation: If the task panics, the `JoinHandle` returns a `JoinError`, and the panic is propagated
               to the calling task when you await the `JoinHandle`.

### Summary of Key Differences

* Spawning Tasks: You use `tokio::spawn` to run an async function concurrently in the background. 
  The function returns a `JoinHandle`, but you can ignore it if you don’t need to track the task’s result.

* Joining Tasks: If you want to wait for the spawned task to complete, you can store the `JoinHandle` and
  await it to get the result. This is essential for coordinating the completion of multiple tasks.

* `JoinHandle`: This is a **future** that represents the result of the spawned task. You can await it to 
  wait for the task to complete and handle its result, including any potential errors or panics.

These concepts allow you to build **concurrent programs** where tasks can run in parallel, and the results
of those tasks can be collected or coordinated as needed.

---

# Unit Tests in Async Rust

Testing async code in Rust can be tricky because unit tests themselves are not asynchronous by default. 
However, **Tokio** (and other runtimes) provide a convenient solution to make unit tests async using a 
special attribute.

### Using `#[tokio::test]` for Async Tests

To write async unit tests, you can annotate your test function with `#[tokio::test]`, which will 
automatically set up a Tokio runtime for the test, allowing you to `await` async code directly in the test 
body.

example:

```rust
#[tokio::test]
async fn test_async_function() {
    let result = async_function().await;
    assert_eq!(result, expected_value);
}
```

### Key Points:

* The `#[tokio::test]` attribute automatically sets up the necessary Tokio runtime for running the async test.
* You can use `await` inside the test function just like in any other async context.
* Unit tests in async contexts still follow the standard testing framework in Rust, with assertions and 
  setup/teardown as needed.

### Advanced Testing Topics:

* **Race Conditions**: You can test for conditions where tasks are racing for resources.
* **Deadlocks**: You can simulate and test how your code handles deadlocks.
* **Mocking async functions**: You may use libraries like `mockall` or `mockito` to mock async behavior.

---

# Blocking and Cancellation in Async Code

## Blocking in Async Code

In async Rust, **blocking** refers to situations where a thread or task is prevented from making progress 
because it is waiting for some resource or operation (usually I/O) to complete. 
However, since Rust async code relies on cooperative multitasking, blocking can lead to problems if a task
blocks the thread, preventing other tasks from making progress.

There are two types of blocking to consider:

1. **Blocking I/O**
   This is when a task is waiting for I/O operations to complete (e.g., reading from a file or making an 
   HTTP request). If you use synchronous I/O in an async task, the entire thread can be blocked, which 
   affects the runtime’s ability to schedule other tasks on that thread.

2. **Blocking Computation**
   This happens when an async task performs heavy computation without yielding control back to the runtime.
   As a result, it prevents the scheduler from running other tasks.

### How to Handle Blocking I/O in Async Code

Async runtimes like **Tokio** have mechanisms to prevent blocking I/O from disrupting the event loop. 
You should always use **non-blocking** I/O inside async tasks. Rust's standard library provides blocking 
I/O (e.g., `std::fs::read`), but you can use Tokio’s async versions (like `tokio::fs::read`) to avoid 
blocking.

When you must perform blocking I/O, you can offload it onto a separate thread or 
use `tokio::task::block_in_place` to ensure that blocking I/O doesn’t block the entire event loop.

#### Example: Offloading Blocking I/O

```rust
use tokio::task;

async fn perform_blocking_task() {
    let result = task::block_in_place(|| {
        // This will block the thread, but not the async runtime
        std::thread::sleep(std::time::Duration::from_secs(2));
        42
    });

    println!("Blocking result: {}", result);
}
```

Here, `block_in_place` ensures that the blocking task doesn’t interfere with other async tasks, allowing 
the runtime to continue scheduling other tasks.

### Blocking Computation

For long-running computations that block the thread, you should ensure that tasks yield control back to the
runtime regularly. You can use `tokio::task::yield_now()` to voluntarily yield control.

#### Example: Preventing Computation from Blocking

```rust
use tokio::task;

async fn compute_large_task() {
    for i in 0..10 {
        // Simulate a computation
        println!("Computing {}", i);
        // Yield control back to the runtime to allow other tasks to run
        task::yield_now().await;
    }
}
```

Using `yield_now()` ensures that the computation doesn’t monopolize the thread, allowing other tasks to run 
concurrently.

---

## Cancellation in Async Code

Cancellation refers to stopping a future (or task) from executing prematurely. 

In Rust, futures are **poll-driven**, meaning that they will continue executing as long as they are polled,
and they don’t execute unless explicitly driven forward by an async runtime.

There are several ways to **cancel** a future:

### 1. Dropping the Future

* If a future is dropped (i.e., goes out of scope), it is canceled and cannot be polled further. 
  This is the simplest form of cancellation.

### 2. Aborting a Task

* If you want to cancel a running task, you can call `abort` on a task’s `JoinHandle` or use an 
  `AbortHandle` from Tokio.

#### Example: Aborting a Task

```rust
use tokio::{task, time::{sleep, Duration}};

async fn long_running_task() {
    sleep(Duration::from_secs(5)).await;
    println!("Task completed!");
}

#[tokio::main]
async fn main() {
    let handle = task::spawn(long_running_task());
    
    // Abort the task before it finishes
    handle.abort();

    // Awaiting a canceled task results in a JoinError
    let result = handle.await;
    if let Err(e) = result {
        println!("Task was aborted: {:?}", e);
    }
}
```

Here, the task is aborted before it finishes, and `handle.await` returns an error (`JoinError`), indicating 
that the task was canceled.

### Cancellation Tokens (Cooperative Cancellation)

In Tokio, you can use a **CancellationToken** to signal a future to cancel itself. 
This approach requires the future to periodically check if it should stop execution.

#### Example: Using a Cancellation Token

```rust
use tokio::{sync::CancellationToken, time::{sleep, Duration}};

async fn process_with_cancellation(token: CancellationToken) {
    for i in 0..5 {
        if token.is_cancelled() {
            println!("Task was canceled");
            return;
        }
        println!("Processing {}", i);
        sleep(Duration::from_secs(1)).await;
    }
}

#[tokio::main]
async fn main() {
    let token = CancellationToken::new();
    let task = tokio::spawn(process_with_cancellation(token.clone()));

    // Simulate some condition to cancel the task
    sleep(Duration::from_secs(2)).await;
    token.cancel();  // Trigger cancellation

    let _ = task.await;  // Wait for task to handle cancellation
}
```

In this example, the future checks the cancellation token at each step and terminates early if the token is 
canceled.

### Important Considerations for Cancellation

* Cooperative Cancellation: 
  If using a cancellation token, the future itself must check periodically (using `is_cancelled`) and react 
  accordingly. If the future doesn’t check the token, it won’t cancel.

* Non-Cooperative Cancellation: Other methods like aborting or dropping a `JoinHandle` can cancel the 
  task **without** notifying the task itself. The task will not have an opportunity to clean up any resources.

---

### Summary

* Unit Tests: Use `#[tokio::test]` to run async unit tests in Tokio. This macro allows async functions to 
  be written as unit tests, enabling you to `await` directly in the test body.

* Blocking and Cancellation: Avoid blocking I/O and long-running computations in async code. 
  Use mechanisms like `block_in_place` for blocking I/O and `yield_now` for long computations.
  Tasks can be canceled by aborting a `JoinHandle` or using a `CancellationToken` for cooperative cancellation.
