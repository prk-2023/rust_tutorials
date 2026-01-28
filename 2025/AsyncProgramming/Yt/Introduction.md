# Asynchronous Programming:


## 1.0 Introduction:
It's a non-blocking technique that allows a program to initiate long-running tasks, such as network
request or file I/O and continue executing other code rather then waiting for the task to finish. 

The idea is to enhance application responsiveness and performance by enabling concurrency, often using
mechanisms like callbacks, promises and async/await to manage background operations.

Behind the seen the waiting request is handled by a separate thread or event loop depending on the
environment. There is no limit and we can initiate many background tasks more then the number of CPU
cores of the system, because Asynchronous tasks aren't always CPU bound, the number of background tasks you 
can run depends on how your runtime handles concurrency, threading and the nature of the work.

Async and multi-threaded programs come with caveat ( warnings ):
- Each thread or background task runs independently, if one crashes or freezes you will  need a plan on
  how to handle that otherwise or-else you risk failing silently or corrupting your program state flow.
- Thread can share resources files, memory, and databases, which require a careful co-ordination to
  prevent race-conditions.
- Thread life cycle management: If you are manually working with threads or worker pool and forget to
  clean them up when not required, they stick around wasting resources, this is called thread leak, and
  its one way your code can slowdown or crash over time. 

Key point is Asynchronous programming and concurrency allow you to do more and faster, But they also
introduce complexity, timing issues, race conditions, and resource contentions can sneak up if you are
not careful. 
So the key is to implement correctly to build efficient and high performance applications.

### 1.1 Key Concepts and Benefits:

- Non-Blocking: 
    Unlike synchronous, where tasks run sequentially, async tasks run independently, preventing the
    main thread from freezing.

- Improved User Experience: 
    Essential for UI development to keep applications responsive while loading data. 

- Efficiency: 
    Ideal for I/O-bound operations (e.g., API calls, database queries), allowing better resource
    utilization.

- Performance: 
    While not always true parallelism, it allows one CPU thread to handle multiple tasks by switching 
    between them during wait times.

### 1.2 Definitions:

- Synchronous: Tasks happening one after another. Is task A is slow database call, then task B gets
  blocked till task A finishes. 


- Asynchronous: Programming model where you can start a task and then move on to something else before
  that task finishes. It's about non-blocking flow.

- Concurrency: Its about dealing with many things at once. It's scheduling property. On a Single-code
  processor you have concurrency by rapidly switching between tasks ( making progress on all , but only
  running one task at a time).

- Parallelism: Doing many things at once. This requires multiple CPU cores physically running different
  pieces of code at the exact same moment. 


### 1.3 Bottlenecks: 

Key point is to keep in mind of the "benefits" of asynchronous programming depends entirely on the
**bottlenecks** of the system.

1. I/O Bound tasks ( Where Async shines ):

    If your program is waiting for a website to respond or a file to load, the CPU is just sitting idle.
    - **The Context**: Async allows the CPU to go do other work while the "wait" happens.
    - **The Confusion**: People think "Async = Faster." In reality, the task takes the same amount of time;
      you’re just not wasting the CPU's time while it happens.

2. CPU-Bound Tasks ( Where Async fails ):

    If you are calculating Pi to a billion digits, using `async`/`await` won't help you.
    - **The Context**: The CPU is already working at 100%. If you try to `async` a heavy calculation on a
      single thread, the UI will still freeze because the CPU is occupied.
    - **The Confusion**: This is where developers get frustrated when their `async` app still feels laggy.
      For this, you need Parallelism (Threads/Workers).

3. Conceptual "Wait":
    The final layer of confusion is the Syntax vs. Reality. 

    When you see await in code, it looks like the program stops.
    In a synchronous world, the thread is "held hostage." 
    In an asynchronous world, the thread is "released" back to the system to handle other
    events (like button clicks or mouse moves) until the data returns.


### 1.4 History:

1. Fist computers had single CPU that executed a set of instructions, with no OS, no scheduling, no
   threads, no multitasking. Which slowly moved to simple OS (DOS) where the entire CPU is yielded to the
   program currently executing, and it was the task of the programmer to make programs work and implement
   any kind of multitasking for their program. This method has its limitations and can not scale, example
   interactive UI's .. 

2. Non-Preemptive multitasking: It refers to a method in OS where a running process or task cannot be 
   interrupted by the OS and holds the CPU until it finishes or voluntarily waits. 
   This approach ensures tasks run to completion, reducing context-switching overhead, but can cause higher 
   waiting times and reduced responsiveness

3. Preemptive multitasking: Is an OS method that allows the OS to interrupt (preempt) a currently running 
   process to switch the CPU to another task, preventing any single application from monopolizing system 
   resources. 
   It improves system responsiveness and stability by enforcing time-sharing among applications, commonly 
   used in modern OSs like Windows, Linux, and macOS. 
   Offers Fair resource distribution, high responsiveness.
   Key aspects: Forced interruption, Priority Scheduling, Stability, Context switching...

4. Hyper-Threading (HT) is Intel's technology that lets a single physical CPU core act like two logical 
   cores, processing two instruction threads simultaneously to boost performance, especially in 
   multi-threaded tasks like video editing or complex multitasking, by better utilizing unused parts of the
   core's pipeline. 
   It creates two "virtual" processors from one physical core, improving efficiency but not matching the 
   power of two full physical cores, with typical gains of 15-30% for suitable workloads, though it can 
   sometimes hurt performance in games, leading to Intel phasing it out in newer designs.

5. Multicore processor: A single computing component (integrated circuit) featuring two or more independent
   processing units, or "cores," that read and execute program instructions simultaneously. 
   By allowing parallel processing, these processors enhance multitasking, increase overall speed, and 
   improve energy efficiency compared to single-core alternatives, serving as the standard for modern 
   computers, smartphones, and servers.

6. What Programmers write and what OS interprets: 
    Generally programmers perspective the process and code should perform as expected but from the OS
    perspective it might or might not as OS has to perform many other tasks like handling interrupts
    and manage how CPU handles concurrency.... 
    Apart from this these modern processors can re-order instructions by using *out-of-order* execution if
    it believes it makes things faster this way. 

7. Distinction between Concurrency and Parallelism:
    - Concurrency: Its about dealing with many things at the same time. i.e managing multiple tasks to make
      progress on them over overlapping time ( like juggling ) often via task switching on one CPU.

    - Parallelism: Its about doing many things at same time. i.e doing multiple things simultaneously by
      executing them at the exact same moment, requiring multiple processing units.

    Note: Dividing a single task in to components and parallelizing them is not necessarily concurrency. 

8. Terminology for Concurrency and Parallelism:

    * Resource: Required to run a process a task. These are limited ( CPU time or memory )
    * Task: Set of operations that requires some resource to progress. ( this generally contains many
      sub-operations)
    * Parallel: Thing happening independently at same time. 
    * Concurrent: Tasks that are in process at the same time, but not necessarily processing simultaneously.

    If two tasks are running concurrently but are not running in parallel they must be able to *stop* and
    *resume* their progress. This type of task is called *interruptible* if it allows for this kind of
    concurrency.


9. Main Distinction between concurrency and parallelism.
    Concurrency is about working smarter, Parallelism is a way to throwing more resources at same problem.

    Asynchronous programming code mostly makes sense when you need to be smart to make optimal use of your
    resources. 

    When it comes to writing code with a aim to solve a problem, then giving additional resources if you can
    split it into parts that you can work on it in parallel. 

    So selecting the code to be concurrent and parallel is to be done based on use cases:

    Ex: 
        - Concurrency: 
            Performing some I/O which requires to wait.
            Prevent complex tasks for taking longer time and resource locking. 

