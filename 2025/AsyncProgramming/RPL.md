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

#### - Creating a New Task with `spawn_task`

In the example below we try to mimic the thread-based approach of counting up using threads, but utilizes 
async programming instead. 

The `trpl` crate provides a `spawn_task` function, akin to the `thread::spawn` API, and an async version
of the sleep function (`trpl::sleep`). 

Example Code: Counting with Async

```rust 
//rust till 2015 edition below line is required
//extern crate trpl; // required for mdbook test
// rust 2018 onwards
use trpl; // 2018 allows to find crates from Cargo.toml  and link/make available. 

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

#### - Ensuring Task Completion

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

#### - Message Passing Between Tasks (**Sending Data with Async Channels**)

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

#### - Sending Multiple Messages

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

#### - Achieving Concurrency with Separate Async Blocks

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

#### - Moving Ownership into Async Blocks

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

#### - Joining Multiple Producers with join! Macro

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

- The above concepts should help developer to leverage async programming to build concurrent applications in
  Rust.

## Working with many futures:

### Yielding Control to the Runtime: 

Yielding control back to runtime at await, preventing one future from blocking others.

How Rust runtime manages control flow during asynchronous operations: 
At each `await` point, the runtime can pause the current task and switch to another if the awaited future is
not ready. 

i.e: Rust only returns control to the runtime at these `await` points, meaning that any operations 
performed between them execute synchronously. This behavior can lead to a situation where one future may 
block others from progressing, a phenomenon known as "starvation."

#### - Starvation in Async Tasks

Blocking Operations

If an asynchronous block performs extensive work without yielding control back to the runtime, it can 
prevent other futures from making progress. This is particularly problematic in scenarios involving 
long-running tasks or expensive computations. 

Ex: Simulating Long-Running Operations

```rust 
use trpl;
use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        // We will call `slow` here later
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
```
- `std::thread::sleep` is used to simulate a slow operation. This function blocks the current thread,
  mimicking real-world long-running tasks. 


CPU-Bound work in pair of futures:
```rust 
use trpl;

use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            slow("a", 10);
            slow("a", 20);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            slow("b", 10);
            slow("b", 15);
            slow("b", 350);
            trpl::sleep(Duration::from_millis(50)).await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
```
- `slow` function blocks the current thread for a specified duration, representing a long-running or
  blocking operation.

When the code is executed, it produces output indicating that future a completes all its blocking operations
before future b starts, highlighting the starvation issue. 

The output confirms that the futures do not interleave as expected, emphasizing the need for `await` points
to allow concurrent execution.

#### - Introducing Await Points for Concurrency

Adding Await Points

The text proposes a solution to the starvation problem by introducing await points using `trpl::sleep`. 
This adjustment allows the futures to yield control back to the runtime between slow operations, enabling 
interleaved execution. 

The modified code demonstrates this interleaving, resulting in a more balanced execution of both futures.
```rust 
use trpl;
use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        let one_ms = Duration::from_millis(1);

        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::sleep(one_ms).await;
            slow("a", 10);
            trpl::sleep(one_ms).await;
            slow("a", 20);
            trpl::sleep(one_ms).await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::sleep(one_ms).await;
            slow("b", 10);
            trpl::sleep(one_ms).await;
            slow("b", 15);
            trpl::sleep(one_ms).await;
            slow("b", 350);
            trpl::sleep(one_ms).await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
```

#### - Using `yield_now`

Yielding Control with `yield_now`

`trpl::sleep` introduces unnecessary delays, the section suggests using `trpl::yield_now` to yield control
without the overhead of sleeping. 
The updated code below replaces `trpl::sleep` with `trpl::yield_now`, allowing for faster execution while 
still enabling the runtime to manage task switching effectively.

```rust 
use std::{thread, time::Duration};

