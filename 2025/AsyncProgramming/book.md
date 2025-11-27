# Concurrency and Asynchronous Programming

Its the ability of a program to execute multiple tasks in overlapping time period, allowing them to make
progress without completing one before starting another. 

This is generally achieved by interleaving the execution of tasks, such as a single-core CPU rapidly
switching between them, creating the illusion of simultaneous progress.

Concurrency is fundamental for modern applications, enabling them to remain responsive and efficient, mainly
when handling tasks that involve waiting like network requests or file I/O.

Concurrency improves Resource efficiency, by allowing other tasks to proceed while one task is waiting, such
as input/output operation.

Its provides the foundation for parallelism ( true simultaneous execution on multiple cores ) by structuring
the program to handle multiple tasks, which can then be distributed across multiple processors for
simultaneous execution. 

## How It works:

- Context switching: 
    On a single CPU core, the operating system can save the state of one task and load the state of another,
    switching between them many times per second.

- Interleaving: 
    Tasks are not necessarily run simultaneously but are interleaved, so they can start, run for a bit, and 
    then pause while another task runs.

- Message passing: 
    Concurrent components can interact by sending messages to each other through communication channels, 
    similar to how a web browser and a web server communicate. 

## Concurrency with Rust:

Generally there are 2 approaches to concurrency:
- Traditional Threading Model
- Asynchronous Programming Model

### Thread Management:
    
The traditional threading model centers around `std::thread::spawn` for creating operating system threads 
and `JoinHandle` for coordination:

```rust
                    +----------------+
                    |  main_thread   |
                    +-------+--------+
                            |
                            v
                    +----------------+
                    | thread::spawn  |
                    +---+--------+---+
                        |        |
                        |        |
                        v        v
            +----------------+   +------------------+
            |  JoinHandle    |   | spawned_thread   |
            +--------+-------+   +--------+---------+
                     |                    |
                     |                    v
                     |          +----------------------+
                     |          |  thread_completion   |
                     |          +----------+-----------+
                     |                     |
                     v                     v
                     +----------+----------+
                                |
                                v
                         +-------------+
                         |    join()   |
                         +------+------+
                                |
                                v
                      +------------------+
                      |  thread_result   |
                      +------------------+

```
Key APIs include `tread::spwan`, `JoinHandle` and `thread::sleep` for thread management and synchronization.

#### Rust Threading: Key APIs Explained


1. **`thread::spawn`**

`thread::spawn` is the primary function used to create a new thread in Rust.

**What it does**

* Starts a new OS thread.
* Executes the closure you pass to it.
* Returns a `JoinHandle`, which lets you manage/await the thread’s result.

**Signature**

```rust
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static
```

**Key points**

* The closure must be **`Send`** and **`'static`**, meaning:

  * It can be safely transferred to another thread.
  * It does not borrow data that might disappear while the thread runs.

* Use it when you want background or parallel work.

**Example**

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("Hello from a new thread!");
    42
});
```

2. **`JoinHandle`**

`JoinHandle<T>` is the handle returned by `thread::spawn`.
It represents a *running or completed* thread that will eventually produce a value of type `T`.

**What it does**

* Allows the parent thread to wait for the spawned thread.
* Provides access to the result the thread returns.
* Ensures that the thread finishes before you access its output.

**Key method: `join()`**

```rust
let result = handle.join();
```

**What `join()` does**

* **Blocks** the current thread until the spawned thread exits.
* Returns `Result<T, Box<dyn Any + Send + 'static>>`

  * `Ok(T)` → thread completed normally
  * `Err(..)` → thread panicked

**Example**

```rust
use std::thread;

let handle = thread::spawn(|| 10 + 2);

let result = handle.join().unwrap();
println!("Thread result = {}", result);
```

3. **`thread::sleep`**

`thread::sleep` pauses the current thread for a specified duration.

**What it does**

* Suspends the thread without consuming CPU.
* Useful for:

  * simple synchronization
  * rate-limiting
  * delaying work
  * simulating long-running tasks