10. Threads: OS threads seem to be mapped to cores, generally OS maps one thread to a core, but when thread
    count increases more then the number of cores, the OS will switch between these threads and process each
    of them concurrently using scheduler to give each thread a run time to run. 

    => threads can be a means to perform tasks in parallel, but they can also be a means to achieve
    concurrency. 


With the above factors its important to think before programming how the OS might run your code from start
to end.


11. Asynchronous vs Concurrency:

    Asynchronous programming is the way a programming language or library abstracts over concurrency
    operations and how we as users of the language/library use that abstraction to execute tasks
    concurrently. 

    Note: OS threads can perform the same abstraction to handle asynchrony, which is referred to as
    Multithreaded programming. 


12. Communicating with OS:
    **syscalls**, these are exposed to the programs via *libc*.

    **Syscalls** in Rust are typically exposed through the `std` lib, which abstractly handles the
    underlying OS interactions. ==> `std` often depends on `libc` via *FFI*. 
    But Rust can also invoke *syscalls* directly using assembly or specialized crates without `libc`,
    particularly in `no_std` environments.

    * `std` library: Most Rust programs ( `File::open` ) which acts as a wrapper around OS `syscalls`. 
    * `libc` Crate: For direct interaction, Rust developers often use `libc` crate which provides 
      `extern "C"` bindings to the systems C library.
    * Direct Assembly/inline ASM : Using `core::arch::asm!` Rust can execute syscall instructions, bypassing
      `libc` entirely. This is common in low-level programming or `no_std` environment (eBPF).
    * Wrapper crates: Other crates `nix`, `linux-raw-sys` provide safer and direct wrapper around syscalls.

    So `FFI` is used for Standard usage (`std` depends on `libc`).

    **FFI mechanisms**: When `libc` is used, it is accessed through Rusts *FFI* which allows calling C
    functions and managing data layout ( `#repr(C)`).

13. CPU and OS: Using inline assembly and raw pointers to see how how CPU actually talks to memory:

```rust 
use std::arch::asm; // Required to use the asm! macro

fn main() {
    let t = 100;
    let t_ptr: *const usize = &t;
    
    // Fixed the function name typo here
    let x = de_ref(t_ptr); 
    
    println!("Value from memory: {}", x);
}

fn de_ref(ptr: *const usize) -> usize {
    let mut res: usize = 0; // Initialized to satisfy the compiler
    unsafe {
        // {0} refers to 'res' (output), {1} refers to 'ptr' (input)
        asm!(
            "mov {0}, [{1}]", 
            out(reg) res, 
            in(reg) ptr
        );
    }
    res
}
```

The `unsafe` block, you are essentially telling the compiler: 
"I promise this mem addr is valid and holds a `usize`. If I'm wrong, feel free to crash the program."

- `asm!`: In Rust we usually deal with variables, with assembly we talk about *registers* and memory
  addresses. ( registers: small, lightning fast storage slots inside CPU )
- instruction `mov {0}, [{1}]` Move destination, source. It copies data from source to destination.
- `{0}`: represents the `res` variable.
- `[{1}]`: The square brackets are key. They mean "dereference." Instead of using the memory address
  itself (the pointer), the CPU goes to that address and grabs the value stored there.
- `out(reg) res`: Tells Rust to grab whatever value ends up in the register and save it to `res`.
- `in(reg) ptr`: Tells Rust to put the address of t into a register so the assembly can use it.


If we change the `t_ptr` address to some random `let t_ptr = 1111111111111111 as *const usize` we will
get a segmentation fault as the memory is illegal to access.


**Reason for segmentation fault**: Modern Computing relies on the below mechanisms to manage memory
  efficiently and securely: 

1. Virtual Memory: 

    A memory management technique that creates an abstraction of physical storage, giving each process the
    illusion of having a very large, private and contiguous block of memory.
    It allows programs to run even if they require more RAM than is physically available by using secondary
    storage (like an SSD) to hold inactive data. 
    It also provides isolation, ensuring one process cannot accidentally or maliciously access another’s 
    data. 