fn main() {
    trpl::block_on(async {
        let a = async {
            println!("'a' started.");
            slow("a", 30);
            trpl::yield_now().await;
            slow("a", 10);
            trpl::yield_now().await;
            slow("a", 20);
            trpl::yield_now().await;
            println!("'a' finished.");
        };

        let b = async {
            println!("'b' started.");
            slow("b", 75);
            trpl::yield_now().await;
            slow("b", 10);
            trpl::yield_now().await;
            slow("b", 15);
            trpl::yield_now().await;
            slow("b", 350);
            trpl::yield_now().await;
            println!("'b' finished.");
        };

        trpl::select(a, b).await;
    });
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}
```
This approach clarifies the intent and can be more efficient than using sleep, as it avoids the minimum 
sleep duration imposed by timers.

#### - Cooperative Multitasking and Performance Considerations

Above examples show that Rust Asynchronous programming can be beneficial even for CPU-bound tasks, as it 
allows for cooperative multitasking. Each future is responsible for yielding control to avoid blocking other
tasks. 
However, excessive yielding may lead to performance degradation, and developers should measure performance 
to identify bottlenecks.

### Building Custom Async Abstractions

Creating a Timeout Function

Building custom asynchronous abstractions, specifically a timeout function that can be used to *limit the 
execution time of a future*. 

The proposed API for the timeout function is outlined, requiring it to be asynchronous and accept a future 
and a maximum duration.

Implementing the Timeout Function

```rust  
use std::time::Duration;

use trpl::Either;

// --snip--

fn main() {
    trpl::block_on(async {
        let slow = async {
            trpl::sleep(Duration::from_secs(5)).await;
            "Finally finished"
        };

        match timeout(slow, Duration::from_secs(2)).await {
            Ok(message) => println!("Succeeded with '{message}'"),
            Err(duration) => {
                println!("Failed after {} seconds", duration.as_secs())
            }
        }
    });
}