**Signature**

```rust
pub fn sleep(dur: Duration)
```

**Example**

```rust
use std::{thread, time::Duration};

thread::sleep(Duration::from_millis(500));
```

##### How They Work Together

Example combining all three:

```rust
use std::{thread, time::Duration};

fn main() {
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_secs(1));
        println!("Work done in spawned thread");
        123
    });

    println!("Main thread waiting...");

    let result = handle.join().unwrap();

    println!("Spawned thread returned: {}", result);
}
```

Flow:

1. `thread::spawn` creates a new thread.
2. The new thread sleeps (simulating work).
3. Main thread waits using `join()`.
4. When the spawned thread is done, `join()` returns the thread’s output.

#### Summary

| API                 | Purpose                 | Key Behavior                                                |
| ------------------- | ----------------------- | ----------------------------------------------------------- |
| **`thread::spawn`** | Start a new thread      | Executes closure in parallel and returns a `JoinHandle`     |
| **`JoinHandle<T>`** | Manage a spawned thread | `join()` waits for the thread and retrieves result or panic |
| **`thread::sleep`** | Pause execution         | Suspends thread without CPU use                             |


#### Thread Synchronization Primitives in Rust

Rust provides powerful primitives to safely share and synchronize data between threads, preventing 
data races and ensuring consistency.

---

##### 1. **`Mutex<T>` — Mutual Exclusion**

* **Purpose:** Protect shared data by allowing only one thread at a time to access it.
* **How it works:** Threads lock the mutex before accessing the data and unlock after they’re done.
* **Prevents:** Simultaneous access that can cause race conditions.
* **Example:**

```rust
use std::sync::Mutex;

let counter = Mutex::new(0);

{
    let mut num = counter.lock().unwrap();  // Lock the mutex
    *num += 1;                              // Modify the protected data
}  // Mutex unlocked automatically here
```

---

##### 2. **`Arc<T>` — Atomic Reference Counting**

* **Purpose:** Safely share ownership of data between multiple threads.
* **How it works:** It keeps a thread-safe reference count of how many threads own the data.
* **Why needed:** Because Rust’s ownership rules don’t allow multiple owners without synchronization.
* **Often combined with:** `Mutex` to allow mutable access to shared data.
* **Example:**

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}

println!("Result: {}", *counter.lock().unwrap());
```

##### 3. **`Condvar` — Condition Variable**

* **Purpose:** Coordinate threads waiting for certain conditions to be true.
* **How it works:** One or more threads can wait on a condition variable, blocking until notified by another
  thread.
* **Typical usage:** For signaling between threads (e.g., producer-consumer problems).
* **Example:**

```rust
use std::sync::{Mutex, Condvar};
use std::thread;

let pair = std::sync::Arc::new((Mutex::new(false), Condvar::new()));
let pair2 = pair.clone();

thread::spawn(move || {
    let (lock, cvar) = &*pair2;
    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_one();  // Notify waiting thread
});

