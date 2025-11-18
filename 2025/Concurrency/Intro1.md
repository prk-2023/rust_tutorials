# Concurrency

Rust’s concurrency model is famous for allowing **safe, data-race–free parallelism** — all enforced **at
compile time**. Let’s explore how it works and build up from simple threads to shared-state concurrency.

---

## Tutorial: Fearless Concurrency in Rust

By the end of this tutorial, you’ll understand:

* How to spawn threads safely
* How ownership and `move` semantics make concurrency safer
* How to share data between threads using `Arc<T>`
* How to synchronize access with `Mutex<T>` and `RwLock<T>`
* How Rust prevents data races **by design**

---

## 1.0 The Problem: Concurrency Is Hard

In many languages, concurrency introduces classic problems:

* 🔁 **Data races** (two threads read/write shared memory unsafely)
* 🔒 **Deadlocks**
* ❓ **Unclear ownership** of data

Rust’s goal is **“fearless concurrency”** — letting you write concurrent code *confidently*, knowing the
compiler enforces safety.

---

## 2.0 Spawning Threads

Rust’s standard library provides the `std::thread` module. You can create a new thread with `thread::spawn`.

```rust 
use std::thread;
use std::time::Duration;

fn main() { 
    thread::spawn(|| 
    { 
        for i in 1..=5 {
            println!("🧵 From spawned thread: {i}");
            thread::sleep(Duration::from_millis(200)); 
        } 
           
    });

    for i in 1..=3 { 
        println!("🧠 From main thread: {i}"); 
        thread::sleep(Duration::from_millis(300)); 
    } 
}
```

Output (order may vary):

``` 
🧠 From main thread: 1 
🧵 From spawned thread: 1 
🧠 From main thread: 2 
🧵 From spawned thread: 2 ...
```

✅ Threads run concurrently. ⚠️ But the program might end before the spawned thread finishes — we’ll fix
that next.

---

## 3.0 Waiting for Threads: `join()`

The `JoinHandle` returned by `thread::spawn` can be used to **wait** for a thread to finish.

```rust use std::thread;

fn main() { let handle = thread::spawn(|| { for i in 1..=5 { println!("🧵 Working... {i}"); } });

    handle.join().unwrap(); // Wait for thread to finish println!("✅ Thread completed!"); } ```

✅ `join()` blocks until the spawned thread finishes. ✅ If the thread panics, `join()` returns an error.

---

## 4.0 Ownership and `move` in Threads

Threads **outlive their creation scope**, so you must **move** ownership of any data into the closure.

```rust use std::thread;

fn main() { let numbers = vec![1, 2, 3];

    let handle = thread::spawn(move || { for n in &numbers { println!("Thread has number: {n}"); } });

    handle.join().unwrap(); } ```

✅ The `move` keyword transfers ownership of `numbers` into the thread. ❌ Without `move`, the closure would
try to borrow `numbers`, which would end when `main`’s scope ends — unsafe!

---

## 5.0  Sharing Data Between Threads

What if multiple threads need access to the same data?

* You can’t share mutable references safely.
* You can’t clone large data easily.

Rust gives you:

* `Arc<T>` → **Atomically Reference Counted** smart pointer (thread-safe `Rc<T>`)
* `Mutex<T>` → **Mutual exclusion lock** for synchronized access

Combine them for shared, mutable data:

> `Arc<Mutex<T>>`

---

## 6.0 Example: Shared Counter Across Threads

```rust use std::sync::{Arc, Mutex}; use std::thread;

fn main() { let counter = Arc::new(Mutex::new(0)); let mut handles = vec![];

    for _ in 0..5 { let counter = Arc::clone(&counter); let handle = thread::spawn(move || { for _ in 0..10
    { let mut num = counter.lock().unwrap(); *num += 1; } }); handles.push(handle); }

    for handle in handles { handle.join().unwrap(); }

    println!("Final count: {}", *counter.lock().unwrap()); } ```

Output:

``` Final count: 50 ```

### What happened

* Each thread gets a clone of the same `Arc<Mutex<i32>>`.
* `Mutex` ensures only one thread modifies the counter at a time.
* When the lock (`num`) goes out of scope, it’s automatically released.

✅ Safe concurrent mutation. ✅ No data races. ✅ Compiler enforces proper locking and ownership.

---

## 7.0 How `Arc<Mutex<T>>` Works Together

| Type            | Purpose                                     |
| --------------- | ------------------------------------------- |
| `Arc<T>`        | Allows **shared ownership** between threads | 
| `Mutex<T>`      | Allows **mutability** with synchronization  | 
| `Arc<Mutex<T>>` | Shared **and mutable** data across threads  |

🧠 Analogy:

> `Rc<RefCell<T>>` → single-threaded shared mutability `Arc<Mutex<T>>` → multi-threaded shared mutability

---

## 8.0 Common Mutex Pitfalls

### Deadlock example:

```rust use std::sync::{Arc, Mutex}; use std::thread;

fn main() { let a = Arc::new(Mutex::new(1)); let b = Arc::new(Mutex::new(2));

    let a1 = Arc::clone(&a); let b1 = Arc::clone(&b);

    let t1 = thread::spawn(move || { let _lock_a = a1.lock().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100)); let _lock_b = b1.lock().unwrap();
    println!("Thread 1 finished"); });

    let a2 = Arc::clone(&a); let b2 = Arc::clone(&b);

    let t2 = thread::spawn(move || { let _lock_b = b2.lock().unwrap();
    std::thread::sleep(std::time::Duration::from_millis(100)); let _lock_a = a2.lock().unwrap();
    println!("Thread 2 finished"); });

    t1.join().unwrap(); t2.join().unwrap(); } ```

⚠️ This can deadlock because threads lock resources in different orders (`a → b` vs `b → a`).

✅ Always lock shared resources in a consistent order.

---

## 9.0  Message Passing with Channels

Rust’s second concurrency model is **message passing**, using `std::sync::mpsc` (multi-producer,
single-consumer).

```rust use std::sync::mpsc; use std::thread; use std::time::Duration;

fn main() { let (tx, rx) = mpsc::channel();

    thread::spawn(move || { let vals = vec!["one", "two", "three"]; for v in vals { tx.send(v).unwrap();
    thread::sleep(Duration::from_millis(200)); } });

    for received in rx { println!("Got: {received}"); } } ```

Output:

``` Got: one Got: two Got: three ```

✅ Threads communicate by transferring ownership through channels — no shared memory needed. ✅ Channels
guarantee *safe data transfer* between threads.

---

## 10.0 Concurrency Summary

| Pattern               | Tool            | Description                                | 
| --------------------- | --------------- | ------------------------------------------ | 
| **Spawning threads**  | `thread::spawn` | Run work in parallel                       | 
| **Shared ownership**  | `Arc<T>`        | Thread-safe reference counting             | 
| **Mutual exclusion**  | `Mutex<T>`      | Safe shared mutability                     | 
| **Message passing**   | `mpsc::channel` | Send data between threads                  | 
| **Async concurrency** | `async/await`   | Cooperative multitasking (see Chapter 17+) |

---

## Analogy: Single vs Multi-threaded Smart Pointers

| Single-threaded | Multi-threaded           | Description         |
| --------------- | ------------------------ | ------------------- |
| `Rc<T>`         | `Arc<T>`                 | Shared ownership    | 
| `RefCell<T>`    | `Mutex<T>` / `RwLock<T>` | Interior mutability |

They serve the same conceptual purposes — but `Arc` and `Mutex` add **atomic operations** and **thread
safety**.

---

## Example: Safe Parallel Computation

```rust use std::sync::{Arc, Mutex}; use std::thread;

fn main() { let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5])); let mut handles = vec![];

    for i in 0..5 { let data = Arc::clone(&data); let handle = thread::spawn(move || { let mut vec =
    data.lock().unwrap(); vec[i] *= 2; }); handles.push(handle); }

    for h in handles { h.join().unwrap(); }

    println!("Result: {:?}", *data.lock().unwrap()); } ```

Output:

``` Result: [2, 4, 6, 8, 10] ```

Each thread safely mutates its own index — all enforced by the borrow checker.

---

## Key Takeaways

> Rust’s concurrency is “fearless” because:
>
> * The compiler enforces ownership and borrowing across threads.
> * You can’t have data races — only compile errors or controlled locks.
> * Patterns like `Arc<Mutex<T>>` and channels let you share or transfer data safely.

---

##  Summary Table

| Tool            | Use Case                        | 
| --------------- | ------------------------------- |
| `thread::spawn` | Run concurrent tasks            | 
| `join()`        | Wait for completion             |
| `move`          | Transfer ownership to threads   |
| `Arc<T>`        | Shared ownership across threads |
| `Mutex<T>`      | Safe interior mutability        | 
| `mpsc::channel` | Message passing concurrency     |
| `RwLock<T>`     | Multiple readers, one writer    |

---

Would you like me to continue with the **next logical part — Chapter 17: “Object-Oriented Programming
Features of Rust”**, which connects everything you’ve learned about ownership, traits, and smart pointers to
object-oriented design patterns (like polymorphism and trait objects)?


# Concurrency ( Book )

Concurrent programming: different parts of the program execute independently.
Parallel programming: Different parts of the program execute at same time, are becoming increasingly
important as more computers take advantage of their multiple processors.

Rust team initially thought that ensuring memory-safety and preventing concurrency problem are two seperate
challenges to be solves with different methods. Over time they discovers that Ownership and type systems are
a powerful set of tools to help manage memory safety and concurrency problems.

- By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather
  then runtime errors. This helps to fix your code while you're working on it rather then potentially after
  it has been shipped to production. 


## Using Threads to Run Code Simultaneously: 

Splitting the computation in your program into multiple threads to run multiple tasks at the same time can
improve performance, but it also adds complexity. 
Because threads can run Simultaneously there's no inherent guarantee about the order in which parts of your
code on different threads will run. This can lead to problems such as :

- Race condition : Threads are accessing data or resources in a inconsistent order. 
- Deadlock: In which two threads are waiting for each other, preventing both threads from continuing. 
- Bugs that only happen in certain situations and are hard to reproduce and fix reliably. 

