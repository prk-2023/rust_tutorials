# Introduction to Rust Asynchronous Programming

## 1. Why Asynchronous Programming?

- In most programming languages, synchronous operations are the default. 
- In synchronous programming, tasks are executed one after the other. 
- If a task takes time—such as reading from a file or making an HTTP request—the program must wait for that
  task to complete before moving on to the next one. This is called **blocking**.

### Example: Synchronous I/O Operation (Blocking)

Imagine you need to download data from a server. In a blocking scenario, your program would:

1. Send a request to the server.
2. Wait for the server to respond.
3. Process the response.
4. Continue to the next task.

This process blocks the program from doing anything else while waiting for the server's response. 
If there are multiple such I/O operations, your program could become inefficient, especially in systems 
programming where responsiveness and parallelism are crucial.

In **asynchronous programming**, however, the program doesn't wait for operations to finish. 
Instead, it can continue executing other tasks while waiting for the I/O operation to complete.

Asynchronous programming is particularly valuable in contexts such as:

* Networking  : Sending and receiving data over a network can take time, and asynchronous programming 
                allows you to handle multiple requests without waiting for each one to complete.

* File I/O    : Reading or writing files can be slow, especially with large files. Async programming allows
                the system to continue processing other tasks while waiting for I/O.

* Concurrency : You can run multiple tasks in parallel, without blocking threads, which is key in 
                system-level programming where efficiency and resource management are critical.

## 2. Synchronous vs Asynchronous Execution in Rust

Let’s start by distinguishing between synchronous and asynchronous execution in Rust.

### - Synchronous Code

In synchronous code, each statement is executed in sequence, and each step must complete before moving to 
the next one. For example, a simple function to fetch data from a server might look like this:

```rust
fn fetch_data_from_server() -> String {
    // Imagine this function sends a request to a server and waits for the response
    // It's a blocking operation
    let response = send_request_to_server(); 
    return response;
}

fn main() {
    let data = fetch_data_from_server();
    println!("Fetched data: {}", data);
}
```

In this example, `fetch_data_from_server` will block until the request is complete.
If fetching data from the server takes a few seconds, your program is frozen during that time.

Imagine a simple program that needs to read configuration data from 3 different files or network endpoints:
```rust 
fn read_data_synchronous(path: &str) -> String {
    println!("Reading data from {}...", path);
    // Simulate a time-consuming I/O operation (e.g., reading a slow disk)
    std::thread::sleep(std::time::Duration::from_millis(1000)); 
    println!("Finished reading {}", path);
    format!("Data from {}", path)
}

fn main() {
    let start_time = std::time::Instant::now();
    
    // Task 1: Wait 1 second
    read_data_synchronous("config_A");
    
    // Task 2: Wait 1 second (Must wait for Task 1 to finish)
    read_data_synchronous("config_B");
    
    // Task 3: Wait 1 second (Must wait for Task 2 to finish)
    read_data_synchronous("config_C");
    
    let duration = start_time.elapsed();
    // The total time is approximately the sum of all wait times.
    println!("--- Total time elapsed: {:.2?}", duration);
}
// Output: Total time elapsed: ~3.00s
```
- Problem: Even though the 3 reading tasks are independent, the program spends 3 secs doing nothing but
           waiting. The CPU is idle during this I/O waiting period.
- A single thread is blocked and cannot be used for other useful computations.

### - Asynchronous Code

Asynchronous programming allows a program to handle multiple tasks concurrently without creating a new, 
expensive operating system thread for every task.

Core Idea: 
> When a task begins an I/O operation and enters a waiting state, instead of blocking the thread, it pauses
> itself and gives up control of the thread. 
> The thread can then immediately start working on another waiting task.

The process of switching between waiting *tasks* and using a small number of threads efficiently is known 
as **concurrency**.

How this is achieved:
1. Cooperative Multitasking: 
    Unlike threads (which are preemptively managed by the OS), asynchronous tasks voluntarily pause and 
    resume, making them a form of cooperative multitasking.