let (lock, cvar) = &*pair;
let mut started = lock.lock().unwrap();
while !*started {
    started = cvar.wait(started).unwrap();  // Wait until notified
}
println!("Thread signaled!");
```

#### Summary

| Primitive   | Use Case                        | Key Feature                    |
| ----------- | ------------------------------- | ------------------------------ |
| **Mutex**   | Exclusive access to shared data | Lock/unlock to prevent races   |
| **Arc**     | Shared ownership across threads | Thread-safe reference counting |
| **Condvar** | Wait/notify between threads     | Block threads until signaled   |

---
Excellent! Let’s move on to:

---

### How Rust Ensures Safety in Multithreading

Rust’s core strength is its **guarantees about memory safety and data race prevention** — *at compile time*.
This makes multithreaded programming much safer compared to many other languages.

##### 1. **Ownership and Borrowing Rules**

* Rust’s ownership system ensures that **only one mutable reference** or **multiple immutable references** 
  to a piece of data exist at any time.
* This prevents simultaneous mutation and access from different threads, which is the root cause of data races.

##### 2. **`Send` and `Sync` Traits**

Rust uses two special marker traits to control what can be safely transferred or accessed across threads:

| Trait      | Meaning                                                                      | Example                                           |
| ---------- | ---------------------------------------------------------------------------- | ------------------------------------------------- |
| **`Send`** | Types that can be safely moved **to** another thread                         | Primitive types, `String`, most `struct`s         |
| **`Sync`** | Types that can be safely referenced from **multiple threads simultaneously** | `&T` (immutable references), `Mutex<T>`, `Arc<T>` |

* Rust **enforces** that only `Send` types can be passed between threads.
* Only `Sync` types can be safely shared by reference.

---

##### 3. **Safe Concurrency Through Primitives**

* **Mutexes** and **atomic reference counting** (`Arc`) provide safe, explicit sharing.
* Compiler ensures you **lock** the mutex before accessing data, or the code won’t compile.
* This stops data races **before your code even runs**.

---

##### 4. **No Data Races**

Rust defines data races as:

* Two or more pointers access the same memory simultaneously,
* At least one is a write,
* There’s no synchronization.

Rust’s rules prevent this scenario by design.

##### 5. **Zero-cost Abstractions**

* These guarantees are **checked at compile time** with no runtime cost.
* Rust’s concurrency features compile down to efficient native threads without overhead.

Example: Compile-Time Error Preventing Data Race

```rust
use std::thread;

let mut data = vec![1, 2, 3];

let handle = thread::spawn(|| {
    data.push(4);  // ERROR: `data` does not live long enough, and is not `Send`
});

handle.join().unwrap();
```

Rust’s compiler will refuse this code because:

* `data` is borrowed by the closure,
* It’s not `Send` (not safe to transfer across threads),
* Ownership rules are violated.

---

#### Summary of Rust’s Thread Safety

| Mechanism                   | Role                                       |
| --------------------------- | ------------------------------------------ |
| Ownership & Borrow Checker  | Ensures no simultaneous mutable access     |
| `Send` & `Sync` traits      | Controls safe transfer and sharing         |
| Safe concurrency primitives | Enforce synchronization with mutexes, arcs |
| Compile-time checks         | Detect data races before runtime           |



### Message Passing Concurrency

Channel-based communication uses the multiple producer, single consumer pattern:

```rust 
                  +-----------+    +-----------+    +-----------+
                  | Producer1 |    | Producer2 |    | Producer3 |
                  +-----+-----+    +-----+-----+    +-----+-----+
                        \             |                /
                         \            |               /
                          \           v              /
                           +-------------------------+
                           |          Sender         |
                           +------------+------------+
                                        |
                                        v
                               +-----------------+
                               | mpsc::channel   |
                               +--------+--------+
                                        |
                                        v
                                   +----------+
                                   | Receiver |
                                   +----+-----+
                                        |
                    +-------------------+-------------------+
                    |                                       |
                    v                                       v
                 +-------+                               +----------+
                 | recv()|                               | try_recv()|
                 +---+---+                               +-----+----+
                     |                                         |
                     v                                         v
                +---------+                               +---------+
                | Consumer|                               |  Result |
                +---------+                               +---------+
```

This shows how multiple producers send data through a channel to a receiver, which consumes either by using
blocking ( `recv()` ) or non-blocking ( `try_recv()` ).

The `std::mpsc` module provides `channel()`, `Sender<T>`, and `Receiver<T>` for thread-safe communication.

#### Rust `std::mpsc` Module: Thread-Safe Channel Communication


Rust’s `std::mpsc` (multi-producer, single-consumer) module provides a safe way for threads to send data 
between each other using **channels**.

---

##### 1. **`channel()`**

What it does:

* Creates a **new communication channel**.
* Returns a **`Sender<T>`** and a **`Receiver<T>`** pair.
* Allows multiple producers to send messages to a single consumer.

Signature:

```rust
pub fn channel<T>() -> (Sender<T>, Receiver<T>)
```

Key Points:

* The channel buffers messages sent from producers until the receiver processes them.
* It is **thread-safe**, so multiple threads can send messages concurrently.
* If the receiver is dropped, sending causes an error.

Example:

```rust
use std::sync::mpsc;