Programming languages implement threads in a few different ways, and many OS's provide an API the
programming language can call for creating new threads. 
Rust Standard library uses a 1:1 model of threads implementation, whereby a program uses one OS thread per
one language thread. ( there are other implementations of threads by different Crates which make different
trade-offs to the 1-1 model. ( Async system provides another approach to concurrency as well ) )


### Creating a New thread with **`spawn`**:


- To create a new thread we call the `thread::spawn` function and pass it a closure containing the code we
  want to run in the new thread. Ex:

```rust 

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("Number #{i} from Spawned thread!");
            thread::sleep(Duration::for_millis(10));
        }
    });

    for i in 1..5 {
        println!("Number #{i} from main thread!");
        thread::sleep(Duration::for_millis(10));
    }
}
```
When the main thread completes, all spawned threads are shut down, whether or not they have finished
running. 

The call `thread::sleep` forces thread to stop execution for a short period of duration, allowing different
thread to run. The threads take turns, but that isn;t guaranteed: if depends on how the OS schedules threads

The above example we have 10 threads to be spawned, but we will only get till 5 and main thread quits after
5 threads are spawned.

If we have to fix this and let all the threads finish their job before terminating the program we can use
`join` Handles


### Wait for all threads to Finish using Join Handles:

We can fix the problem of the spawned thread not running or of it ending prematurely by saving the return
value of `thread::spawn` in a variable. The return type of `thread::spwan` is `JoinHandle<T>`. 

A `JoinHandle<T>` is an owned value that, when we call the `join()` method on it, will wait for its thread
to finish. 

```rust 
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
            println!("Hi number {i} from the Main thread");
            thread::sleep(Duration::from_millis(100));
    }
    handle.join().unwrap();
}
```
The `join()` causes main thread to wait till the spawned thread is finished.

Moving `join()` before the for loop: In this case the main will wait for the spawned threads to finish and 
then run for loop.

```rust 
use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Hi number {i} from the spawned thread");
            thread::sleep(Duration::from_millis(100));
        }
    });
    handle.join().unwrap();
    for i in 1..5 {
            println!("Hi number {i} from the Main thread");
            thread::sleep(Duration::from_millis(100));
    }
}

$ cargo run 
Hi number 1 from the spawned thread                                                                                                                                                            │
Hi number 2 from the spawned thread                                                                                                                                                            │
Hi number 3 from the spawned thread                                                                                                                                                            │
Hi number 4 from the spawned thread                                                                                                                                                            │
Hi number 5 from the spawned thread                                                                                                                                                            │
Hi number 6 from the spawned thread                                                                                                                                                            │
Hi number 7 from the spawned thread                                                                                                                                                            │
Hi number 8 from the spawned thread                                                                                                                                                            │
Hi number 9 from the spawned thread                                                                                                                                                            │
Hi number 1 from the Main thread                                                                                                                                                               │
Hi number 2 from the Main thread                                                                                                                                                               │
Hi number 3 from the Main thread                                                                                                                                                               │
Hi number 4 from the Main thread
```
### Using `move` Closures with Threads:

- What is `move` in closures?

    * Closures can capture variables from their surrounding environments, either by borrowing them (creating
      a reference ) or by **taking ownership** of them.

    * By default, closures borrow values. However using the `move` keyword forces the closure to take
      ownership of the variables it uses, which means the closure will move those values into its
      environment. This is particularly important when passing closures to threads, as it ensures that the
      values are safely transferred to the new thread.

    ```rust 
    fn main() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("printing the borrowed vector : {v:?}");
            thread::sleep(Duration::from_millis(100));
        } });

        for i in 1..5 {
            println!("Hi number {i} from the Main thread");
            thread::sleep(Duration::from_millis(100));
        }
            handle.join().unwrap();
        }
    ```
    This fail to compile with error (may outlive borrowed value) indicates that the reference to `v` might
    not be valid when the thread starts executing. 
    This is Rust’s way of preventing potential dangling references, where the closure might try to access a 
    value that has already been dropped or moved in the main thread.

- Why use `move` with `thread::spawn` :

    * `thread::spawn` runs closures in a *new thread**, and because thread in Rust have independent
      lifetimes, we need to ensure that the values used by the thread ( in the closure ) live long enough to
      be valid in the context of the thread's execution. 

    * If we don't use `move` rust will default to borrowing values from the main thread, and it won't know
      how long those references are valid. This issue arises because Rust can't guarantee how long the
      spawned thread will live, so it can't guarantee that a reference to `v` in the main thread will still
      be valid when the thread runs. 

    ```rust 
    fn main() {
        let v = vec![1, 2, 3];
        let handle = thread::spawn( move || { // <= We use move here to  take ownership of surrounding vars
        for i in 1..10 {
            println!("printing the borrowed vector : {v:?}");
            thread::sleep(Duration::from_millis(100));
        } });

        for i in 1..5 {
            println!("Hi number {i} from the Main thread");
            thread::sleep(Duration::from_millis(100));
        }
            handle.join().unwrap();
        }
    ```
- So `move` keyword forces a closure to take ownership of variables it uses, which is necessary when passing
  closured to threads because the lifetime of the threads might exceed the lifetime of the original data. 

- Note: when a variable is moved into the closure, it can't be used after the move in the original thread,
  or dropped ( drop(v) ) this would be caught by the compiler with a error. 

