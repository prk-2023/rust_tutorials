# Concurrency

Rust’s concurrency model is famous for allowing **safe, data-race–free parallelism** — all enforced **at compile time**.
Let’s explore how it works and build up from simple threads to shared-state concurrency.

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

Rust’s goal is **“fearless concurrency”** — letting you write concurrent code *confidently*, knowing the compiler enforces safety.

---

## 2.0 Spawning Threads

Rust’s standard library provides the `std::thread` module.
You can create a new thread with `thread::spawn`.

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
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
🧵 From spawned thread: 2
...
```

✅ Threads run concurrently.
⚠️ But the program might end before the spawned thread finishes — we’ll fix that next.

---

## 3.0 Waiting for Threads: `join()`

The `JoinHandle` returned by `thread::spawn` can be used to **wait** for a thread to finish.

```rust
use std::thread;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("🧵 Working... {i}");
        }
    });

    handle.join().unwrap(); // Wait for thread to finish
    println!("✅ Thread completed!");
}
```

✅ `join()` blocks until the spawned thread finishes.
✅ If the thread panics, `join()` returns an error.

---

## 4.0 Ownership and `move` in Threads

Threads **outlive their creation scope**, so you must **move** ownership of any data into the closure.

```rust
use std::thread;

fn main() {
    let numbers = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        for n in &numbers {
            println!("Thread has number: {n}");
        }
    });

    handle.join().unwrap();
}
```

✅ The `move` keyword transfers ownership of `numbers` into the thread.
❌ Without `move`, the closure would try to borrow `numbers`, which would end when `main`’s scope ends — unsafe!

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

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..10 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", *counter.lock().unwrap());
}
```

Output:

```
Final count: 50
```

### What happened

* Each thread gets a clone of the same `Arc<Mutex<i32>>`.
* `Mutex` ensures only one thread modifies the counter at a time.
* When the lock (`num`) goes out of scope, it’s automatically released.

✅ Safe concurrent mutation.
✅ No data races.
✅ Compiler enforces proper locking and ownership.

---

## 7.0 How `Arc<Mutex<T>>` Works Together

| Type            | Purpose                                     |
| --------------- | ------------------------------------------- |
| `Arc<T>`        | Allows **shared ownership** between threads |
| `Mutex<T>`      | Allows **mutability** with synchronization  |
| `Arc<Mutex<T>>` | Shared **and mutable** data across threads  |

🧠 Analogy:

> `Rc<RefCell<T>>` → single-threaded shared mutability
> `Arc<Mutex<T>>` → multi-threaded shared mutability

---

## 8.0 Common Mutex Pitfalls

### Deadlock example:

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let a = Arc::new(Mutex::new(1));
    let b = Arc::new(Mutex::new(2));

    let a1 = Arc::clone(&a);
    let b1 = Arc::clone(&b);

    let t1 = thread::spawn(move || {
        let _lock_a = a1.lock().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
        let _lock_b = b1.lock().unwrap();
        println!("Thread 1 finished");
    });

    let a2 = Arc::clone(&a);
    let b2 = Arc::clone(&b);

    let t2 = thread::spawn(move || {
        let _lock_b = b2.lock().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(100));
        let _lock_a = a2.lock().unwrap();
        println!("Thread 2 finished");
    });

    t1.join().unwrap();
    t2.join().unwrap();
}
```

⚠️ This can deadlock because threads lock resources in different orders (`a → b` vs `b → a`).

✅ Always lock shared resources in a consistent order.

---

## 9.0  Message Passing with Channels

Rust’s second concurrency model is **message passing**, using `std::sync::mpsc` (multi-producer, single-consumer).

```rust
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let vals = vec!["one", "two", "three"];
        for v in vals {
            tx.send(v).unwrap();
            thread::sleep(Duration::from_millis(200));
        }
    });

    for received in rx {
        println!("Got: {received}");
    }
}
```

Output:

```
Got: one
Got: two
Got: three
```

✅ Threads communicate by transferring ownership through channels — no shared memory needed.
✅ Channels guarantee *safe data transfer* between threads.

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

They serve the same conceptual purposes — but `Arc` and `Mutex` add **atomic operations** and **thread safety**.

---

## Example: Safe Parallel Computation

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let data = Arc::new(Mutex::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];

    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let mut vec = data.lock().unwrap();
            vec[i] *= 2;
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {:?}", *data.lock().unwrap());
}
```

Output:

```
Result: [2, 4, 6, 8, 10]
```

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

Would you like me to continue with the **next logical part — Chapter 17: “Object-Oriented Programming Features of Rust”**, which connects everything you’ve learned about ownership, traits, and smart pointers to object-oriented design patterns (like polymorphism and trait objects)?
