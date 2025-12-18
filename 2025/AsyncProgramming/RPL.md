# Rust Asynchronous programming:


## Concurrency vs Parallelism :

### Concurrency: 
---
Structuring a program to handle multiple tasks so they make progress over overlapping time periods, even if
only one task runs at any given instance.

- Managing many things at once ( Interleaving tasks )
- You can have Concurrency on a single core by rapidly switching between tasks.
- Concurrency is achieved via context switching on a single CPU.
- Programs become responsive, proper resource utilization, proper management of I/O bound tasks.

Ex: WebServer handling multiple user requests by switching between them rapidly

### Parallelism: 
---
Execution of multiple tasks or parts of the task simultaneously, at the exact same moment in time. 
- Is about doing many things at the exact same time ( Simultaneous execution )
- True Parallelism requires multi-core/processors to run tasks truly simultaneously for speed. 
- Programs improve in speed, give higher throughput for CPU-intensive tasks. 
Ex: Analyzing different sections of a large dataset using separated processor cores.

Parallelism is a form of Concurrency, but Concurrency doesn't require parallelism.
Concurrent system can use parallelism if multiple cores are available, but it can still be concurrent on
just one core. Or using tools such as threads, processes and async, the computer can pause one activity and
switch to others before eventually cycling back to the first activity again.

Running async code in Rust usually happend concurrently. ( depending on the HW the OS and the async runtime
we are using that concurrency may also use parallelism under the hood )

## Futures and the Async:

Future: Or **async future** is also known as a **promise** or **task** is an object that acts as a 
placeholder for the result of an operation that is still in progress. 
It represents a value that will be available at some point in the future, allowing the program to continue
executing other tasks without blocking the main thread while waiting for a long-running operations ( like
performing some calculation or wait for file I/O) to complete. 

- Asynchronous Programming: 
    Programming model that allows a program to initiate a time-consuming task and then immediately switch 
    to other work. 
    This improves efficiency and responsiveness, especially for I/O-bound operations, by making use of 
    "idle time" where the program would otherwise be waiting.

- Non-Blocking: 
    The core benefit of **futures** is that they are **non-blocking**. When an **async function** is called,
    it returns an uncompleted future and yields control back to an **event loop** or **executor**, which can
    then manage other tasks.

- State: 
    A future typically exists in two states:
    1. Uncompleted (Pending): The asynchronous operation is still running.
    2. Completed (Resolved/Ready): Operation has finished, either with a successful result or an error.

- How they Work:
    The most common way to interact with **futures** is through the **async** and **await** keywords, which 
    are syntactic sugar that makes asynchronous code look like synchronous, linear code.
    
    1. **async** func: 
        Declaring a function with the **async** keyword indicates that it can perform asynchronous 
        operations and will automatically return a future.

    2. **await** keyword: 
        **await** keyword is used inside an **async** function to pause the execution of that specific 
        function until the awaited future completes. 
        While the function is paused, control is returned to the **async** runtime, which runs other pending
        tasks, thus preventing the entire program from freezing.

To short **futures** are a fundamental abstraction for managing concurrent, non-blocking operations 
efficiently, leading to more responsive and scalable applications.

Rust: Uses the Future trait and async/await, managed by an executor like Tokio, smol, async-std ...

## Rust Futures and Async 

Key elements of Rust async programming are centered around *futures* and Rust's `async` and `await`
keywords.

Rust provides a `Future` trait as a building block for implementing various asynchronous operations across
different data structures while maintaining common interface.

=> So in Rust futures are types that implement a `Futures` trait. Each future stores its own
state/information tracking the progress of the asynchronous computation and defining what it means for the 
future to be "ready"

Futures: 
- The futures are types that implement the **Future trait**. Each future encapsulates its own state
  regarding its progress and the definition of what it means for the future to be "ready".

- The `async` keyword can be applied to blocks and functions, ( which means these blocks can be paused,
  resumed ). 

- Inside an async context, the `await` keyword is utilized to pause execution until the future becomes
  ready.

- The act of checking a future's readiness if termed as **polling**
  i.e checking for readiness: ( Whether a `Future` is able to produce its final result right now )
  When future is polled it returns 2 values:
  `Poll::Ready(output)`     // future is completed 
  `Poll::Pending`           // future cannot complete yet.
    