2. Page Table:

    A **data structure** managed by the OS that stores the mapping between virtual addresses (used by the
    program) and physical addresses (actual locations in RAM).
    When a CPU executes an instruction to read memory, the *MMU* consults the page table to translate the
    virtual address into a physical one. Each process typically has its own unique page table. 

3. Page Fault

    A specific type of exception raised by the hardware (MMU) when a program tries to access a memory pate
    that that is not currently mapped in RAM.
    => The OS handles this by fetching the missing data from disk, loading it into a physical memory frame,
    updating the page table, and then restarting the instruction that caused the fault. 

4. Exceptions

    Synchronous events triggered by the CPU in response to an internal condition or error during instruction
    execution (e.g., division by zero or a page fault).
    => An exception forces the CPU to pause the current program and transfer control to a specific handler
    in the OS kernel to resolve the issue or terminate the process if it is an illegal operation. 

5. Privilege Level

    A hardware-enforced security mechanism (often called "rings," such as Ring 0 for the kernel and Ring 3 
    for users) that restricts which instructions a piece of software can execute.
    => It prevents regular app's (user mode) from performing sensitive tasks, such as directly modifying the
    page table or accessing HW, which are reserved for the OS (supervisor mode)

    Any attempt by user mode trying to change the page table will be treated as exception by the CPU and an
    exception is triggered causing a jump to the handler for that exception provided by the OS> 


## 2.0 Asynchronous Programming: ( program logic flow )

Abstract: How programming languages model Asynchronous program flow, threads, co-routines, futures.

Threads, Futures, fibers, goroutines, promises -.- are all abstractions that give us a way to model an
asynchronous programming flow. 

Topics covered:
- Definitions
- Threads ( OS threads )
- Green threads/ Stack co-routines/ fibers 
- Callback based approach 
- Promises, Futures and async/await


### 2.1 Definitions:
Abstraction can be of two types, those that yield or suspend or to a scheduler, tasks that are generated by
`async/await` in Rust. ( example: a data base query or network call ).
Other kind are those that do not yield, and require OS scheduler to pre-empt a running task, where scheduler
can stop and take control of CPU  even the task would have been able to do work and progress. ( ex: OS
threads, Go-Routins )


Co-operative tasks start and yield to the scheduler to perform multitasking. 
Non-cooperative tasks the scheduler starts/stops the task to perform multitasking.

Tasks that have there own *call stack* are called stackful and they can suspend execution at any time in the
program as the whole stack is preserved. 
Taks that do not have there own call stack or use shared call stacks can not be suspended in the middle of
stack frame, this limits the ability to pre-empt them. 


### 2.2 Threads:

Two types:
- OS threads : OS creates and manages them. ( kernel threads )
- User threads: created by user programs, created and managed by user program and OS does not know about
  them. 

Creating new threads:
    Creating OS threads : Involves initialization and bookkeeping. Switching between running threads in same
    process is fast, its same with creating and discarding threads. But too many of such small tasks can be
    a problem.

