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

Key elements of Rust async programming are *futures* and Rust's `async` and `await` keywords.

Rust provides a `Future` trait as a building block so that many different async operations can be
implemented with different data structures but with a common interface. 

=> So in Rust futures are types that implement a `Futures` trait. Each future holds its own information
about the progress that has been made and what "ready" means.

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
    Write a function that takes one page URL as a parameter, makes a request to it, and returns the text of
    the title element.