2. Non-Blocking I/O: 
    Async code uses non-blocking I/O primitives. ( that's reason for many extensions to tokio:fs,net..)
    When a read is requested, the system returns immediately, saying "I don't have the data yet" (instead 
    of pausing the thread).

3. The **Future's Role**: 
    The **Future** object captures the state of the operation that is currently waiting, allowing the 
    program to resume it exactly where it left off once the data arrives.


In async Rust, we can write the same logic but without blocking the entire thread. 
Here's a simplified version using async/await:

Sequential async version : prints the start and finish of A → B → C in order takes 3 seconds
```rust 
use tokio::time::{sleep, Duration};

async fn read_data_async(path: &str) -> String {
    println!("Reading data from {}...", path);
    // Simulate async I/O work
    sleep(Duration::from_millis(1000)).await;
    println!("Finished reading {}", path);
    format!("Data from {}", path)
}

#[tokio::main]
async fn main() {
    let start_time = std::time::Instant::now();
    // Task 1: Wait 1 second
    read_data_async("config_A").await;
    // Task 2: Wait 1 second
    read_data_async("config_B").await;
    // Task 3: Wait 1 second
    read_data_async("config_C").await;
    let duration = start_time.elapsed();
    println!("--- Total time elapsed: {:.2?}", duration);
}
```
Parallel Asyc version: Takes ~1 second total

```rust 
use tokio::time::{sleep, Duration};

async fn read_data_async(path: &str) -> String {
    println!("Reading data from {}...", path);
    sleep(Duration::from_secs(1)).await;
    println!("Finished reading {}", path);
    format!("Data from {}", path)
}

#[tokio::main]
async fn main() {
    let start_time = std::time::Instant::now();

    let (a, b, c) = tokio::join!(
        read_data_async("config_A"),
        read_data_async("config_B"),
        read_data_async("config_C"),
    );

    let duration = start_time.elapsed();
    println!("--- Total time elapsed: {:.2?}", duration);
}
```

* The `async` keyword marks the function as asynchronous, which means it returns a `Future`.
* The `.await` keyword is used inside the async function to await the completion of asynchronous operations
  (like the request to the server).
* We use an **executor** (ex: `tokio::runtime::Runtime`) to run asynchronous tasks. (more on executor below)

Above example using tokio::spawn 
```rust 
use tokio::time::{sleep, Duration};

async fn read_data_async(path: &str) -> String {
    println!("Reading data from {}...", path);
    sleep(Duration::from_millis(1000)).await;
    println!("Finished reading {}", path);
    format!("Data from {}", path)
}

#[tokio::main]
async fn main() {
    //Serial ex
    let start_time = std::time::Instant::now();
    // Spawn each task independently.
    let handle_a = tokio::spawn(async { read_data_async("config_A").await });
    let handle_b = tokio::spawn(async { read_data_async("config_B").await });
    let handle_c = tokio::spawn(async { read_data_async("config_C").await });

    // Await all of them.
    let (a, b, c) = tokio::join!(handle_a, handle_b, handle_c,);

    println!("Result A: {:?}", a.unwrap());
    println!("Result B: {:?}", b.unwrap());
    println!("Result C: {:?}", c.unwrap());

    let duration = start_time.elapsed();
    println!("--- Total time elapsed serial ex mode: {:.2?}", duration);

    // parallel ex
    let start_time = std::time::Instant::now();

    // Task A
    let a = tokio::spawn(async { read_data_async("config_A").await })
        .await
        .unwrap();

    // Task B (waits for A)
    let b = tokio::spawn(async { read_data_async("config_B").await })
        .await
        .unwrap();

    // Task C (waits for B)
    let c = tokio::spawn(async { read_data_async("config_C").await })
        .await
        .unwrap();

    println!("Result A: {}", a);
    println!("Result B: {}", b);
    println!("Result C: {}", c);

    let duration = start_time.elapsed();
    println!("--- Total time elapsed parallel ex mode: {:.2?}", duration);
}
```

### - Key Difference:

In synchronous programming, everything happens in order, blocking until each operation finishes. 
In asynchronous programming, we can initiate tasks that are expected to take time, without blocking the 
rest of the program from continuing. However, we still need a mechanism to track the results of those tasks 
later, which is where the `Future` trait comes into play.

## 3. The `async`/`await` Syntax in Rust

Rust’s `async` and `await` keywords allow us to write asynchronous code in a natural, readable manner. 
This syntax provides a way to mark functions as asynchronous, and to pause their execution until they return
a result.

`async fn` doesn’t run immediately when you call it. 
Instead, it creates a **future**, which is essentially a description of the **task**. 
The future only runs when it is polled by an executor.

=>  `async fn`  just creates a `Future` that, when polled, will perform the steps inside the function.

1. `async fn` produces a Future:

```
async fn read_data_async(path: &str) -> String {
    println!("Reading {}...", path);
    smol::Timer::after(std::time::Duration::from_secs(1)).await;
    println!("Finished {}", path);
    format!("Data from {}", path)
}
``` 
```rust 
//calling it 
let fut = read_data_async("config_A");
```
Ddoes not execute anything yet.
It just creates a `Future` that, when polled, will perform the steps inside the function.

i.e `tokio::spawn(fut);`         // schedules it on Tokio’s runtime
    Here `spawn` submits the future as a **task** to the **executor**

### - What Happens Behind the Scenes?

When you mark a function as `async`, Rust transforms it into a **state machine**. 
Instead of executing the function linearly, Rust will compile it into code that can yield intermediate 
results and resume execution later. 

This process is entirely managed by the Rust runtime (ex: Tokio or async-std), which manages tasks on a 
separate thread pool or event loop.

Here’s a breakdown:

* `async fn`: Marks a function as asynchronous. The function now returns a `Future` instead of result.
* `.await`: Suspends execution of the current function until the result of the `Future` is ready. 
            It does not block the entire thread, unlike a synchronous `wait`.

### - Example: An Asynchronous Function

```rust
use tokio;

async fn do_some_work() {
    // Simulate doing some work (like I/O or computation)
    println!("Starting work...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    println!("Work completed!");
}

#[tokio::main]
async fn main() {
    // Start the asynchronous work
    do_some_work().await;
}
```

In this example:

* The `do_some_work` function is asynchronous. It simulates some work by sleeping for 2 seconds.
* The `await` keyword ensures that the program waits for the task to finish before moving on to the next 
  operation.

---

## 4. Executors and the Runtime

The key challenge in asynchronous programming is managing concurrency without traditional threads. 
In Rust, this is achieved using **executors**. Executors are responsible for running asynchronous tasks and
scheduling their execution.

=> In async Rust, an executor is the runtime component that:
    - stores tasks (futures)
    - polls them when they are ready
    - runs them to completion
    - schedules them across threads

Tokio’s executor is the thing that actually runs your future’s poll() function repeatedly.

When you write:

```rust 
#[tokio::main]
async fn main() {} 
```
This macro sets up:
    - the Tokio scheduler
    - its thread pool (if multithreaded)
    - its I/O driver (epoll/kqueue/wepoll)
    - its timer wheel
    - the task executor

=> This executor is always running in the background.
And `tokio::spawn` simply submits the future to this executor.

When we call :
```rust
tokio::spawn(async move {
    do_something().await;
});
```
This happens:
    - The `async move { ... }` block is turned into a Future.
    - The future is wrapped in a *task* (Tokio allocates a Task object).
    - The task is submitted to the **Tokio executor**.
    - The executor decides:
        * which thread will poll it
        * when to poll it
        * when to reschedule it
    - The returned `JoinHandle` lets you `.await` the task’s result.

So **spawn = submit**
and 
**Executor = run and schedule**

```text
    [Your async function]
            |
            | tokio::spawn
            v
    ┌───────────────────────┐
    │   Tokio Executor       │
    │  - Thread pool         │
    │  - Timer driver        │
    │  - IO driver           │
    └───────────────────────┘
            |
            | polls tasks
            v
    [ Future state machines ]
```
    * Note:
        - All `tokio::spawn` tasks use the same executor created by `#[tokio::main]`.
        - Unless you explicitly create local executors, but that’s advanced.

        - `tokio::spawn_blocking`
            * `spawn_blocking`:
                - runs on a special blocking thread pool
                - used for CPU-heavy or blocking operations
            * `spawn`:
                - runs on the **async executor threads**
                - tasks must be non-blocking and async-friendly


- Rust’s `async`/`await` requires an **executor** to run the tasks. 
  Popular libraries like **Tokio** and **async-std** provide these runtimes.

  * **Tokio**: 
    A runtime for writing reliable, scalable, and fast asynchronous applications. 
    It’s widely used for systems programming, networking, and web services.

  * **async-std**: 
    Another runtime similar to Tokio but with a more lightweight focus.

An executor manages tasks and schedules when they are polled for completion. 
It allows multiple tasks to run concurrently, without blocking the main thread.

### - Example: Tokio Runtime

```rust
use tokio::time::sleep;
use std::time::Duration;

async fn do_async_task() {
    println!("Task started.");
    sleep(Duration::from_secs(1)).await;
    println!("Task completed.");
}

#[tokio::main]
async fn main() {
    // The executor runs the asynchronous task
    do_async_task().await;
}
```

* `#[tokio::main]`: This attribute macro sets up the Tokio runtime & starts the `main` func asynchronously.
* `sleep`: Built-in async func from `Tokio` that simulates delay (like waiting for I/O or  network request).

## 5. The Need for Futures

At the heart of asynchronous programming is the concept of a **Future**.

A `Future` represents a value that is not immediately available, but will be available at some point. 
This allows your program to continue executing while waiting for the result of long-running tasks, such as 
network calls, file I/O, or complex computations.

In the next section, we will introduce the `Future` trait, explain how it works, and how you can use it to 
write asynchronous code in Rust.

## Summary of Preliminaries:

1. Blocking vs. Non-blocking: In synchronous programming, each task blocks the program. 
   Asynchronous programming allows tasks to run without blocking the main thread.

2. `async`/`await` Syntax: Rust provides an easy-to-use syntax to write asynchronous code, but under the 
   hood, it works by creating state machines that yield control to the executor.

3. Executors: Executors like Tokio or async-std manage the scheduling and execution of asynchronous tasks, 
   ensuring they run concurrently without blocking.

4. Futures: The `Future` trait is the fundamental building block of async programming in Rust. 
   It represents a computation that will eventually return a value.


# Futures:

Async programming allows you to write code that can handle potentially blocking operations without blocking
the entire thread. One of the code building blocks of asynchronous programming in Rust is the `Future`
trait. 

`async Future` is the core abstraction for asynchronous programming representing a computation that may not
be ready now but will produce result as some point in the future. 
- `Future` : represents an operation that can be executed without blocking the current thread. Allowing
  other tasks to run concurrency while waiting for the `Future` to complete. 

Key characteristics of Rust's async Future:

1. Asynchronous Computation: 
    A `Future` represents an operation that can be executed without blocking the current thread. 
    This allows other tasks to run concurrently while waiting for the Future to complete.

2. Lazy Execution: 
    `Futures` in Rust are "lazy," meaning they do not automatically start executing when created. 
    They require an executor to drive them to completion. 

3. Polling Mechanism: 
    The executor drives a `Future` by repeatedly calling its `poll` method.

    - If the `Future` is complete and ready to return a value, `poll` returns `Poll::Ready(result)`.

    - If the `Future` is not yet complete, `poll` returns `Poll::Pending` and arranges for the current task
      to be woken up via a `Waker` when it's ready to make further progress.

4. `async` and `await` Keywords: 

    Rust's `async` keyword defines an asynchronous function that returns a `Future`. 

    The `await` keyword is used within an async function to pause execution until a `Future` it's waiting 
    on completes, effectively handling the polling process implicitly. 

5. Executors and Runtimes: 

    To actually execute a `Future` and get its result, an asynchronous runtime (like Tokio) and its 
    associated executor are needed. 
    The executor is responsible for scheduling and polling `Futures`. 
    Methods like `block_on` provided by runtimes can be used to block the current thread until a `Future` 
    completes and returns its result.

## `Future` trait in Rust std library. ( std::future::Future )

- `std::future::Future`: `Future` trait represents a computation that will eventually yield a value. 
  This trait has a single method called `poll` that is responsible for determining if the computation is
  complete or still pending.

Core definition :
```rust 
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

// This is the actual Future trait from std library:
pub trait Future {
    // The type of value produced when the future completes.
    type Output;

    // This method is called to drive the future's execution.
    // It returns:
    // - Poll::Ready(T): The future is complete and the result is T.
    // - Poll::Pending: The future is not yet complete. The Waker is registered
    //                  to be notified when progress can be made.
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```
- Most important method is `poll`, its the engine that drives a `Future` forward.
    * When a Future is **polled**, it attempts to make progress.
    * If the operation is complete, it returns `Poll::Ready(result)`
    * If the operation is still ongoing, it returns `Poll::Pending` and registers a mechanism ( the `Waker`)
      so the executor knows to poll it again later when something changes

### - Example 1 :
```rust 
use std::future::Future;
use std::task::{Context, Poll};

struct MyFuture;

impl Future for MyFuture {
    type Output = i32;

    fn poll(self: std::pin::Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        // Imagine some work is being done here (computation, I/O, etc.)
        // Once done, we return Poll::Ready with the result
        Poll::Ready(42)
    }
}
```
- Future is like a math function that's defined but not evaluated until you explicitly ask for the result.

- Lazy Evaluation: 
    A Future doesn’t start computing until it is polled by an executor (a runtime).
    That is you must explicitly **poll** the `Future` to make it run. 

- Poll Mechanism: 

    `poll` is the method that is called to check the state of the future (whether it is completed or not). 
    It returns:
    - `Poll::Pending` if the computation is still in progress.
    - `Poll::Ready(T)` if the computation is complete, where T is the type of the result.
    - `Output Type` The Output associated type defines what the Future will eventually return.


Ex: A simple Future that is designed to read a value from a HW sensor: ( temparature )

```rust 
// 1. Defining the Future struct
// This struct holds the state needed for the computation: 
// a mock counter to simulate waiting.
use std::time::{Duration, Instant};

/// A future that simulates reading temperature from a slow physics sensor
struct SensorReading {
    start_time: Instant,
    sensor_id: u32,
    measurement_duration: Duration,
}

impl SensorReading {
    fn new(sensor_id: u32) -> Self {
        Self {
            start_time: Instant::now(),
            sensor_id,
            measurement_duration: Duration::from_millis(100), // Sensor takes 100ms to read
        }
    }
}

impl Future for SensorReading {
    type Output = f64; // Temperature in Kelvin
    
    fn poll(self: std::pin::Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let elapsed = self.start_time.elapsed();
        
        if elapsed >= self.measurement_duration {
            // Sensor reading is complete
            let temperature = 273.15 + 20.0 + (self.sensor_id as f64 * 0.1); // Base temp + noise
            println!("Sensor {} reading complete: {}K", self.sensor_id, temperature);
            Poll::Ready(temperature)
        } else {
            // Sensor still measuring - need to be polled again later
            println!("Sensor {} still measuring... {}ms elapsed", 
                    self.sensor_id, elapsed.as_millis());
            // In real async, we'd arrange to be woken up
            Poll::Pending
        }
    }
}

// Note: This future does ABSOLUTELY NOTHING until we poll it!
```
### - Example 2:
```rust
// 1. Defining the Future struct
// This struct holds the state needed for the computation: 
// a mock counter to simulate waiting.
struct SensorReadFuture {
    polls_remaining: u32,
}

impl Future for SensorReadFuture {
    type Output = u32; // The sensor reading (e.g., temperature)

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // This is where the *actual* work or check happens.
        if self.polls_remaining == 0 {
            println!("✅ Sensor: Reading is complete!");
            // The future is done. Return the final result.
            Poll::Ready(25) // The temperature is 25 degrees.
        } else {
            // Simulate waiting for the hardware to be ready.
            println!("⏳ Sensor: Waiting for data... ({} attempts left)", self.polls_remaining);
            self.polls_remaining -= 1;
            // The future is not done. Tell the executor to try again later.
            Poll::Pending
        }
    }
}

// 2. Creating the Future is **Lazy**
fn create_sensor_read() -> SensorReadFuture {
    println!("--- Future Created (STILL LAZY) ---");
    SensorReadFuture { polls_remaining: 3 }
}

// In real-world async Rust, an 'executor' (like tokio)
// handles this polling loop, but here we do it manually for demonstration.
fn main() {
    let mut future = create_sensor_read();
    
    // The `main` function is synchronous, so we must manually drive the Future.
    // This part of the code simulates what an executor does.
    
    // We need a dummy Waker and Context for manual polling.
    // (This part is complex, but it shows polling is needed)
    let waker = futures::task::noop_waker();
    let mut context = Context::from_waker(&waker);

    println!("--- STARTING EXECUTION (POLLING) ---");
    
    // Loop until the Future is ready
    loop {
        // Use `Pin::new(&mut future)` to create a Pinned reference needed by the trait
        let pinned_future = unsafe { Pin::new_unchecked(&mut future) };
        
        match pinned_future.poll(&mut context) {
            Poll::Ready(result) => {
                println!("Final Result Received: {}°C", result);
                break;
            }
            Poll::Pending => {
                // In a real executor, the thread would now go work on other tasks
                // and wait for the Waker to wake it up again.
                // Here, we just sleep for a moment to simulate a delay.
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}
```
- **creation is instant**: When `create_sensor_read()` runs it immediately prints "Future created (STILL
  LAZY)". **No actual sensor reading is attempted yet.