=> `move` keyword with closures ensures values are safely transferred to new thread, preventing issues with
invalid references. And also enforces Rust's strict ownership rules, ensuring that you don't accidentally
use values after they've been moved.

### Use Message passing to transfer data between threads:

- A popular approach to ensure safe concurrency is *message passing*, where threads or actors communicate by
  sending each other messages containing data. ( Do not communicate by sharing memory; Instead share memory
  by communicating. )

- Rust std lib provides **channels** which help accomplish message-sending concurrency. 

- core idea of message passing is that threads communicate by sending each other messages, rather then
  sharing memory directly. This approach helps avoid race conditions and other concurrency issues by
  ensuring that threads don't have to directly access each other's memory. 

- **channel** is a general programming concept by which data is sent from one thread to another. And it has
  two parts (transmitter/receiver). With one part of the code calls methods on transmitter data you want to
  send, and another part checks the receiving end for arriving message.

  A channel is said to be closed if either the transmitter or receiver half is dropped.

- channels: Rust std lib has built-in mechanism for msg passing via channels:(std::sync::mpsc::channel) and
  a channel consists of "transmitter" and "receiver".
  1. Transmitter (sender):The part that sends data into the channel.
  2. Receiver: Part of the code that reads data from the channel.

  Rust implementation of channels follows a **multiple producer single consumer model** `mpsc`.
  (mpsc : stands for multiple producers, single consumer). 
  => you can have multiple senders (producers) putting messages into the channel, but only one receiver 
  (consumer) that reads from it.

  ```rust 
    use std::sync::mpsc;
    fn main() {
        let (tx, rx) = mpsc::channel();
    }
  ```
  - channel is created with `mpsc::channel` function which returns a tuple, the 1st element is sender(tx)
    used to send data into the channel.  And second element is receiver (rx) used to retrieve data from the
    channel.

- example:
  ```rust 
  use std::sync::mpsc;
  use std::thread;
  fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });
    let received = rx.recv().unwrap();
    println!("Got: {received}");
  }
  ```
  - `move` ensure that the sender (tx) is owned by the thread, and can be used to send data.
    * new thread is spawned using `thread::spawn`.
    * Inside the closure a message "hi" is sent through the channel using `tx.send(val)` 
    * unwrap() is used to panic in case of error. In real world scenario you likely handle the error
      gracefully.

  - `rx.recv()` is blocking operation: it waits until a message is received. Once a message is received it
    returns the value inside a `Result<T,E>`. If the channel is closed and no more messages are coming,
    `recv()` will return an error, which can be handled accordingly.

- There are 2 ways to receive messages: `recv` and `try_recv`

    * `recv` : blocking, blocks receiver until a message is available. 
    * `try_recv`: non blocking, it checks if there is a message immediately available in the channel. If
      there is it returns the message. If not it returns error. 

    example:
    ```rust 
        loop {
            match rx.try_recv() {
                Ok(msg) => { println!("Got:{msg}") ;}
                Err(_)  => { 
                    // Do some other work while waiting for message 
                    println!("No message: Doing other work...");
                    thread::sleep(Duration::from_millis(1));
                }
            }
        }
    ```

- Summary:
    1. message passing helps ensure threads safety by allowing to communicate through channels, rather then
       sharing memory.
    2. A channel has two parts (tx, rx)
    3. To send a message to another thread, you move the transmitter (context) into the thread and call
       `tx.send()`
    4. To recv message you use `rx.recv()` blocking or `rx.try_recv()` for non-blocking.

#### Channels and Ownership transference:

- To avoid issues like data races, where multiple threads try to access or modify the same data
  concurrently, Rust uses channels to provide a safe communication between threads. However when you send
  data through a channel, the ownership of the data is transferred from sender to the receiver. 

- Problem: using a value after its sent through the channel:

    ```rust 
    use std::sync::mpsc;
    use std::thread;

    fn main() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();    //<= Ownership of val is sent to receiver 
            println!("val is {val}"); //<= trying to access after  move ( Compiler stops with error )
        });

        let received = rx.recv().unwrap();
        println!("Got: {received}");
    }
    ```
    - Rust’s ownership system is preventing a potential issue preventing undefined behavior. Ownership
      enforcement guarantees:
      - No data races ( since only one thread owns the data at any given time )
      - No use-after-free error. 

Summary: ownership and message passing work:
    1. When you send data through a channel, the ownership of that data is moved to the receiver, and Rust
       prevents any further use of that data in the sender thread.
    2. Rust’s ownership model guarantees that you can’t accidentally use data that has already been moved,
       avoiding problems like data races or use-after-free errors.
    3. Channels provide a safe way to communicate between threads, and Rust’s ownership system ensures that
       once a value is sent, it’s no longer accessible in the sending thread, protecting the integrity of 
       the data and preventing concurrency bugs.

#### Sending Multiple Values and Seeing the Receiver Waiting:

How to send multiple values over a channel from one thread to another and shows how iterating over the 
receiver can provide insight into thread synchronization. This also highlights the idea that the receiver 
(main thread) waits for messages, while the sender (spawned thread) sends multiple messages with pauses in 
between.