let (tx, rx) = mpsc::channel();
```

---

##### 2. **`Sender<T>`**

What it is:

* A sending endpoint of the channel.
* Can be cloned to allow multiple producers to send messages concurrently.
* Sends messages of type `T` to the `Receiver`.

Key Methods:

* `send(msg: T)` — Sends a message, **blocking** if the channel buffer is full (for bounded channels).
* `clone()` — Creates another `Sender` instance for more producers.

Behavior:

* If the receiver is dropped, `send` returns an error.
* Used by producer threads to push data into the channel.

Example:

```rust
let tx1 = tx.clone();
tx.send(42).unwrap();
```

---

##### 3. **`Receiver<T>`**

What it is:

* The receiving endpoint of the channel.
* Receives messages sent by `Sender<T>`.
* Only **one** `Receiver` can exist (single-consumer).

Key Methods:

* `recv()` — **Blocking** call that waits until a message is available or the channel is closed.
* `try_recv()` — **Non-blocking** call that returns immediately with a message or an error if none available.

Behavior:

* When all `Sender`s are dropped, `recv()` returns an error signaling channel closure.
* Used by consumer threads to receive and process messages.

Example:

```rust
match rx.recv() {
    Ok(msg) => println!("Got: {}", msg),
    Err(_) => println!("Channel closed"),
}
```

#### Summary Table

| Component     | Purpose                     | Key Method(s)                | Notes                                            |
| ------------- | --------------------------- | ---------------------------- | ------------------------------------------------ |
| `channel()`   | Creates Sender and Receiver | Returns `(Sender, Receiver)` | Supports multi-producer, single-consumer pattern |
| `Sender<T>`   | Sends messages              | `send()`, `clone()`          | Can be cloned for multiple producers             |
| `Receiver<T>` | Receives messages           | `recv()`, `try_recv()`       | Single consumer, blocking/non-blocking calls     |


#### Typical Workflow

1. Call `mpsc::channel()` to get `(Sender, Receiver)`.
2. Clone the `Sender` for each producer thread.
3. Producers call `send()` to push messages.
4. The consumer calls `recv()` or `try_recv()` to receive messages.
5. When all `Sender`s are dropped, `recv()` returns an error to indicate no more messages.

---

##### Example

```rust
use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    thread::spawn(move || {
        tx.send("Message from producer 1").unwrap();
    });

    thread::spawn(move || {
        tx1.send("Message from producer 2").unwrap();
    });

    for received in rx {
        println!("Got: {}", received);
    }
}
```
Assignment : Add Error handling  and channel types to above Example


### Shared State Concurrency

Shared memory access requires synchronization primitives:
| Type  | Purpose | Key Methods  |
| :---  | :--- | :--- |
|Mutex<T> |	Mutual exclusion         | lock(), try_lock()      |
|Arc<T>	  | Atomic reference counting| clone(), strong_count() |
|RwLock<T>|	Reader-writer lock	     | read(), write()         |

```rust 

      +---------+    +---------+    +---------+
      | Thread1 |    | Thread2 |    | Thread3 |
      +----+----+    +----+----+    +----+----+
           \             |             /
            \            |            /
             \           v           /
              +---------------------+
              |    Arc::clone       |
              +----------+----------+
                         |
                         v
                      +------+
                      | Arc> |
                      +---+--+
                          |
                          v
                   +--------------+
                   | mutex.lock() |
                   +------+-------+
                          |
                          v
                   +--------------+
                   | MutexGuard   |
                   +------+-------+
                          |
                          v
                  +-------------------+
                  | shared_data_access |
                  +-------------------+