- A future does not run on its own. It makes progress when: 
    - An executor polls it. 
    - The future check's its internal state 
    - It reports whether it is ready or not ready.
      If not ready the future:
      * Stores its state 
      * Registers a *waker* so it can be polled again later 
      * Returns `Poll::Pending`
        
How to use:

- You can apply `async` keyword to *blocks* or *functions* to specify that they can be interrupted and
  resumed.

- Within an `async` block of `async` function, you can use the `await` keyword to await a *future*, that is
  wait for it to become ready. 

- Any point where you await a future within an async block or function is potential spot for that async
  block or function to pause and resume.

- The process of checking with a future to see if its value is available yet is called **polling**.

When writing async Rust, we use `async` and `await` keyword most of the time. Rust compiles them into
equivalent code using `Future` trait much as it compiles `for` loops into equivalent `Iterator` trait. 

As Rust provides the `Future` trait, though, you can also implement it for your own data types when there is
a need to. 

This all sounds abstract and the better way to understand is to write our own async programs:

NOTE: TRPL chapter used a `trpl` crate which consists of :
    - re-exports all the types, traits and functions you need primarily from the `futures` and `tokio`
      crates.

## First async program

The `futures` crate is the official home for experimentation for async code and its actually where  the
`Future` trait was originally designed. 

`Tokio` is the most widely used async runtime in Rust today, especially for web applications. There are
other great runtimes out there and are more suitable for different purposes. 

The `trpl` crate uses `Tokio` crate as its most widely used and tested. 

```text 
    $ cargo new hello-async
    $ cd hello-async
    $ cargo add trpl
```

Example 1: 
   Simple web scraper that fetches two URLs concurrently and returns the result of whichever finishes first.

Step1: Define a function `page_title(url)`  which takes in a URL , makes a HTTP request and returns the test
of the <title> element from the HTML response. 

```rust 
async fn page_title(url: &str) -> Option<String> {
    let response = trpl::get(url).await;
    let response_text = response.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}
```
- `async` keyword marks the function as asynchronous.
- `trpl::get` function is called to fetch the URL and result is awaited. 
- Response text is then parsed into Html page allowing for richer interactions with the HTML structure.
- The function returns a Option<String> representing the title if its available.

Changing Async Call: The above function can be streamlined by changing the calls together:

```rust 
async fn page_title(url: &str) -> Optoon<String> {
    let response_text = trpl::get(url).await.text().await;

    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
    
}
```

### Execute Async Function with a Runtime:

To execute the `async` function, the `main` function cannot be marked as `async` directly. 
Instead, Rust requires a runtime to manage `async` code execution. 
The `trpl::block_on` function is used to run an `async` block until completion:

```rust 
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let url = &args[1];
        match page_title(url).await {
            Some(title) => println!("The title for {url} was {title}"),
            None => println!("{url} had no title"),
        }
    })
}
```

### Runtime:
- Rust ecosystem does not bundle a runtime; instead various runtimes exist, each tailored to different use
  cases. 
- The **block_on** function initializes a runtime to execute the future passed to it. 
- This function is crucial for running async code, as it blocks the current thread until the future
  completes. 

### Racing Two URLs Concurrently:

To extend the functionality, the program can be modified to race two URLs against each other. 
This can be done by calling the `page_title` for both URLS and using `trpl::select` to detect which future
completes first:

```rust
fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::block_on(async {
        let title_fut_1 = page_title(&args[1]);
        let title_fut_2 = page_title(&args[2]);

        let (url, maybe_title) =
            match trpl::select(title_fut_1, title_fut_2).await {
                Either::Left(left) => left,
                Either::Right(right) => right,
            };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title was: '{title}'"),
            None => println!("It had no title."),
        }
    })
}
```
- The future of two URLs are created, but they are not executed ( as futures are lazy ) until awaited. 
- `trpl::select` function used to`await` both futures, returning the result of the one that finishes first.
- The result is wrapped in the `Either` type, which indicates which future completed and provides its
  output.

```rust 
pub enum Either<A,B> {
    Left(A),
    Right(B),
}
```
SumUp:
- use of future, async, await, runtime for executing async code. 

## Applying Concurrency with Async:

Using the concurrency challenges addressed in Threads, High light the differences between thread based and
async based concurrencies, Including API similarities behavioral distinctions and performance
characteristics.

### Overview of Async Vs Threads: 

- APIs for concurrency using async may resemble those for threads, they often exhibit different behaviors
  and performance metrics. 