```rust 
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
```
- Multiple messages sent , where `vals` vector holds 4 string "hi", "from", "the" and "thread".
- Pause between send of 1 sec to prevent flooding the receiver with messages.
- Receiver as Iterator: Instead of calling `rx.recv()` we use `rx` as an iterator in `for` loop:
  Which means loop automatically wait for new messages to arrive from the channel. 
- Once the channel is closed ( when sender has finished sending all messages ) the loop terminates. 
- use of `thread::sleep` makes the concurrency behavior of the program visible, if we remove the
  `thread::sleep` it would be hard to see the concurrency. 
- The `rx` channel implements the`IntoIterator` trait, so it can be used directly in a `for` loop. When
  there are no more values to receive ( i.e the channel is closed ) the loop exits.
  This makes `for` loop a clean and idiomatic way to receive multiple messages from a channel without 
  needing to manually call `recv()` each time.


#### Creating Multiple Producers by Cloning the Transmitter:

Using multiple producers for sending msgs to the same single consumer, using the **cloning the transmitter
`tx`**, this enables multiple threads (producers) send data to a single receiver (consumer).

- `std::sync::mpsc` : multiple producer, single consumer. This means:
    * Multiple threads can send messages (producers), but only one thread can receive messages (consumer).
    * The core idea is that the transmitter (tx) can be cloned, which allows multiple threads to send data
      through the same channel.
    * The receiver (rx), however, is shared only by the receiver thread (in this case, the main thread).

Ex: 
```rust 
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    fn main() {
        let (tx, rx) = mpsc::channel();

        // Clone the transmitter (tx) for use by another thread
        let tx1 = tx.clone();

        // First thread (producer 1)
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // Second thread (producer 2)
        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        // Main thread (consumer)
        for received in rx {
            println!("Got: {received}");
        }
    }
```
- `let tx1 = tx.clone()` : clone the transmitter `tx` to get a new transmitter `tx1`, this allows to send 
   messages to the same receiver (`rx`) from two different threads.

- Main thread listens for incoming messages from the `rx` channel, till the channel is closed. 

- Above code demos:
    * Concurrency ( main thread prints msg as they receive from two threads ) Order of concurrency is not
      guaranteed, as receiver may receive in different order.( due to thread scheduling)

    * Thread Scheduling: ( OS thread scheduler controls the order in which threads execute )


### Shared-State Concurrency: 

Shared state concurrency an alternative to *message-passing* for handling concurrency in Rust.
It allows multiple threads to access the same memory location, which requires careful management to ensure
safety and correctness. 

`Mutex` is one primary tool for managing shared state in concurrent Rust programs.

Shared-state concurrency: occurs when multiple threads access the same data in memory. 
Unlike message-passing, where ownership is transferred between threads, shared-state concurrency allows 
threads to directly read from and write to the same memory location. 
However, without proper synchronization, this can lead to race conditions, where the outcome of a program 
depends on the timing of thread execution.

This demands a new mechanism to control which thread has access to the data at any given time. 
If 2 threads try to read and write the same data concurrently without synchronization, it can lead to data 
corruption, crashes, or unpredictable behavior. Rust's ownership system and mutexes help prevent these issues.

Caution Against memory sharing:
1. Race Conditions: Without careful synchronization, multiple threads could read and modify shared data at 
   the same time, leading to inconsistent or incorrect results.
2. Deadlocks: Threads might wait indefinitely for each other to release resources, causing a freeze.
3. Complexity: Managing shared state across multiple threads often requires careful coordination to avoid 
   conflicts, and the logic can quickly become difficult to maintain.

However, Rust's type system and ownership rules help mitigate these issues by preventing data races and
ensuring proper synchronization.

#### Using Mutexes for Shared-State Concurrency:

A mutex (short for mutual exclusion) is a synchronization primitive that ensures only one thread can access 
a particular piece of data at a time. 
When a thread wants to access the data inside a `mutex`, it must acquire a lock. 
This lock ensures that only one thread can access the data at any given time.

A `mutex` is a valuable tool for shared-state concurrency because it guarantees exclusive access to the 
data it guards, preventing race conditions. 
Once a thread is done using the data, it releases the lock, allowing other threads to acquire it.

Ex1: Mutex in single threaded context:

```rust 
use std::sync::Mutex;
fn main() {
    let m = Mutex::new(5);  // Create a Mutex guarding the value 5
    {
        // Acquire the lock and modify the value
        let mut num = m.lock().unwrap();
        *num = 6;
    }  // The lock is released here, automatically when `num` goes out of scope
    // Print the value inside the Mutex
    println!("m = {m:?}");
}
```
- The `Mutex::new` function creates a new `Mutex` that wraps the value `5`. 
  The value `5` is protected by the `mutex`, ensuring exclusive access.

- To access the data inside the `mutex`, we call `m.lock()`. This will block the current thread until it 
  can acquire the lock. If another thread holds the lock, the current thread will wait.

If the lock cannot be acquired (for instance, if another thread panics while holding the lock), the program 
will panic, which is why we call unwrap here. In real-world code, you'd handle errors more gracefully.

- when `num` goes out of scope, the lock is automatically released. This is handled by the `MutexGuard` type
  which ensures the lock is released as soon as we're done with the data.

Ex2: Sharing a Mutex between multiple threads.