```
- **Thread1, Thread2, Thread3:** Multiple threads are running concurrently.
- **Arc::clone:** Each thread clones the `Arc` to share ownership of the same data.
- **Arc<T>:** A thread-safe reference-counted pointer managing shared data.
- **mutex.lock():** Each thread attempts to lock the mutex to gain exclusive access.
- **MutexGuard:** The lock guard that ensures safe access and automatically releases the lock.
- **shared_data_access:** The thread safely accesses or modifies the shared data protected by the mutex.

#### Arc Atomic operation magic:

**`Arc` provides atomic reference counting**, allowing **multiple owners (threads)** to safely share the 
same underlying data—in this case, a `mutex`. 

This means:

- The **ownership of the mutex-protected data is shared safely** across threads without data races.
- `Arc` ensures the data lives as long as there are owners.
- The **mutex inside the `Arc` guarantees exclusive access** when a thread locks it.
- Together, `Arc` + `Mutex` enable **safe, concurrent access** to shared mutable data across threads.

So yes, **Arc’s atomic operations handle ownership management**, while the **mutex handles synchronization**
to protect the data during access.

### Concurrency Traits￼

The `Send` and `Sync` traits define thread safety guarantees:

`Send`: Types safe to transfer between threads
`Sync`: Types safe to share references between threads

Rust uses these two fundamental **marker traits** to express thread safety guarantees at the type level:

#### 1. **`Send` Trait**

What it means:

* A type that implements `Send` **can be safely transferred (moved) from one thread to another**.
* Ownership of the value can cross thread boundaries without causing undefined behavior.

Key points:

* Most primitive types, smart pointers, and many Rust standard types are `Send`.
* Types containing raw pointers or non-thread-safe interior mutability usually **do NOT** implement `Send`.
* When you spawn a thread with `thread::spawn()`, the closure’s environment must be `Send`.

Example:

```rust
let v = vec![1, 2, 3];  // Vec<T> is Send if T is Send
thread::spawn(move || {
    println!("{:?}", v);  // `v` moved safely to new thread
});
```

#### 2. **`Sync` Trait**

What it means:

* A type that implements `Sync` **can be safely referenced from multiple threads simultaneously**.
* It guarantees that **immutable references (&T) to that type can be shared safely across threads**.

Key points:

* Types with internal synchronization (e.g., `Mutex<T>`) are `Sync`.
* Immutable types and many built-in types are naturally `Sync`.
* Types that allow mutation without synchronization are **not `Sync`**.

Example:

```rust
use std::sync::Mutex;

let m = Mutex::new(5);  // Mutex<T> is Sync
let r = &m;             // &Mutex<T> can be shared safely between threads
```

---

#### Why Are These Traits Important?

* They **enforce Rust’s guarantees about safe concurrency at compile time**.
* The Rust compiler checks that only `Send` types are moved between threads.
* It also ensures shared references are only to `Sync` types.
* This prevents data races and undefined behavior **before your program even runs**.

---

#### Summary Table

| Trait    | Meaning                                    | Example Types                            |
| -------- | ------------------------------------------ | ---------------------------------------- |
| **Send** | Safe to transfer ownership between threads | `i32`, `Vec<T>`, `Box<T>` (if `T: Send`) |
| **Sync** | Safe to share references between threads   | `&T`, `Mutex<T>`, `Arc<T>`               |

---

##### Combined Example

```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));  // Arc<T> and Mutex<T> are Send + Sync
let counter2 = Arc::clone(&counter);

thread::spawn(move || {
    let mut num = counter2.lock().unwrap();
    *num += 1;
}).join().unwrap();
```

### Asynchronous Programming Model￼

Async Syntax and Futures
The async model uses `async fn` and `await` keywords with the `Future` trait:

```rust 
    Runtime execution         Async Function Compilation
    -----------------         --------------------------
    trpl::run              async fn page_title        await            
        ↓                          ↓                     ↓             
    async_runtime              impl Future            Future::poll     
        ↓                          ↓                     ↙      ↘      
    executor                 async move block          Poll       Poll 
        ↓
    task_polling