#### Creating a New Task with `spawn_task`

The ex mirrors the thread-based approach of counting up using threads, but utilizes async programming 
instead. 

The `trpl` crate provides a `spawn_task` function, akin to the `thread::spawn` API, and an async version
of the sleep function (`trpl::sleep`). 

Example Code: Counting with Async

```rust 
extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }
    });
}
```

In this code, the main function is wrapped with `trpl::block_on`, allowing the top-level function to be
asynchronous. 
Two loops are executed: 
one within the `spawn_task` and the other directly in the `main` async block. 

The output may vary in order due to the concurrent nature of the tasks.

*Ensuring Task Completion*

Initially, the async task spawned by `spawn_task` may terminate prematurely when the main function 
completes. 

To ensure that all tasks finish, a `join` handle can be used to `await` the completion of the first task.

Example Code: Awaiting Task Completion

```rust 
extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let handle = trpl::spawn_task(async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        for i in 1..5 {
            println!("hi number {i} from the second task!");
            trpl::sleep(Duration::from_millis(500)).await;
        }

        handle.await.unwrap();
    });
}
```

In this updated version, the program runs until both loops finish, 
this demons that async tasks can be awaited similarly to threads.


**Using `trpl::join` for Multiple Futures**

An important distinction in async programming: tasks do not necessarily require spawning new OS threads.
Instead, they can be executed concurrently within the same async block. 
The `trpl::join` function allows multiple futures to be awaited together, producing a single future 
that resolves when all input futures complete.


Example Code: Joining Futures
```rust 
extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
}
```

This demo's that using `trpl::join` allows for fair execution of both futures, resulting in a predictable
output order, unlike the thread-based approach.

**Message Passing Between Tasks**
**Sending Data with Async Channels**

How to share data between futures using message passing. 

The `trpl::channel` function creates an `async` channel, allowing for communication between tasks without 
spawning separate threads.

Example Code: Basic Message Passing
```rust
extern crate trpl; // required for mdbook test

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let val = String::from("hi");
        tx.send(val).unwrap();

        let received = rx.recv().await.unwrap();
        println!("received '{received}'");
    });
}
```
- In example, a message is sent and received within a single async block. 
- The recv method does not block but instead returns a future that must be awaited.

**Sending Multiple Messages**

To illustrate sending multiple messages, we modify the previous example to include a loop that sends 
messages with delays in between.

Example Code: Sending Multiple Messages
```rust 
extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("future"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            trpl::sleep(Duration::from_millis(500)).await;
        }

        while let Some(value) = rx.recv().await {
            println!("received '{value}'");
        }
    });
}
```
- The messages are sent in a loop, but they arrive all at once after the total delay, demonstrating that
  everything in an async block executes linearly.

**Achieving Concurrency with Separate Async Blocks**

To achieve the desired behavior where messages are received at intervals, we separate the sending and 
receiving logic into distinct async blocks.

Example Code: Separate Async Blocks
```rust 
extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}
```
This structure allows the sending and receiving tasks to execute concurrently, resulting in messages being 
printed at the correct intervals.

**Moving Ownership into Async Blocks**

To ensure the program exits gracefully, we can move ownership of the sender into the async block, allowing 
it to be dropped when the block completes.

Example Code: Moving Ownership
```rust
extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        trpl::join(tx_fut, rx_fut).await;
    });
}
```
- The `async move` keyword is used to ensure that the sender is dropped after sending the last message,
  allowing the program to terminate correctly.

**Joining Multiple Producers with join! Macro**

The async channel supports multiple producers, and we can clone the sender to allow multiple async blocks 
to send messages.

Example Code: Multiple Producers
```rust 
extern crate trpl; // required for mdbook test

use std::time::Duration;

fn main() {
    trpl::block_on(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let rx_fut = async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        };

        let tx_fut = async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        };

        trpl::join!(tx1_fut, tx_fut, rx_fut);
    });
}
```

In this example, two producers send messages at different intervals, demonstrating the flexibility of async 
programming in Rust. 
The messages are received in the order they are sent, showcasing the efficiency of the async channel.

Summing Up:
- The differences between threads and futures, the mechanics of creating and managing tasks, and 
  the intricacies of message passing between tasks. 

- The above concepts should help developer to leverate async programming to build concurrent applications in
  Rust.