The below code will not compile and is for learning purpose:

```rust 
use std::sync::Mutex;
use std::thread;

fn main() {
    let counter = Mutex::new(0);  // Mutex guards a counter value
    let mut handles = vec![];

    // Create 10 threads, each of which increments a counter stored in a mutex.
    for _ in 0..10 { 
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  // Acquire the lock
            *num += 1;  // Increment the counter
        });
        handles.push(handle);  // Collect thread handles
    }

    for handle in handles {
        handle.join().unwrap();  // Wait for all threads to finish
    }

    // Print the result
    println!("Result: {}", *counter.lock().unwrap());
}
```
This will cause compile error: 
   Move occured because `counter` has a type `Mutex<i32>` which does not  implement copy trait. 
   Keyword  `move` value moved into closure, in previous itteration of loop.

- Ownership Move: `move || ` moves ownership of `counter` into the closure. And after which its not
  available to main thread. Causing error occurs when we try to use `counter` in main thread after the
  thread have been spawned.

To Fix: We need shared Ownership of Mutex across multiple threads. ( Rust `ARC` atomic reference counter )
allows multiple threads to safely share ownership of the same data.

```rust 
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));  // Wrap the mutex in an Arc
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);  // Clone the Arc to share ownership
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();  // Acquire the lock
            *num += 1;  // Increment the counter
        });
        handles.push(handle);  // Collect thread handles
    }

    for handle in handles {
        handle.join().unwrap();  // Wait for all threads to finish
    }

    // Print the result
    println!("Result: {}", *counter.lock().unwrap());
}
```
- `Arc` (Atomic Reference Counting): We use Arc<Mutex<i32>> to allow multiple threads to share ownership of 
  the mutex safely.

    * Arc is used to enable shared ownership across threads, unlike Mutex alone, which requires exclusive 
      ownership.

    * `Arc::clone(&counter)` is used to create additional references to the same Mutex.

- Locking the Mutex: We use `counter.lock().unwrap()` to acquire the lock in each thread.

Shared-state concurrency with `mutexes` provides a way for multiple threads to safely share and modify data.
Rust’s ownership system and type system ensure that mutexes are used correctly, preventing common 
concurrency errors like race conditions or forgetting to release locks.


#### Multiple Ownership with Multiple Threads:

How Reference Counting works in Rust and how to manage shared ownership in a thread-safe manner using types
like `Rc<T>` and `Arc<T>`.

- Multiple ownership is a concept where multiple entities ( in this case threads ) can have access to the
  same piece of data. This requires data to be managed carefully to ensure its not corrupted or accessed
  Simultaneously in conflicting way. Rust Ownership does not allow this multiple ownership of same data, but
  smart pointers like `Rc<T>` and `Arc<T>` enable functionality by managing the reference count of the data.

    * `Rc<T>` Smartpointer that tracks number of references to a value and deallocates the value once all
      references are dropped. ( but this is not thread safe ==> it can not be shared between threads. )

    * `Arc<T>` Atomic reference counting: Thread safe, ensures safety by updating reference count is updated
      atomically. This allows it to be safely shared across multiple threads. 
      An example is above code sample


### Similarities Between RefCell<T>/Rc<T> and Mutex<T>/Arc<T>

In Rust, interior mutability allows for the mutation of data even when the data itself is considered 
immutable. This concept is crucial when working with types like `RefCell<T>` and `Rc<T>` in single-threaded 
contexts, or `Mutex<T>` and `Arc<T>` in multi-threaded contexts.

Let's explore the similarities between these pairs and their respective use cases.

#### 1. Interior Mutability:

Both `RefCell<T>` (with `Rc<T>`) and `Mutex<T>` (with `Arc<T>`) enable **interior mutability**, allowing
you to mutate data even when the outer container is immutable.

* `RefCell<T>` with `Rc<T>`:

  - `RefCell<T>` allows you to mutate data inside an immutable reference. It does this through 
     **dynamic borrowing** the reference is checked at runtime, and it enforces borrowing rules (one mutable
     reference or multiple immutable references).

  - `Rc<T>` is used for **shared ownership** in single-threaded contexts, where the reference count is 
    tracked, and multiple parts of your code can own the data.

* **`Mutex<T>` with `Arc<T>`:**

  * Similarly, `Mutex<T>` allows **mutating** the data it guards, even when the `Mutex` itself is immutable.
    This is done by **locking** the mutex and obtaining a mutable reference to the underlying data.

  * `Arc<T>` is a thread-safe, **atomic reference counter** for shared ownership across multiple threads. 
    It is similar to `Rc<T>`, but it works in concurrent situations by ensuring thread-safety with atomic 
    operations.

In both pairs, **interior mutability** allows data that is normally immutable (like `Rc<T>` or `Arc<T>`) to 
be modified. This pattern is essential for situations where you need **shared ownership** and **mutability**.

#### 2. Shared Ownership:

Both `Rc<T>`/`RefCell<T>` and `Arc<T>`/`Mutex<T>` allow **multiple ownership** of the same data:

* `Rc<T>` (Reference Counted) and `RefCell<T>`:

  - `Rc<T>` allows multiple owners of the same data in a **single-threaded** environment. 
    Each `Rc<T>` clone increments the reference count, and the data is deallocated once the reference count 
    drops to zero.

  - When combined with `RefCell<T>`, you get shared ownership of data, along with the ability to mutate the
    data at runtime, provided you follow Rust’s borrowing rules.

* `Arc<T>` (Atomic Reference Counted) and `Mutex<T>`:

  - `Arc<T>` is the **thread-safe** version of `Rc<T>`, and it can be used to share ownership of data across 
    **multiple threads**.

  - When combined with `Mutex<T>`, you can mutate the data safely across threads, where the `Mutex` ensures 
    that only one thread can access the data at a time.

In both cases, **shared ownership** is achieved, but the concurrency aspect is addressed differently: 
`Rc<T>` is for single-threaded programs, while `Arc<T>` is for multi-threaded programs.

#### 3. Risks of Logic Errors:

While `RefCell<T>` and `Rc<T>` provide flexibility and power, they come with potential pitfalls:

* `RefCell<T>` and `Rc<T>` Risks:

  - `RefCell<T>` can lead to runtime borrowing errors if you violate Rust's borrowing rules, like trying to
    borrow data mutably while it’s already borrowed immutably.

  - `Rc<T>` can lead to **reference cycles**, where two or more `Rc<T>` instances reference each other,
    preventing the reference count from ever reaching zero and causing memory leaks.

* `Mutex<T>` and `Arc<T>` Risks:

  - `Mutex<T>` introduces the risk of **deadlocks**. This happens when two threads lock different `Mutex<T>` 
    instances and then try to lock the other thread’s mutex, resulting in an infinite wait.

  - `Arc<T>` doesn’t directly introduce new risks, but when combined with `Mutex<T>`, the complexity of 
    managing shared ownership and lock acquisition increases, making deadlocks possible.

#### 4. Synchronization and Borrowing Mechanisms:

* `RefCell<T>` and `Rc<T>`:

  - `RefCell<T>` works by enforcing borrowing rules **dynamically** at runtime, ensuring that mutable and
    immutable borrows don’t coexist at the same time. This is **not thread-safe** because `RefCell<T>` is 
    designed for use in a single-threaded context.

* `Mutex<T>` and `Arc<T>`:

  - `Mutex<T>`, on the other hand, ensures **exclusive access** to the data it guards by requiring threads 
    to lock and unlock the mutex. This mechanism synchronizes access to the data but introduces potential 
    risks, such as deadlocks and contention between threads.

---

#### Summary of Similarities:

| Feature                   | `RefCell<T>` + `Rc<T>`                                                      | `Mutex<T>` + `Arc<T>`                          |
| ------------------------- | --------------------------------------------------------------------------- | ---------------------------------------------- |
| **Interior Mutability**   | Allows mutation of data inside immutable references.                        | Allows mutation of data through locks.         |
| **Shared Ownership**      | Multiple owners via `Rc<T>`.                                                | Multiple owners via `Arc<T>` (thread-safe).    |
| **Concurrency**           | Not thread-safe.                                                            | Thread-safe; used in multi-threaded contexts.  |
| **Risks of Logic Errors** | Potential runtime borrowing errors (`RefCell`) and reference cycles (`Rc`). | Risk of deadlocks and incorrect lock ordering. |
| **Synchronization**       | Enforces borrowing rules at runtime.                                        | Synchronizes access to data using locks.       |

#### Conclusion:

* `RefCell<T>` and `Rc<T>` offer flexibility for **interior mutability** and **shared ownership** in 
  **single-threaded** contexts, but they require careful management to avoid runtime borrowing errors and 
  reference cycles.

* `Mutex<T>` and `Arc<T>` provide the same patterns in **multi-threaded** contexts, but introduce new risks 
  such as **deadlocks** and require careful handling of locks to ensure thread safety.

In both cases, the key takeaway is that **shared ownership** and **interior mutability** are powerful tools, 
but they come with the responsibility of handling concurrency and resource management carefully.


### Extensible Concurrency with the `Send` and `Sync` Traits

In Rust concurrency is mainly handled via *Type System and the Standard library*, but language also embeds
key traits that ensure thread safety and proper resource management in concurrent situations. 

These traits `Send` and `Sync` are essential tools for writing concurrent program that avoid issues like
data races and unsafe memory access. 

Working of `Send` and `Sync` traits:

#### 1. `Send` Trait: Ownership Transfer Between Threads

The `Send` trait indicates that a type’s ownership can be safely transferred between threads. 
When you move a value from one thread to another, the type must implement `Send` to ensure that the transfer 
is safe and the data won’t be accessed simultaneously in a way that could lead to data races.

* Types that implement `Send`:

  Almost all Rust types implement `Send`, including primitive types like integers, floats, and `Vec<T>`. 
  However, types that involve **shared ownership** or **mutable references** are generally not `Send` 
  because allowing ownership transfer could introduce safety issues.

* Why doesn’t `Rc<T>` implement `Send`?
  `Rc<T>`, or Reference Counted pointers, is **not thread-safe**. 
  If you cloned an `Rc<T>` value and sent the clones to multiple threads, each thread could attempt to 
  modify the reference count, leading to a **data race**. This is why `Rc<T>` does **not** implement `Send`.