- **Polling is Execution** : The work ( the decrementing counter and the println! inside pool) only starts
  once the `loop` begins and calls `pinned_future.poll(&mut context)`
- **Completion**: Future only finished (returns `Poll::Ready`) after being polled enough times to reach 
  `polls_remaining == 0`

### - Example 3: simulate a computationally expensive operation that eventually returns a result.
```rust 
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;
use std::thread;
use std::time::Duration;

struct ExpensiveComputation;

impl Future for ExpensiveComputation {
    type Output = f64;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // Simulate some computation that will take time (e.g., solving a physics problem)
        println!("Starting expensive computation...");

        // Pretend the computation will take 2 seconds
        thread::sleep(Duration::from_secs(2));

        println!("Computation finished.");
        Poll::Ready(3.14159)  // Return the result of our 'computation', e.g., π
    }
}

fn main() {
    let computation = ExpensiveComputation;

    // In a real async environment, this would be run on an executor
    let result = futures::executor::block_on(computation);

    println!("Result of computation: {}", result);
}
```
- `ExpensiveComputation`: `struct` that simulates a computationally expensive task 
  (in this case, simply sleeping for 2 seconds).

- `poll`: The method simulates waiting for some long-running computation (e.g., calculating something 
  important in physics or engineering).

- `block_on`: This is a helper function from the `futures` crate that runs the future to completion, 
  effectively blocking the current thread until the future is ready.