async fn timeout<F: Future>(
    future_to_try: F,
    max_time: Duration,
) -> Result<F::Output, Duration> {
    match trpl::select(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}
```
The timeout function checks the result of `trpl::select` and returns either the successful output or an 
error indicating the timeout duration.

The function uses `trpl::select` to race the provided future against a timer created with `trpl::sleep`. 
Depending on which completes first, the function returns either the result of the future or an error 
indicating that the timeout elapsed.

When the timeout function is executed with a slow future, the output reflects the failure to complete within 
the specified duration, demonstrating the function's effectiveness.


## Streams: Futures in Sequence:

### Introduction to Asynchronous Streams

The use of the receiver for asynchronous channels, which allows for the production of a sequence of items 
over time through the `async recv` method. 
This method exemplifies a broader programming pattern known as `streams`, which can represent various data 
flows, such as items in a queue, incremental data from a filesystem, or data received over a network.

#### - Understanding Streams

Streams are recognized as futures, enabling their combination with other future types to create complex 
workflows. This allows developers to batch events, set timeouts on long-running operations, or throttle user
interface events to optimize performance and resource usage. 

A comparison between streams and iterators, highlighting two key differences:

Synchronous vs. Asynchronous: 
- Iterators operate synchronously, while the channel receiver operates asynchronously.
- API Differences: Iterators utilize a synchronous next method, whereas the `trpl::Receiver` stream employs
  an asynchronous `recv` method.

Despite these differences, both APIs share a conceptual similarity, as `streams` can be viewed as an
asynchronous form of iteration.

#### - Creating a Stream from an Iterator

We will see how any iterator can be transformed into a stream. 
This transformation allows the use of the next method in an asynchronous context, which is demonstrated in 
the provided code example:

Code Example: Creating a Stream
```rust 
extern crate trpl; // required for mdbook test

fn main() {
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
```
Explanation of the Code

Initialization: An array of integers is defined, and an iterator is created to double each value using the 
`map` function.

**Stream Creation**: The iterator is then converted into a stream using the `trpl::stream_from_iter` function.

**Asynchronous Loop**: `while let` loop is employed to asynchronously receive items from the stream as they 
become available.

The above code generates compilation error, indicating that the `next` method is not found for the 
`tokio_stream::iter::Iter` struct. 
The error message suggests that the required trait is not in scope and recommends importing the 
`StreamExt` trait.

```rust 
use trpl::StreamExt;

fn main() {
    trpl::block_on(async {
        let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        // --snip--
        let iter = values.iter().map(|n| n * 2);
        let mut stream = trpl::stream_from_iter(iter);

        while let Some(value) = stream.next().await {
            println!("The value was: {value}");
        }
    });
}
```

Conclusion

With the `StreamExt` trait correctly imported, the code compiles successfully and functions as intended. 
This allows the use of utility methods provided by `StreamExt`, enhancing the functionality similar to that
of iterators. 

## In-Depth Summary of Rust's Future, Stream, Pin, and Unpin Traits

Finer details of Future, Stream, and their associated traits in Rust, including the `Pin` type and the 
`Unpin` trait. 
These concepts are pivotal for understanding asynchronous programming in Rust.

### Understanding the Future Trait

Definition and Structure

The Future trait in Rust represents a value that may not be immediately available but will be resolved at 
some point in the future. 
The trait is defined as follows:

```rust 
pub trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

*Associated Type*: The Output associated type specifies what the future will resolve to, akin to the Item type
in the Iterator trait.

*Poll Method*: The poll method takes a `Pin` reference to `self` and a `mutable reference` to a `Context`,
returning a `Poll<Self::Output>`.

The Poll Type

The Poll type is defined as:
```rust
pub enum Poll<T> {
    Ready(T),
    Pending,
}
```

*Variants*: 
    `Ready(T)`: Indicates that the future has completed and the value is available.
    `Pending`: Indicates that the future is still working and the caller should check back later.

Usage of Poll

It is generally uncommon to call poll directly. If a future has returned Ready, calling poll again may 
lead to a panic. This behavior is similar to how `Iterator::next` operates.

Await and Poll

When using the `await` keyword, Rust translates this into calls to `poll`. For instance, an `await` on a 
future can be conceptually represented as:

```rust
match future.poll() {
    Ready(value) => { /* handle value */ }
    Pending => { /* handle pending state */ }
}
```

To manage a Pending state, a loop is typically employed, allowing the runtime to yield control and check 
back later.

The Pin Type and the Unpin Trait

#### Introduction to Pin

The Pin type is a wrapper that ensures a value cannot be moved in memory, which is crucial for types that 
contain references to themselves (self-referential types). The Pin type is defined as: `Pin<P>`

Where `P` is a pointer-like type (e.g., `&`, `&mut`, `Box`, `Rc`). 
The main purpose of `Pin` is to enforce safety guarantees around memory movement.

#### The `Unpin` Trait

The `Unpin` trait is a marker trait that indicates whether a type can be safely moved after being pinned. 
Most types in Rust implement Unpin by default, but some types, especially those that are self-referential, 
do not.

Pinning Futures

When futures are stored in collections (like vectors), they must be pinned to ensure their internal 
references remain valid. This is illustrated in the following code snippet:

`let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![/* pinned futures */];`

Error Handling with Pin and Unpin

If a future does not implement Unpin, attempting to use it in a context that requires it (like join_all) 
will result in compilation errors. 

The solution is to `pin` the futures using the `pin!` macro, allowing them to be safely moved into collections.

#### The Stream Trait

Definition and Structure

The Stream trait is conceptually similar to the Future trait but is designed for sequences of values that 
become available over time. 
The Stream trait is defined as follows:

```rust
trait Stream {
    type Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>>;
}
```

*Associated Type*: Item represents the type of items produced by the stream.
*poll_next Method*: This method polls the stream for the next item, returning a `Poll<Option<Self::Item>>`.

`StreamExt` Trait

The `StreamExt` trait provides additional convenience methods for working with streams, including an 
asynchronous next method that simplifies the process of retrieving items from a stream.
```rust 
trait StreamExt: Stream {
    async fn next(&mut self) -> Option<Self::Item>
    where
        Self: Unpin;
}
```

Integration of Streams and Futures

The Stream trait combines aspects of both Iterator and Future, allowing for asynchronous iteration over 
sequences of items. 
The `StreamExt` trait enhances usability by providing default implementations for common operations.


## Futures, Tasks, and Threads

### Introduction to Concurrency Models

Concurrency, can be implemented through two primary models: `threads` and `async` programming with futures 
and streams. 

It emphasizes that the choice between these models is not always binary; rather, it often involves 
utilizing both threads and async programming together, depending on the specific requirements of the task at
hand.

**Understanding Threads**

Historical Context and Limitations

Threads have been a staple in OS for long, providing a concurrency model that many programming languages 
support. However, the use of threads comes with certain limitations:
1. Memory Overhead:
    Each thread consumes a significant amount of memory, which can be a constraint in systems with limited 
    resources.

2. OS Dependency: 
    Threads are only available in environments where the operating system and hardware support them.
    This is a critical consideration for embedded systems, which may not have an operating system at all.

Characteristics of Threads

Threads serve as a boundary for synchronous operations, allowing for concurrency between different threads.
However, they operate in a "fire and forget" manner, meaning they run to completion without the ability to 
be interrupted or managed through a finer granularity like futures.

**The Async Model**

Complementary Tradeoffs

The async model offers a different set of tradeoffs:
1. Task Management: 
    Instead of relying on OS system-level management, tasks in the async model are managed by the runtime, 
    which allows for more efficient use of resources.

2. Concurrency: 
    Tasks can manage multiple futures, enabling concurrency both between and within tasks. 
    This flexibility allows for more complex operations without the overhead associated with threads.

**Futures as Granular Units of Concurrency**

Futures represent the most basic unit of concurrency in Rust. 
Each future can encapsulate a tree of other futures, allowing for intricate workflows. 
The runtime’s executor is responsible for managing tasks, while tasks manage futures, creating a layered 
structure of concurrency management.

#### Comparing Threads and Async Tasks

Strengths and Weaknesses

While `async` tasks offer advanced capabilities, they are not always superior to threads. 
The simplicity of the threading model can be advantageous in certain scenarios. 

**Threads**: 
    Easier to conceptualize for CPU-bound tasks where operations can be parallelized.

**Async Tasks**: 
    Better suited for I/O-bound tasks where operations may not require continuous CPU usage and can benefit 
    from non-blocking behavior.

#### Integration of Threads and Async

Threads and tasks can effectively work together: Many runtimes utilize a technique called "work stealing," 
which allows tasks to be moved between threads dynamically based on current utilization, enhancing overall 
system performance.

Guidelines for Choosing Between Threads and Async:

The chapter provides practical rules of thumb for deciding when to use threads versus async:

- CPU-bound Work: 
    If the task is highly parallelizable, threads are preferable.

- I/O-bound Work: 
    For tasks that require handling multiple inputs or outputs concurrently, `async` is the better choice.

- Combined Needs: 
    In scenarios requiring both parallelism and concurrency, developers can freely mix threads and async 
    programming to leverage the strengths of both approaches.

Example Implementation

Example below demonstrating how to send messages using a thread while awaiting them in an `async` block. 

```rust 
use std::{thread, time::Duration};

fn main() {
    let (tx, mut rx) = trpl::channel();

    thread::spawn(move || {
        for i in 1..11 {
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    trpl::block_on(async {
        while let Some(message) = rx.recv().await {
            println!("{message}");
        }
    });
}
```

- Creation of an async channel
- Spawning a thread that sends numbers 1 through 10, pausing for one second between sends.
- Using `trpl::block_on` to run an `async` block that awaits and prints the received messages.

This example illustrate a practical application of combining threads and async programming in Rust.

Real-World Applications

- video encoding tasks, which are compute-bound, can run on dedicated threads, while UI updates can be 
  handled asynchronously through channels.

Conclusion and Future Directions

The chapter emphasizes that concurrency will continue to be a theme in the book, with more complex 
applications and comparisons between threading and async tasks explored in later chapters. 

It reassures readers that Rust provides robust tools for writing safe and efficient concurrent code, whether 
for high-performance web servers or embedded systems.