* **Safe alternatives:**
  The **atomic reference-counted smart pointer** `Arc<T>` is the thread-safe counterpart to `Rc<T>`, and 
  it **does implement `Send`**. 
  This allows `Arc<T>` values to be sent between threads safely, as it ensures that the reference count is 
  updated in a thread-safe manner using **atomic operations**.

* **Automatically implementing `Send`:**
  If a type is composed of other types that implement `Send`, it will automatically implement `Send` as well. 
  For example, if you have a struct with all `Send` types (like integers or `Vec<T>`), the struct itself 
  will implement `Send` as long as all its fields are `Send`.

---

#### 2. `Sync` Trait: Safe Access from Multiple Threads

The `Sync` trait is used to ensure that a type can be safely referenced from multiple threads. 
A type implements `Sync` if its immutable reference (`&T`) can be safely sent across threads. 
In other words, `Sync` ensures that a value can be accessed simultaneously from multiple threads without 
causing data races or unsafe behaviors.

* What makes `Sync` different from `Send`?
  While `Send` allows ownership transfer between threads, `Sync` focuses on **shared access**. 
  If a type is `Sync`, multiple threads can access it **simultaneously** without violating safety guarantees,
  provided they only hold immutable references (`&T`).

* Types that implement `Sync`:
  Similar to `Send`, most primitive types (such as integers, floats, and arrays) implement `Sync` because 
  immutable references to these types are safe to share across threads. 
  Types composed entirely of `Sync` types are also automatically `Sync`.

* Why doesn’t `Rc<T>` implement `Sync`?
  `Rc<T>` doesn't implement `Sync` because it doesn’t guarantee safe access across threads. 
  If multiple threads tried to access the same `Rc<T>` instance simultaneously, they could potentially 
  modify the reference count at the same time, leading to inconsistent or unsafe behavior. 
  This is the same reason why `Rc<T>` doesn’t implement `Send`.

* Example of `Sync` in action:
  `Mutex<T>` is an example of a type that implements `Sync`. 
  Although it provides **exclusive access** to data by locking it, the `Mutex<T>` itself is **safe to 
  share** between threads because it ensures that only one thread can access the data at a time.

---

#### 3. Implementing `Send` and `Sync` Manually

While Rust automatically implements `Send` and `Sync` for most types based on their composition, you can 
manually implement these traits for your custom types if necessary. 

However, doing so involves **unsafe Rust** because it requires careful handling of concurrency to ensure 
thread safety.

* Why is manual implementation unsafe?
  Implementing `Send` and `Sync` manually requires the programmer to ensure that the type can be safely 
  shared or transferred across threads, which is a delicate task. You must guarantee that:

  - The type can be sent between threads without violating Rust’s ownership and borrowing rules.
  - The type can be safely accessed from multiple threads without causing data races.

  These guarantees are not automatically provided by the type system, so the implementation of these traits 
  in unsafe Rust code must be done with great care.

* Rustonomicon:
  The **Rustonomicon** (Rust’s official guide to writing unsafe code) offers valuable insights into how to 
  safely implement `Send` and `Sync` for custom types. 
  It covers how to manage raw pointers, thread synchronization, and how to uphold safety guarantees in 
  unsafe code.

---

### 4. Why `Send` and `Sync` Matter

Rust’s approach to concurrency via these marker traits is powerful because it provides compile-time 
guarantees that your concurrent code will be safe. 

By relying on the type system to enforce these rules, Rust avoids the **common pitfalls** of concurrent 
programming, such as:

- **Data races**, where multiple threads access and modify the same data simultaneously without proper
  synchronization.
- **Dangling references** or **use-after-free errors**, where threads access memory that has been 
  deallocated.
- **Memory safety issues** related to manual memory management.

With these traits, the Rust compiler ensures that, if your code compiles, it will run safely on multiple 
threads. 

You can confidently build concurrent applications that are efficient, safe, and less prone to the subtle 
bugs that plague other languages with manual memory management and unsafe concurrency models.

---

### **Summary of Key Points**

| Trait                     | Purpose                                                        | Example Types                                    | Why It Matters                                                        |
| ------------------------- | -------------------------------------------------------------- | ------------------------------------------------ | --------------------------------------------------------------------- |
| **`Send`**                | Allows ownership of a value to be transferred between threads. | `Arc<T>`, `Mutex<T>`, primitive types like `i32` | Ensures that transferring ownership across threads is safe.           |
| **`Sync`**                | Allows immutable references to be shared across threads.       | `Mutex<T>`, `Arc<T>`, primitive types like `i32` | Ensures that shared references across threads are safe.               |
| **Manual Implementation** | Implementing `Send` and `Sync` manually involves unsafe code.  | Custom types with concurrency needs.             | Provides flexibility but requires careful attention to thread safety. |

### **Conclusion:**

The **`Send`** and **`Sync`** traits form the backbone of Rust’s **concurrency model** by providing **compile-time checks** for safe data access and ownership transfer between threads. These traits allow you to build concurrent applications without worrying about low-level issues like data races and memory safety, offering a robust and scalable way to manage concurrency. Whether you're using standard library types or building your own, understanding and leveraging these traits will ensure that your programs are thread-safe and efficient.