### - Example 4: Simulate motion of a obj under gravity:

```rust 
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;
use std::thread;
use std::time::Duration;

struct GravitySimulation {
    time: f64,   // Time in seconds
    gravity: f64, // Gravitational constant (m/s²)
}

impl GravitySimulation {
    fn new(time: f64) -> Self {
        GravitySimulation {
            time,
            gravity: 9.81, // Earth's gravity
        }
    }
}

impl Future for GravitySimulation {
    type Output = f64;  // The future will output the final position

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        // Simulate the physics calculation with an artificial delay
        println!("Starting gravity simulation...");

        // Simulate the computation by "sleeping" for a moment
        thread::sleep(Duration::from_secs(1));

        // Calculate the final position using the formula s = 0.5 * g * t^2
        let position = 0.5 * self.gravity * self.time.powi(2);

        println!("Gravity simulation finished.");
        Poll::Ready(position)  // Return the computed position after time has passed
    }
}

fn main() {
    let simulation = GravitySimulation::new(3.0);  // 3 seconds of free fall

    // Run the future to completion and get the result
    let final_position = futures::executor::block_on(simulation);

    println!("Final position after 3 seconds: {:.2} meters", final_position);
}
```
- `GravitySimulation`: `struct` that represents the simulation of an object falling under gravity, where the
  position is calculated using the kinematic equation $s = 1/2g*t^2$

- `poll`: The method calculates the final position after a certain time (self.time), simulating the delay 
  for the computation.

- `block_on`: The computation is executed synchronously in this simple example, but in a real async runtime,
  the computation could be non-blocking.

## Key Takeaways:

1. `Future` is an abstraction for deferred computation. 
   It allows us to represent operations that will eventually return a result but haven't completed yet.

2. `Lazy Evaluation`: The computation doesn't start until the `Future` is polled.

3. `Poll Mechanism`: Each poll can make a partial progress, it checks the state of the future, either
   `Pending` or `Ready`.

4. Cooperative multitasking: Futures yield control back when they can't make progress.

### Futures Use in Systems Programming:

When working with **networking**, **file I/O**, or even in physics simulations, we often need to perform 
operations that take time but do not want to block the entire application. For example:

- File I/O: Reading large files from disk can take significant time, and we don’t want to block other tasks
  (like handling user input or network traffic) while waiting.

- Network Operations: Connecting to a server or fetching remote data is inherently slow and can benefit from
  asynchronous programming. 

- Computation: In scientific computing, simulations often involve heavy calculations that can be done
  asynchronously to improve overall system performance.

# Pooling the `Future` and `Waker`


## Pooling
When `Future` gets executed the executor check the status of `Future`: It returns two states:

- `Poll::Pending` : `Future` is still being computed and isn't ready yet. 
- `Poll::Ready<T>` : `Future` has completed and result is available.

```rust
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

struct MyFuture;

impl Future for MyFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        // Simulating a long-running task (e.g., I/O, computation, etc.)
        // We'll return Poll::Ready to indicate completion
        Poll::Ready(42)
    }
}
```
- `poll` takes `self: Pin<&mut Self>` 
    ``` 
    fn poll(self:Pin<&mut Self>,cx:&mut Context<_>)→Poll<Self::Output>
    ```
    * Self-Referential Structs: To implement async control flow (like state machines), `Futures` often need 
    to contain pointers to their own data. If the `Future` were to be moved in memory while waiting, these 
    internal pointers would become invalid, leading to memory unsafety.

    * The Role of `Pin`: `Pin<&mut T>` is a smart pointer that guarantees the data it points to (T) will 
      not be moved in memory for the duration of its pinning. 
      This is a crucial, advanced concept that enables Rust's safe and efficient async state management.

    In short, `Pin` ensures the Future remains stable in memory while it's being polled, preventing memory 
    corruption.

- `poll` method checks if computation has completed  
- if done we return `Poll::Ready(T)` with the result. 
- If not yet finished, we would return `Poll::Pending`.

When task is pending we need a mechanism to tell the executor when the task can be polled again. 

## Walker
`Walker` is a mechanism that tells the executor that the `Future` is ready to be polled again.

It is created during the polling process and passed to the `Future` as part of the Context argument.
=> The Waker is a small, encapsulated handle passed to the Future via the Context (the `cx` argument in `poll`).

When a `Future` is **pending**, it can register a `Waker` that tells the executor, "Hey, pls check again."

When the `Future` becomes ready (e.g., an I/O operation completes), the `Waker` is called to wake the 
executor, allowing it to re-poll the `Future`.

This process ensures that the program doesn't block the thread waiting for the Future to complete. 
Instead, the executor will periodically check the status of the Future, efficiently managing the tasks.

### Example 1:
```rust 
use std::future::Future;
use std::task::{Context, Poll, Waker};
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

struct MyAsyncTask {
    state: TaskState,
}

enum TaskState {
    Pending,
    Completed,
}

impl MyAsyncTask {
    fn new() -> Self {
        MyAsyncTask {
            state: TaskState::Pending,
        }
    }
}

impl Future for MyAsyncTask {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        // If the task is already completed, return the result
        if matches!(self.state, TaskState::Completed) {
            return Poll::Ready(42);
        }

        // Simulate some work that will eventually complete
        if let TaskState::Pending = self.state {
            println!("Task is still pending. Registering Waker.");
            // Register the Waker to notify us when the task is ready to be polled again
            let waker = cx.waker().clone();

            // Simulate some work being done in another thread (e.g., I/O or long computation)
            thread::spawn(move || {
                // Simulate a delay (like waiting for I/O)
                thread::sleep(Duration::from_secs(2));
                println!("Work completed. Waking the task.");

                // Call the Waker to wake up the executor and re-poll the task
                waker.wake();
            });

            // Return Poll::Pending because the task is still in progress
            self.state = TaskState::Pending;
            Poll::Pending
        } else {
            Poll::Ready(42) // Task is done, so return the result
        }
    }
}

fn main() {
    use futures::executor::block_on;

    let task = MyAsyncTask::new();
    let result = block_on(task);

    println!("Task result: {}", result);
}
```
- **State Management**: The `MyAsyncTask struct` tracks the state of the task (whether it is Pending or Completed).

- **Polling with Waker**: When the poll method is called, if the task is still pending, we register the
  `Waker` with `cx.waker().clone()`. This allows us to be notified later when the task is ready to be polled
  again.

- **Simulating Work in a Separate Thread**: We simulate a time-consuming task (like an I/O operation) by 
   spawning a new thread that waits for 2 seconds and then calls waker.wake().

- **Waking the Executor**: After the simulated task completes, `waker.wake()` tells the executor to poll 
  the future again. At that point, the future has completed, and we return `Poll::Ready(42)`.

The Waker is essential for ensuring that the executor can efficiently manage tasks that aren’t immediately 
ready. Without it, the executor wouldn’t know when to check a pending future again, potentially leading to 
wasted resources. 

The Waker allows the program to efficiently manage concurrent tasks without blocking threads unnecessarily.

Efficiency: 
    `Waker` ensures that the program isn't continually checking a Future that is still pending, but instead 
    waits for an external event (like I/O completion) before trying again.
Non-blocking: 
    By using a `Waker`, we can implement non-blocking asynchronous tasks that only wake up when necessary, 
    which is crucial for high-performance systems programming.