Stack: Each thread has its own stack ( this can be a cause of concern as they are open doors for stack
overflow if we configure them with to small for the tasks you're running. )

Context switching: When CPU stops executing one thread and proceeds with another one. Threads within the
same process can share the resources as all child threads share the same address space.

Scheduling: OS can schedule tasks differently, every time a task yields to OS you put that thread in to
queue along with all other threads and processes of the system. Care should also be taken to prevent race
conditions. 
Rust language will help in preventing many of the pitfalls, but synchronization adds on to extra work and
complexity of the program. 

### 2.3 Decouple async from threads:

Decoupling threads from asynchronous operations is the core "magic" of modern high-performance software. 

In short: **Threads are workers, while async operations are tasks.**

In a traditional synchronous model, a worker is tied to a task until it's finished. 
In an asynchronous model, the worker just starts the task and moves on.

---

=> The Core Concept: The "Waiter" Analogy

* **Thread-per-request (Coupled):** A waiter takes your order, and waits "blocked" and cannot help anyone else.
* **Asynchronous (Decoupled):** A waiter takes your order, hands the ticket to the kitchen, and jumps to the
  next order. When the order is ready any available waiter can pick and close the task.

=> How it works: The Event Loop & Polling

When you decouple the two, you use a **Runtime** (like Tokio in Rust, libuv in Node.js, or the .NET ThreadPool). 

It uses three main components:

1. The Future/Promise: A placeholder for a value that hasn't arrived yet.
2. The Executor (The Worker): A small pool of threads that execute the code.
3. The Reactor (The Notifier): A system-level tool (like `epoll` on Linux or `IOCP` on microsoft) that
   watches hw/network signals. 

Instead of a thread waiting for a network packet, the thread tells the OS: *"Hey, let me know when data 
arrives on this socket,"* and then that thread returns to the pool to do other work.


=> Why do this?

* Efficiency: You can handle 10,000 concurrent network connections using only 4 or 8 CPU threads.
* Scalability: You avoid the massive memory overhead of creating thousands of threads (each thread usually
  allocated ~2MB of stack memory).
* Responsiveness: Your main "UI" or "Acceptor" thread is never stuck waiting for a slow database query.

| Feature | Thread-Based (Coupled) | Async-Based (Decoupled) |
| --- | --- | --- |
| **Resource Usage** | High (1 thread per task) | Low (Many tasks per thread) |
| **Idle Time** | Thread sits doing nothing | Thread performs other work |
| **Complexity** | Simple mental model | Requires a runtime/state machine |
| **Best For** | Heavy CPU calculations | I/O bound tasks (Web, DB, Files) |

In Rust, decoupling is handled through a **State Machine** model. 
Unlike other languages, Rust’s async is "lazy" nothing happens unless you specifically ask for it.

Short list of how Rust handles this:

* The Future Trait: 
    Every `async` function returns a `Future`. 
    It’s not background task yet; just a data structure that says, "I have the *potential* to finish later."

* Zero-Cost State Machines: 
    The compiler transforms your `async` code into a state machine. 
    This avoids the memory overhead of "green threads" or hidden allocations.

* Polling (The "Are we there yet?" model):
    The runtime (like **Tokio**) doesn't push data to your task. 
    Instead, it calls `poll()`. If the task is blocked, it returns `Pending` and the thread immediately
    moves to a different task.

* The Waker: 
    When an I/O operation (like a disk read) finally finishes, the OS sends a signal. 
    Rust uses a `Waker` to tell the Executor: "the specific task is ready now—put it back in the queue."

* The Executor vs. The Reactor:
    * The Executor manages the threads and runs the tasks.
    * The Reactor talks to the OS (via `epoll` or `io_uring`) to wait for hardware events.

=> The Result

You get the performance of manual callback-based C++ with the readability of synchronous code. 
One thread can manage thousands of `Futures`, swapping between them only when they are actually ready to 
do work.

```rust 

use std::thread::{self, sleep};

#[allow(dead_code)]
#[allow(unused_variables)]
fn main() {
    let t1 = thread::spawn(move || {
        sleep(std::time::Duration::from_millis(2000));
        println!("1!");
    });

    let t2 = thread::spawn(move || {
        sleep(std::time::Duration::from_millis(1000));
        println!("2!");
        let t3 = thread::spawn(move || {
            sleep(std::time::Duration::from_millis(500));
            println!("3!");
        });
        t3.join().unwrap();
    });
    println!("The tasks run concurrently!");
    t1.join().unwrap();
    t2.join().unwrap();
}
```
- use OS threads and put them to sleep. ( sleep is same as yielding to OS with a request to re-schedule to
  run after a certain time has passed.)
