``` 
Key components include the `trpl` crate, `Future` trait, and runtime execution via `trpl::run`.
Refer **introToRustAsyncProg.md** for `Future`, `async`, `await`.

#### - The trpl Crate Ecosystem￼

The `trpl` crate is a utility library that **re-exports and consolidates functionality from the broader 
Rust async ecosystem**, making it easier to work with asynchronous programming patterns. 

It provides convenient wrappers for common tasks such as HTTP requests, async delays, task spawning, 
channel communication, and combining or racing futures, all in a unified API.

| Function/Type      | Source        | Purpose            |
| ------------------ | ------------- | ------------------ |
| `trpl::get`        | tokio/reqwest | HTTP requests      |
| `trpl::sleep`      | tokio         | Async delays       |
| `trpl::spawn_task` | tokio         | Task spawning      |
| `trpl::join`       | futures       | Future combination |
| `trpl::race`       | futures       | Future racing      |
| `trpl::channel`    | tokio         | Async channels     |

#### - Task Management and Concurrency:

=> Async Concurrency uses tasks instead of threads

```rust 
                 +----------------------+
                 |   Future Coordination |
                 +----------------------+

   +------------+          +------------+          +----------------+
   | trpl::join |          | trpl::race |          | trpl::join_all |
   +------------+          +------------+          +----------------+
          |                       |                        |
          v                       v                        v
+--------------------+   +------------------+   +----------------------+
| concurrent_execution|   | first_completion|   |   all_completion     |
+--------------------+   +------------------+   +----------------------+

-----------------------------------------------------------------------------------------
                     +------------------+
                     |  Task Spawning   |
                     +------------------+

   +------------------+                   +--------------------+
   | trpl::spawn_task |                   | main_async_block   |
   +------------------+                   +--------------------+
             |                                      |
             v                                      v
      +--------------+                       +--------------+
      |  async_task  |                       |   trpl::run  |
      +--------------+                       +--------------+
             |                                      |
             v                                      v
     +----------------+                     +---------------------+
     |  JoinHandle    |                     | runtime_executor    |
     +----------------+                     +---------------------+
             |                                      |
             v                                      v
   +--------------------+                 +-----------------------+
   |   await handle     |                 |   task_scheduling     |
   +--------------------+                 +-----------------------+

```

- `tlpr` crate: Mostly re-exports other crates,  'tokio, 'futures' and other crated to make asyn programming concept in rust clear.
```toml 
# tlpr: Cargo.toml 

[dependencies]
futures = "0.3"
reqwest = { version = "0.12", default-features = false, features = [
    "rustls-tls",
] }
scraper = "0.20"
tokio = { version = "1", default-features = false, features = [
    "fs",
    "rt-multi-thread",
    "sync",
    "time",
] }
tokio-stream = "0.1"
```

- `trpl` crate provides a clear conceptual bridge between standard Rust `async` concepts and your custom 
  API names (`trpl::join`, `trpl::race`, `trpl::spawn_task`, etc.).


### Concurrency with Async Programming (using the `trpl` crate)

Rust's `async` model provides **lightweight concurrency** built on *`futures`* and *`tasks`*, rather than 
system threads. 

Instead of preemptive multitasking, Rust `async` works through **cooperative** progress: 
    futures yield when they cannot make progress, and the *runtime* wakes them when they can continue.

In this model:

* **Futures** represent values that will be available later.
* **Tasks** are asynchronous units of execution managed by a runtime.
* **An executor** polls tasks until completion.

Your diagram maps cleanly to Rust's `async` workflow:

#### 1. Futures and Coordination

A `Future` in Rust is an asynchronous computation. It does nothing on its own; it must be **polled** by an 
executor.

The `trpl` crate provides combinators to coordinate multiple futures:

```
+----------------------+
|  Future Coordination |
+----------------------+
```

##### 1.1 `trpl::join`: Run Futures Concurrently

`trpl::join` waits for **all** futures to complete.
This allows several independent async operations to happen *simultaneously* (concurrently).

Example

```rust
use trpl::{join, sleep};