### Example 2:
```rust 
use std::future::Future;
use std::task::{Context, Poll, Waker};
use std::pin::Pin;

// A mock struct representing the state of reading a large file.
struct FileReadFuture {
    // Total size of the file to read (in blocks).
    total_blocks: u32,
    // How many blocks have been read so far.
    blocks_read: u32,
    // A place to store the Waker so the 'disk driver' can wake the executor.
    waker_to_store: Option<Waker>,
}

impl Future for FileReadFuture {
    type Output = String; // The final content of the file.

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // --- 1. Check for Completion ---
        if self.blocks_read >= self.total_blocks {
            println!("**Poll Status:** File read complete!");
            return Poll::Ready(format!("Successfully read {} blocks.", self.total_blocks));
        }

        // --- 2. Make Progress (Non-Blocking I/O) ---
        // Simulate reading one block per poll attempt.
        self.blocks_read += 1;
        println!(
            "**Poll Status:** Read block {}/{}. Requesting to be polled again.",
            self.blocks_read, self.total_blocks
        );

        // --- 3. Handle Pending State and Register Waker ---
        
        // This is the critical step: we must store the Waker provided by the executor.
        // The 'disk driver' (simulated here) will use this Waker to notify the executor.
        // We clone it because the Waker will be used *later* by an external entity.
        if self.waker_to_store.is_none() {
            self.waker_to_store = Some(cx.waker().clone());
            // In a real scenario, this Waker would be passed to the OS/driver.
            // For this example, we'll manually call it later.
        }

        Poll::Pending
    }
}

// --- SIMULATED EXECUTOR AND WAKER PROCESS ---

fn main_system_example() {
    // Set up the Future: needs 5 blocks to be read.
    let mut future = FileReadFuture {
        total_blocks: 5,
        blocks_read: 0,
        waker_to_store: None,
    };
    
    // We need a dummy Waker and Context for manual polling.
    let waker = futures::task::noop_waker();
    let mut context = Context::from_waker(&waker);
    let mut pinned_future = unsafe { Pin::new_unchecked(&mut future) };

    println!("--- Starting Asynchronous File Read (Poll Cycle) ---");

    // Loop until the Future is ready
    let mut poll_count = 0;
    loop {
        poll_count += 1;
        println!("\n*** Executor Poll Cycle #{} ***", poll_count);
        
        match pinned_future.as_mut().poll(&mut context) {
            Poll::Ready(result) => {
                println!("\n⭐ Final Result: {}", result);
                break;
            }
            Poll::Pending => {
                // In a real application, the executor would switch to another task now.
                // We simulate the Waker being called after some time:
                if let Some(waker) = pinned_future.waker_to_store.as_ref() {
                    if poll_count >= 5 {
                        println!("** WAKER ALERT! ** The 'disk driver' is done and is calling `waker.wake()`.");
                        waker.wake_by_ref(); 
                        // Note: In this manual simulation, the `wake_by_ref()` call
                        // doesn't magically break the loop; it just simulates the 
                        // *signal* that tells a real executor to poll again.
                    }
                }
                std::thread::sleep(std::time::Duration::from_millis(100));
            }
        }
    }
}

// To run this example in a live environment, you would need the `futures` crate
// for `noop_waker`.

// main_system_example(); // If this were executable code
```
- Non-Blocking Progress: Each poll call makes a small, non-blocking unit of progress (blocks_read += 1).

- Waker Registration: The Future stores the Waker so the underlying resource can notify the executor when 
  it's done.

- Result: The Future only transitions to Poll::Ready after enough poll calls have occurred to complete the 
  required work.

## Summary of Polling and Waker Mechanism

- **Polling**: 
    The `executor` calls the `poll` method to check if the `Future` is ready to return a result. 
    If not, it returns `Poll::Pending`, signaling that the future is still being worked on.

- `Waker`: 
    The Waker is a mechanism that tells the executor when a blocked future is ready to be polled again 
    (Ex after a network request, or file I/O completes). It prevents the executor from repeatedly polling 
    the future unnecessarily.

- `Task Management`: 
    The executor uses the `Waker` to wake up when it's time to check the future again, helping with 
    non-blocking I/O and concurrency.

## Key Concepts in Action

* The `Polling Cycle`:
    1. Executor calls `poll()` on a future
    2. Future returns:
        - `Poll::Ready(result)` - done, return result
        - `Poll::Pending` - not done, but might be later
    3.Future stores Waker - so it can notify executor when ready
    4. Waker triggers re-poll - when future can make progress

* `Waker` Best Practices:
    - **Clone and store** the `waker` when returning `Poll::Pending`
    - Call `wake()` when the future can make progress again
    - Don't call `wake()` unnecessarily - it causes wasteful polling
    - Use `wake_by_ref()` when you might need the waker again


# `async` and `.await` 

That's the perfect transition! After understanding the low-level mechanics of `Futures`, `polling`, and the 
`Waker`, reader need to see how Rust makes this complexity manageable with the high-level `async` and 
`.await` syntax. 

The `Future` trait, `poll` method, and `Waker` are powerful, but manually writing them is tedious. 
The `async` and `.await` keywords are syntactic sugar provided by the Rust compiler to handle this 
machinery for you safely and efficiently.

`async` and `.await` are shorthand for managing the above complexity.

## `async` as Syntax sugar for State Machines:

The `async` keyword in Rust is a shorthand that allows us to write asynchronous functions without having to 
manually implement the `Future` trait. 

In reality, Rust transforms the body of an `async` function into a **state machine** that is capable of 
being polled multiple times.

### How `async` works:

When you mark a function as `async`, Rust creates an anonymous struct that implements the `Future` trait.

The body of the func is then split into parts, and these parts represent the states of the state machine.

Each time the func is polled (via `.await`), it either moves to a new state or returns the result when it's 
ready.

## Example: Transforming an async function into a state machines

```rust 
// 1. Defining the async function
// The 'async' keyword means this function returns a Future, NOT the result directly.
async fn fetch_sensor_data(sensor_id: u32) -> String {
    println!("  [Task {}] 🔍 Preparing to send request...", sensor_id);
    
    // The code *before* the first .await is synchronous and runs immediately 
    // when the Future is created. (If no .await is used, the Future returns Poll::Ready on the first poll).
    
    // Simulate waiting for a network response.
    // This is where the Future will yield control (return Poll::Pending).
    let data = get_data_from_network(sensor_id).await; 
    
    // This code only runs *after* the Future is woken up and polled again 
    // (i.e., after the network data is received).
    println!("  [Task {}] ✅ Data received: {}", sensor_id, data);
    
    data
}
// 2.
async fn simple_task() -> i32 {
    println!("Start of task");
    42
}
```
When you call `fetch_sensor_data(42)`, it returns a Future immediately.