async fn fetch_user() { /* ... */ }
async fn fetch_posts() { /* ... */ }

async fn load_dashboard() {
    let (user, posts) = join(fetch_user(), fetch_posts()).await;
}
```

Conceptually matches:

```
+------------+
| trpl::join |
+------------+
      |
      v
+--------------------+
| concurrent_execution|
+--------------------+
```

##### 1.2 `trpl::race`: Wait for the First Completion

`trpl::race` resolves as soon as *one* future completes.

Example

```rust
use trpl::race;

async fn fastest() {
    let winner = race(do_first(), do_second()).await;
}
```

Diagram mapping:

```
+------------+
| trpl::race |
+------------+
      |
      v
+------------------+
| first_completion |
+------------------+
```

##### 1.3 `trpl::join_all`: Wait for Every Future in a List

Useful when handling dynamically sized work sets.

```rust
use trpl::join_all;

async fn run_batch(tasks: Vec<impl Future>) {
    join_all(tasks).await;
}
```

Diagram mapping:

```
+----------------+
| trpl::join_all |
+----------------+
        |
        v
+----------------------+
|    all_completion    |
+----------------------+
```

#### 2. Task Spawning

```
+------------------+
|  Task Spawning   |
+------------------+
```

Futures by themselves are inert. To run them concurrently, they need to be **spawned** onto a runtime as *tasks*.

##### 2.1 `trpl::spawn_task`: Create a Lightweight Async Task

`spawn_task` creates a new async task executed by the runtime.
It returns a `JoinHandle`, which you can `.await` to get the result.

```rust
use trpl::spawn_task;

async fn background_job() { /* ... */ }

async fn run() {
    let handle = spawn_task(background_job());
    let result = handle.await;
}
```

Diagram:

```
+------------------+
| trpl::spawn_task |
+------------------+
          |
          v
    +--------------+
    |  async_task  |
    +--------------+
          |
          v
    +----------------+
    |  JoinHandle   |
    +----------------+
          |
          v
+--------------------+
|   await handle     |
+--------------------+
```

#### 3. The Async Entry Point

Rust does not allow `async fn main()` by default, because futures need a running executor.
The `trpl` crate provides a macro and runtime to achieve this.

---

##### 3.1 `main_async_block` and `trpl::run`

```rust
use trpl::run;

async fn main_async_block() {
    // spawn tasks, join futures, etc.
}

fn main() {
    run(main_async_block());
}
```

Diagram mapping:

```
+--------------------+      +--------------+
| main_async_block   | ---> | trpl::run    |
+--------------------+      +--------------+
                                     |
                                     v
                          +---------------------+
                          | runtime_executor    |
                          +---------------------+
                                     |
                                     v
                          +-----------------------+
                          |   task_scheduling     |
                          +-----------------------+
```

#### 4. How the Async Runtime Works

The executor:

1. Holds a queue of active tasks
2. Polls tasks until they yield (e.g., waiting on I/O)
3. Wakes tasks when progress can resume
4. Repeats until all tasks finish

Tasks are cheap — far cheaper than OS threads—allowing Rust to scale efficiently.

#### 5. Summary Table (Above Diagrams ↔ trpl API)

| Concept                 | Diagram Node        | trpl Equivalent            | What It Does                |
| ----------------------- | ------------------- | -------------------------- | --------------------------- |
| Concurrency via futures | Future Coordination | `join`, `race`, `join_all` | Run several futures at once |
| Spawning tasks          | Task Spawning       | `spawn_task`               | Run a future in background  |
| Async main              | main_async_block    | `run`                      | Start the runtime           |
| Task result             | JoinHandle          | await handle               | Retrieve task output        |
| Runtime                 | runtime_executor    | internal                   | Schedules and polls tasks   |


### - Message Passing with Async Channels:

Async channels provide similar functionality to thread channels but with `async` operations:

#### Async Channels (using `trpl`)

Async channels allow asynchronous tasks to **communicate safely** by sending values from a *producer* to a 
*consumer*.

Channels transfer **ownership**, prevent data races, and integrate seamlessly with the `async` runtime.

##### Flowchart: Async Channel Flow (trpl)

```
                        +-----------------------+
                        |     Async Channel     |
                        +-----------------------+
                                  |
                          trpl::channel()
                                  |
        ----------------------------------------------------------------
        |                                                              |
        v                                                              v