Code looks like a normal func, but it is `asynchronous`. 

Rust turns this into a **state machine** under the hood. 

In fact, the Rust compiler transforms it into something like this:
```rust 
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;

struct SimpleTaskFuture;

impl Future for SimpleTaskFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context) -> Poll<Self::Output> {
        // The task is immediately completed in this simple example
        Poll::Ready(42)
    }
}

async fn simple_task() -> i32 {
    // This async function will generate a Future (i.e., SimpleTaskFuture)
    42
}
```
- **State Machine**: 
    The body of the `async` function is divided into states (for example, awaiting other `async` operations). 
    When polling, the state machine checks if it can move to the next state or if it needs to wait for 
    another operation.

- **Future Implementation**: 
    The `async` func implements the `Future` trait behind the scenes, meaning that when you call an `async` 
    func, it returns a Future — something that may not be immediately available but will be in the future.

## `await` as Syntax Sugar for Polling

The `.await` keyword is the partner to `async`. It can only be used inside an `async` func or `async` block.
The `await` keyword is used to block the execution of an `async` function until the inner future is ready. 

Conceptually, `await` is shorthand for polling the inner Future until it resolves with `Poll::Ready`.

When you use `.await` on a `Future`, the executor will continuously `poll` that `Future` (without blocking 
the current thread) until the Future returns a result.

### How await works:

- Polling: 
    `.await` effectively calls the poll method on the `Future`.
    If the result is `Poll::Pending`, the execution of the async func is suspended until `Future` is ready.

- Resumption: 
    Once the `Future` is ready (i.e., it returns `Poll::Ready(T)`), the await expression gives control 
    back to the function with the final result.

### Example that illustrates the polling mechanism with `await` :

```rust 
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;
use std::thread;
use std::time::Duration;

struct DelayedTask;

impl Future for DelayedTask {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        println!("Polling...");

        // Simulate some delayed work
        thread::sleep(Duration::from_secs(2));

        // Once the work is done, we return Poll::Ready
        Poll::Ready(42)
    }
}

async fn perform_task() -> i32 {
    let result = DelayedTask.await; // This is where the polling happens
    result
}

fn main() {
    use futures::executor::block_on;

    // Running the async task using block_on
    let result = block_on(perform_task());
    println!("Task completed with result: {}", result);
}
```
- The `DelayedTask` `Future`: 
    This struct simulates a task that takes some time (e.g., I/O or computation). 
    The poll method checks if the task is completed.

- Initially, the task is not ready, so `poll` returns `Poll::Pending`.

- After simulating a delay (`thread::sleep`), the task becomes ready, and poll returns `Poll::Ready(42)`.

- Using `.await`: In `perform_task()`, the `.await` keyword is used to pause the execution of the function
  until `DelayedTask` is ready. 
  The `block_on` function (from the `futures` crate) runs the `async` function to completion, simulating the 
  behavior of an `async` runtime.

## Example: Calculating Trajectory
```rust 
// Mock Futures for the two stages
async fn calculate_air_resistance(initial_vel: f64) -> f64 {
    // This function body returns a Future that simulates a long CPU task.
    // A real system would use a specific Future here, but we use a mock.
    println!("    [Stage 1] Calculating complex air resistance...");
    // Mock await: Pauses here and yields control.
    tokio::time::sleep(std::time::Duration::from_millis(500)).await; 
    initial_vel * 0.9 // Reduced velocity after resistance
}

async fn calculate_gravity_and_drag(vel: f64) -> (f64, f64) {
    println!("    [Stage 2] Calculating gravity and drag effects...");
    // Mock await: Pauses here and yields control.
    tokio::time::sleep(std::time::Duration::from_millis(300)).await; 
    (vel * 0.8, 10.5) // (Final Velocity, Max Height)
}

// The main asynchronous function driving the computation
async fn simulate_trajectory(initial_velocity: f64) -> (f64, f64) {
    println!("Starting Trajectory Simulation (Initial V: {})", initial_velocity);

    // --- First .await ---
    // The current Future pauses and polls 'calculate_air_resistance()' repeatedly.
    // The thread is yielded while waiting for the 500ms sleep Future to complete.
    let velocity_after_resistance = calculate_air_resistance(initial_velocity).await;

    println!("Air Resistance complete. Remaining Velocity: {:.2}", velocity_after_resistance);

    // --- Second .await ---
    // The current Future pauses and polls 'calculate_gravity_and_drag()' repeatedly.
    // The thread is yielded while waiting for the 300ms sleep Future to complete.
    let final_results = calculate_gravity_and_drag(velocity_after_resistance).await;

    println!("Trajectory finished.");
    final_results
}

// In a real application, an executor is needed to run the main Future.
#[tokio::main] // This macro sets up the necessary executor (Tokio)
async fn main() {
    let initial_v = 100.0;
    
    // We create the Future for the simulation, but we still need to await it 
    // so the executor polls it.
    let (final_v, max_h) = simulate_trajectory(initial_v).await; 

    println!("Summary: Final Velocity = {:.2}, Max Height = {:.2}", final_v, max_h);
}
```
What Happens Internally:

- When you call `.await`, it essentially calls the `poll` method on the `Future` and checks if the result 
  is ready.

- If the `Future` returns `Poll::Pending`, the execution is paused (i.e., the function is suspended) until 
  the `Future` becomes ready.

- Once the `Future` is ready (`Poll::Ready`), the function resumes from where it left off and returns the 
  result.

# `async` and `await` Together:

When combined, `async` and `await` make asynchronous programming in Rust much more readable and intuitive.

Here’s how the entire process works in a typical flow:

1. The `async` function is a state machine: 
    It’s not a normal function. When you mark a function as `async`, the Rust compiler creates a 
    state machine behind the scenes that implements the Future trait.

2. The `await` keyword polls the `Future`: 
    When you call `.await`, it starts polling the `Future` until it becomes ready. 
    If the `Future` is still pending, the function execution is suspended, and once the `Future` resolves,
    the function picks up where it left off.

## Example: Chaining Multiple async Operations