+--------------------+                                      +---------------------+
|   async_producer   |                                      |   async_consumer    |
+--------------------+                                      +---------------------+
          |                                                              |
          | tx.send(value).await                                         | rx.recv().await
          v                                                              v
+---------------------+                                      +------------------------+
|   send operation    |                                      |    receive operation   |
+---------------------+                                      +------------------------+
          |                                                              |
          v                                                              |
+-------------------------+      ownership_transfer      +--------------------------+
| enqueue into channel    | ---------------------------> |   delivered to consumer  |
+-------------------------+                               +--------------------------+
                                  |
                                  v
                       +---------------------------+
                       |     channel_cleanup       |
                       |  (when tx or rx drops)    |
                       +---------------------------+
```
##### 1. Creating the Channel

Using `trpl::channel`, the program creates a **sender (`tx`)** and a **receiver (`rx`)**:

```rust
let (tx, rx) = trpl::channel();
```

* `tx` lets the producer *send* values.
* `rx` lets the consumer *receive* them.

Both halves can be cloned or moved to different tasks.

##### 2. The Producer Task (`async_producer`)

The producer owns the sending half:

```rust
async fn async_producer(tx: Sender<Data>) {
    tx.send(data).await;
}
```

###### `tx.send(value).await`

* Async operation that yields until the channel has capacity.
* Transfers **full ownership** of `value` into the channel queue.
* Never blocks the thread, only the async task.

Flowchart node:

```
async_producer → tx.send() → send operation → channel queue
```

##### 3. Ownership Transfer

Rust ensures that the value being sent is **moved**:

```
producer (value owner)
      ─────────────→ ownership_transfer → consumer (new owner)
```

This guarantees:

* no cloning unless chosen,
* no shared mutable references,
* data race freedom.

---

##### 4. The Consumer Task (`async_consumer`)

The consumer uses the receiver:

```rust
async fn async_consumer(mut rx: Receiver<Data>) {
    while let Some(v) = rx.recv().await {
        // process v
    }
}
```

###### `rx.recv().await`

* Waits asynchronously until a value arrives.
* Returns `None` only when **all senders are dropped**.

Flowchart node:

```
rx.recv().await → receive operation → consumer receives ownership
```

#### 5. Channel Cleanup

A channel shuts down cleanly when:

* all `tx` handles are dropped → consumer receives `None`.
* or the `rx` handle is dropped → producers get an error on send.

This behavior aligns with Rust’s ownership model.

Flowchart node:

```
channel_cleanup
```

---

#### - Full Example (using `trpl`)

```rust
use trpl::{channel, spawn_task};

async fn async_producer(tx: Sender<String>) {
    tx.send("hello".to_string()).await;
    tx.send("world".to_string()).await;
}

async fn async_consumer(mut rx: Receiver<String>) {
    while let Some(msg) = rx.recv().await {
        println!("Received: {msg}");
    }
}

async fn main_async_block() {
    let (tx, rx) = channel();

    let prod = spawn_task(async_producer(tx));
    let cons = spawn_task(async_consumer(rx));

    prod.await;
    cons.await;
}

fn main() {
    trpl::run(main_async_block());
}
```

Key differences include `mut rx` receivers and awaitable `recv()` operations.