```rust 
use std::future::Future;
use std::task::{Context, Poll};
use std::pin::Pin;
use std::thread;
use std::time::Duration;

struct TaskA;
struct TaskB;

impl Future for TaskA {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        println!("Task A in progress...");
        thread::sleep(Duration::from_secs(1));
        Poll::Ready(10)  // Task A is done
    }
}

impl Future for TaskB {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
        println!("Task B in progress...");
        thread::sleep(Duration::from_secs(1));
        Poll::Ready(20)  // Task B is done
    }
}

async fn perform_tasks() -> i32 {
    let result_a = TaskA.await;
    let result_b = TaskB.await;

    result_a + result_b  // Combine results from both tasks
}

fn main() {
    use futures::executor::block_on;

    // Running the async function
    let result = block_on(perform_tasks());
    println!("Combined result: {}", result);
}
```
- `TaskA` and `TaskB`: These are two `Future` implementations that simulate asynchronous tasks.

- `async` Function `perform_tasks`: It performs two async tasks sequentially, using `await` to pause until 
  each task completes.

- Sequential Execution: The tasks are executed one after the other. After `TaskA` completes, `TaskB` starts.
  The final result is the sum of the two tasks.

## Summary:
- `async` creates a state machine behind the scenes that implements the `Future` trait, allowing the 
  function to be paused and resumed.

- `await` is shorthand for repeatedly polling the `Future` returned by an `async` function until it resolves
  (i.e., until `Poll::Ready` is returned).

- `async` and `await` simplify writing asynchronous code by providing a syntax that looks synchronous but 
  runs asynchronously behind the scenes. This makes Rust's async model more approachable without sacrificing
  performance.

---
# Executor: (Asynchronous Runtime)

`async` creates a `Future` (a definition of work) and 
`.await` drives the polling cycle. And returns future.
Future is lazy; it doesn't start or advance itself. 
The **Executor** is the entity responsible for actively polling `Futures` and managing the overall 
asynchronous program flow.

- **Executor** :
    An Executor, or Asynchronous Runtime, is the specialized library component (like Tokio or smol) that 
    manages the execution of all your `Futures`. It is essentially the OS kernel for your asynchronous tasks.

    Its primary job is to ensure that a set of potentially thousands of independent `Futures` can be driven 
    to completion using only a small, fixed number of operating system threads (often just 1 or 2 / CPUcore).

- Systems Analogy: The Task Scheduler
    Think of the Executor as a highly efficient **thread pool** combined with a **task scheduler** and a 
    **Waker dispatcher**.

    1. **Task Queue**: It maintains a queue of **ready** Futures (tasks) that need to be polled.

    2. **Polling Loop**: Its threads continuously pull a Future from the queue and call its `poll()` method.

    3. **Waker Integration**: It holds the mechanism that receives the signal when a resource (like a 
       completed network read) calls a Future's associated `Waker`. Upon receiving a wake-up signal, the 
       Executor moves that `Future` from a "waiting" state back into the "ready" queue.

## The Relationship Between Executor, Future, and waker

The Executor ties all the previous concepts together:

| Component | Responsibility | Action  |
| :--- | :---: | ---: |
|Future (Code)|Represents the pausable work.| Defines the poll logic and holds local state |
|.await (Syntax)|Defines where the Future can yield control.|Returns Poll::Pending to the Executor|
|Waker (Mechanism)|Defines how to wake up the waiting Future.|Notifies the Executor to put the Future back into the ready queue.|
|Executor (Runtime)|Defines when and how to run the Future.|Calls poll() and manages the ready/waiting queues|

## Running a Future: The Final step
Since Futures are lazy, your main synchronous function (fn main()) cannot simply call an `async fn` and 
expect it to run. You must explicitly start an Executor to `poll` the root Future.

In asynchronous Rust, this is often done using a macro provided by the runtime:

```rust 
// Requires the Tokio runtime in Cargo.toml
use tokio; 
use std::time::Instant;

// Future 1: A calculation that takes 800ms
async fn long_cpu_task(id: u32) {
    println!("  [Task {}] Started (800ms expected wait)", id);
    // Simulate a blocking wait (I/O). The thread yields here.
    tokio::time::sleep(std::time::Duration::from_millis(800)).await;
    println!("  [Task {}] Finished.", id);
}

// Future 2: A calculation that takes 300ms
async fn short_io_task(id: u32) {
    println!("  [Task {}] Started (300ms expected wait)", id);
    // Simulate a blocking wait (I/O). The thread yields here.
    tokio::time::sleep(std::time::Duration::from_millis(300)).await;
    println!("  [Task {}] Finished.", id);
}

#[tokio::main]
async fn main() {
    let start = Instant::now();

    // The Executor (Tokio) starts here. The main function is now asynchronous.
    println!("** Executor Starts **");

    // We don't want to wait for Task 1, then Task 2. We want to start them 
    // and let the Executor run them concurrently.
    
    // We create the Futures first. They are still lazy.
    let future1 = long_cpu_task(1);
    let future2 = short_io_task(2);

    // The 'tokio::join!' macro runs the two Futures on the Executor concurrently.
    // The Executor polls Future 1, it hits .await and yields.
    // The Executor polls Future 2, it hits .await and yields.
    // The Executor runs other tasks (or waits) until the Wakers notify it.
    tokio::join!(future1, future2);

    // The join! macro returns only after BOTH Futures have returned Poll::Ready.

    let duration = start.elapsed();
    
    // Total time should be close to 800ms, NOT 1100ms (800 + 300).
    println!("** Executor Ends **");
    println!("Total execution time with concurrency: {:.2?}", duration);
}
```
## The Power of Concurrency:

If we had run these 2 tasks synchronously (sequentially), the total time would be approx 800ms+300ms=1100ms.

Because the Executor uses non-blocking I/O and the `Waker` mechanism:
-  It starts Task 1 and yields at .await.
- It starts Task 2 and yields at .await.
- It waits.
- After 300ms, the Waker for Task 2 fires, and the Executor completes Task 2.
- The Executor continues waiting on Task 1.
- After a total of 800ms, the Waker for Task 1 fires, and the Executor completes Task 1.

The total time is dominated by the longest task, approximately 800ms. 
This is the core benefit of Rust's asynchronous model: maximum concurrency with minimum thread overhead.


