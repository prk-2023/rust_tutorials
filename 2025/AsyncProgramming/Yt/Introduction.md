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
- OS threads : OS creates and manages them. ( kernel threads ) ( these are created using std::threads )
- User threads: created by user programs, created and managed by user program and OS does not know about
  them. (these are created using Tokio... or other async executors)

1. OS thread:  ( In rust we use `std::thread` )
  - OS is the boss 
  - OS decides by looking all the threads from a program and decides thread_A to run on core #1 for 10 ms, and then stop and run thread_b...
  - Ever time the OS decided switch as above, it has to jump to kernel mode and save everything the CPU was doing and load the new thread. **Context Switching**.

2. User-space threads ( green threads ): ( In Rust we can create them using async executors ex: Tokio )
  - Your program ( runtime like *tokyo* ..) 
  - The OS only sees, say, 4 OS threads. But inside your code, you have 10,000 "Tasks." Your Runtime (tokio) manually swaps these tasks in and out of those 4 OS threads. The OS has no idea the tasks even exist.
  - As runtime is in user-space, it switches tasks in "user mode" with out the help of OS. The operation just moving a pointer around memory ( and inexpensive )

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
The "magic" is what allows a modern web server to handle 100,000 requests simultaneously using only 8 CPU cores.

In short: **Threads are workers, while async operations are tasks.**
In Async operations the Worker is the thread and it can creat tasks and moves on.

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


### 2.4 fibers/green threads: 


Fibers and green threads are lightweight, user-space concurrency primitives designed for efficient
multitasking without the overhead of OS threads. 

They are managed by a runtime or application, allowing thousands or millions of tasks to run within a 
single thread, relying on cooperative multitasking to yield control.

Key Concepts of Fibers and Green Threads: 
- Definition & Purpose: 
    Both are "user-space" threads, meaning they are managed by the application code or a virtual machine
    (VM) rather than the OS kernel. They enable high-concurrency applications without heavy resource
    consumption.

- Green Threads: 
    Historically refers to threads managed by a VM (like early Java). 
    They are designed to mimic native threads but are scheduled by the user-level runtime.

- Fibers: 
    Often described as a type of coroutine with a scheduler. 
    They are "stackful," meaning each fiber carries its own stack, allowing for complex control flow, such
    as pausing and resuming (yielding).

- Cooperative vs. Preemptive: 
    Unlike OS threads which are preemptively switched by the kernel, fibers/green threads generally 
    require voluntary yielding (cooperative multitasking).

- Efficiency: 
    Because context switching between fibers happens in user space (without kernel intervention), they are 
    much faster and cheaper to create than OS threads.

- M:N Scheduling: 
    Many fibers (\(M\)) can be mapped onto a smaller number of OS threads (\(N\)). 

- Common Use Cases: 
    Asynchronous I/O: Ideal for network requests, web servers, and database operations.

- High-Concurrency: 
    Scenarios needing many simultaneous tasks (e.g., chat apps). 

Differences: 

- Scheduling: 
    Some definitions suggest "green threads" imply a runtime scheduler automatically switching them, 
    whereas "fibers" may require explicit yielding by the developer.

- Terminology: 
    They are often used interchangeably, although "fibers" frequently refers to a more modern, explicitly 
    controlled mechanism.


**fibers** and **green threads** are often used interchangeably, but they represent a specific way of 
handling "multitasking" without relying directly on the OS heavy lifting.

With Rust context:

1. The Core Concept: User-Space Scheduling

In standard programming, when you create a thread, you are usually creating an **OS Thread** (Kernel Thread). 
The OS is responsible for pausing one thread and starting another (context switching).

**Green threads** and **fibers** move this responsibility from the OS to the **Application** (or its runtime).

* **Green Threads:** 
    "pseudo-threads" managed by a language's runtime (like Go's goroutines). 
    The runtime schedules thousands of green threads onto a small number of real OS threads.

* **Fibers:** 
    Term more common in C++ or Windows env. 
    A fiber is essentially a green thread that uses **cooperative multitasking**. 
    It won't stop running until it explicitly says, "I'm done for now; someone else can go."

2. Key Differences

While they solve similar problems, the nuances lie in how they yield control.

| Feature | OS Threads | Green Threads / Fibers |
| --- | --- | --- |
| **Managed by** | Operating System (Kernel) | Language Runtime / Library |
| **Context Switch** | Expensive (High overhead) | Cheap (Low overhead) |
| **Memory** | Large stack (usually ~2MB) | Tiny stack (KB range) |
| **Multitasking** | Preemptive (OS interrupts them) | Cooperative (They must "yield") |

3. Rust specifics:

History of Rust, it actually **used** to have green threads (around version 0.1). Removed from 1.0 release 
for a few reasons:

* **The "Runtime" Cost:** 
    Green threads require a heavy runtime to manage the switching. 
    Rust aims for "zero-cost abstractions" and wants to work on embedded systems where a big runtime isn't 
    possible.

* **FFI Issues:** 
    Calling C code from a green thread is notoriously difficult and slow.

4. What Rust uses instead: *Futures* & *Poll*

Rust settled on a **State Machine** approach rather than fibers. When you write `async` in Rust:

1. The compiler turns your function into a **Future**.
2. A **Runtime** (like **Tokio** or **async-std**) polls that Future.
3. Instead of a fiber saving its entire "stack" (local variables, etc.), the Future only saves the specific
   data it needs to continue.

> **Note:** While Rust doesn't have native green threads, some libraries (like `may`) do implement them. 
> However, the vast majority of the Rust ecosystem uses the `async/await` state-machine model because it is
> more memory-efficient.

### 2.5 Callback based approach:

Early days of async programming, the **Callback-based approach** was the primary way to handle long-running
tasks without "blocking" the main thread.

Instead of waiting for a task to finish and returning a value, a function accepts an extra argument:
**another function** (the callback) to be executed once the task is complete.

1. How It Works

Think of it like ordering a pizza for pickup.

* **Synchronous:** 
  You stand at the counter and stare at the oven until the pizza is done. You can't do anything else.

* **Callback:** 
  You give the shop your phone number and go run errands. They **call you back** when the pizza is ready.

The Flow:

1. Initiate: You call an async function (e.g., `download_file`).
2. Register: You pass a callback function to it.
3. Continue: The async function returns immediately, allowing your code to keep running other logic.
4. Trigger: When the download finishes, the background worker "fires" your callback with the result.

---

2. A Simple Comparison

To see why this was a breakthrough (and eventually a headache), look at the transition from blocking to 
callbacks.

Synchronous (Blocking)

```rust
let data = read_file("config.txt"); // Program stops here until file is read
process(data);

```

Callback (Non-Blocking)

In a callback world (using pseudo-code, as Rust favors Futures), it looks like this:

```javascript
read_file("config.txt", (data) => {
    // This code runs ONLY when the file is ready
    process(data);
});
// This code runs IMMEDIATELY, even before the file is read
console.log("Moving on to other tasks...");

```

3. The Downside: "Callback Hell"

While callbacks are efficient for the computer, they are brutal for the programmer. 
When you have multiple dependent async tasks, you end up with deeply nested code often called the 
**Pyramid of Doom**.

**Example of the mess:**

```javascript
getData(function(a) {
    getMoreData(a, function(b) {
        getEvenMoreData(b, function(c) {
            getFinishedData(c, function(d) {
                console.log(d);
            });
        });
    });
});

```

This makes error handling and debugging incredibly difficult, as each layer needs its own error logic, and 
the "stack trace" is often lost between calls.

---

4. How this leads to Rust's `async/await`

Rust (and modern languages like JavaScript and Python) moved away from raw callbacks to solve this nesting issue.

* Callbacks: 
    You give the function a "phone number" to call when it's done.
* Futures/Promises: 
    The function gives you a "receipt" (a Future) that represents a value that will exist later.
* Async/Await: 
    A syntactic sugar that lets you write code that *looks* synchronous but behaves like a callback behind 
    the scenes.

In Rust, the compiler effectively takes your `async` code and breaks it into a state machine, essentially 
handling the "callback" logic for you without the messy nesting.

The idea behind callback approach is to save a pointer to a set of instructions we want to run later
together with whatever state is needed. ( this would be a *closure* )

Implementing callbacks dose not require any context switching or pre-allocated memory for each task.

cons:
Memory usage grows with number of callbacks.

Ownership can be hard to reason about.

Sharing state between tasks is not simple.

Debugging callbacks is not easy.


### 2.6 Promises and futures:

Callback approach led to "Callback Hell," the programming world needed a better way to manage pending 
results. 

Enter **Promises** and **Futures**.

While different languages use different names, they represent the same concept: A **placeholder for a value
that hasn't been computed yet.**

1. The "IOU" Analogy

Imagine you go to a busy burger joint.

* **Callback:** You give them your number, and you have to stay alert for the call.
* **Future/Promise:** They give you a **buzzer**.

The buzzer is an object you hold. It isn't a burger, but it’s a **legal promise** that a burger will 
eventually exist. You can check if the buzzer is vibrating, or you can set it on the table and wait.

2. Futures vs. Promises (The Terminology)

The distinction is often subtle and depends on the language, but here is the general rule of thumb:

* Future: 
    A "read-only" view of the result. 
    You are the consumer waiting for the value. (Common in **Rust** and **Java**).

* Promise: 
    The "writable" side.
    The background task "fulfills" the promise by shoving a value into it. (Common in **JavaScript**).

In many modern contexts, people use the terms interchangeably to mean: 
    *"An object representing an eventual completion."*

3. How they solve "Callback Hell"

The magic of Futures/Promises is **Chaining**. 
Instead of nesting functions inside functions, you "chain" them linearly.

#### The JavaScript way (Promises)

```javascript
fetchData()
  .then(data => process(data))
  .then(result => display(result))
  .catch(err => console.error(err));

```

#### The Rust way (Futures)

In Rust, a `Future` is a trait. 
When you call an `async` function, it doesn't run immediately; it returns a type that implements `Future`.

```rust
let future_data = fetch_data(); // Nothing has happened yet!
let processed_data = future_data.await; // Now we pause and wait for the result

```

4. The "Pull" vs. "Push" Difference

This is the most important part for a Rust learner. There are two ways to handle these objects:

#### JavaScript/C# (Push-based)

As soon as a Promise is created, it starts running in the background. 
When it’s done, it "pushes" the result to the next step. The runtime is actively driving the promise forward.

#### Rust (Pull-based / Lazy)

In Rust, **Futures do nothing unless you poll them.** 
If you call an async function and don't `.await` it (or spawn it), the code inside will **never execute**. 
The runtime (like Tokio) has to "pull" the future to make progress.

> **Why?** 
> This makes Rust's async system "zero-cost." 
> If you don't use it, you don't pay for the overhead of background management.

Summary

| Feature | Callbacks | Promises/Futures |
| --- | --- | --- |
| **Structure** | Nested (Pyramid) | Linear (Chained) |
| **Control** | Inversion of Control | You hold the handle to the result |
| **Error Handling** | Hard (handled in every callback) | Easy (one `.catch()` or `?` operator) |
| **Readability** | Poor | High |



### 2.7 Async/Await ( co-routines )

The final state of asynchronous programming:
**Async/await** is the syntax we use today, but **Coroutines** are the engine sitting under the hood that 
makes it possible.

1. What are Coroutines?

Think of a standard function (a **subroutine**) as a one-way trip: you call it, it runs to completion, 
and it returns. You cannot "pause" it in the middle and come back later without losing your place.

A **Coroutine** is a function that can **suspend** its execution and **resume** later. 
It remembers exactly where it was, including the values of all its local variables.

The Two Flavors:

* Stackful Coroutines:
    These act like **Green Threads/Fibers**. 
    They have their own stack, so they can pause anywhere (even deep inside nested function calls).

* Stackless Coroutines:
    These are what **Rust** and **JavaScript** use. 
    They don't have their own stack. 
    Instead, the compiler transforms the function into a **State Machine**. 
    This is much more memory-efficient.

---

2. Async/Await: The "Syntactic Sugar"

If Coroutines are the engine, `async/await` is the steering wheel. 
It allows you to write asynchronous code that looks and feels exactly like synchronous (blocking) code.

The Transformation

When you mark a function with the `async` keyword, you are telling the compiler: 
*"Turn this function into a Coroutine (a state machine) that returns a Future."*

When you use the `await` keyword, you are saying: *"Suspend this coroutine right here. 
Don't block the thread; just save my spot and let the executor know to wake me up when the result is ready."*

---

3. How it looks in Rust

In Rust, this is a beautiful partnership between the **Language** and the **Runtime**.

```rust
async fn get_user_data(id: u32) -> User {
    // 1. We call an async function. It returns a Future.
    // 2. We .await it. The coroutine "pauses" here.
    let raw_data = db::fetch(id).await; 
    
    // 3. Once db::fetch is done, the executor resumes us right here.
    let user = parse(raw_data);
    
    user
}

```

What the Compiler actually sees:

Behind the scenes, Rust turns that function into an `enum` that looks something like this:

```rust
enum GetUserDataStateMachine {
    Start,
    WaitingForDb(db::FetchFuture), // Saving the state while we wait
    Finished,
}

```

Because it's just an `enum`, it takes up almost no space in memory. 
This is why you can have **millions** of concurrent tasks in Rust without crashing your computer.

---

4. The Big Picture Comparison

| Concept | The Analogy |
| --- | --- |
| **Callbacks** | Giving the chef your phone number and leaving. |
| **Promises/Futures** | Holding a buzzer that goes off when the food is ready. |
| **Coroutines** | The ability for the chef to pause mid-burger, help another customer, and return to the exact same burger. |
| **Async/Await** | Telling the chef: "I'll wait for this burger," but you actually teleport to a different table to work until it's done. |

---

The "Gotcha" in Rust

Unlike JavaScript, where the "Executor" is built into the browser or Node.js, 
**Rust has no built-in executor.** If you write `async/await` code in Rust, it won't run until you pull in 
a library like **Tokio**. 
The `async` keyword just builds the machine; the executor is the electricity that turns it on.


### 2.8 Tokio Executor:

To understand how **Tokio** (or any Rust executor) works, you have to look at the 
**Reactor-Executor Pattern**.

In Rust, as we discussed, futures are **lazy**. 

They don't move unless someone calls `poll()` on them. 

The Executor is the "boss" that calls `poll`, and the Reactor is the "assistant" that watches the hardware
(network/disk) for updates.

1. The Three Main Players

To run an async task, Rust uses a specialized loop involving three components:

- **The Executor:** 
   A loop that constantly checks a queue of "Ready" tasks and calls `poll()` on them.

- **The Future (Task):** 
   The state machine we talked about. When polled, it returns either `Poll::Ready(value)` or `Poll::Pending`.

- **The Reactor (Waker):** 
    This is usually part of the OS (like `epoll` on Linux or `iocp` on Windows). 
    It waits for external events (like a socket receiving data) and "wakes up" the task.

2. The Lifecycle of a Request:

Let’s say you want to read data from a TCP socket:

* Step 1 (The Call): 
    You call `socket.read().await`.

* Step 2 (The Poll): 
    The Executor calls `poll()` on your task. 
    The task tries to read the socket, but the data isn't there yet.

* Step 3 (The Registration): 
    Your task returns `Poll::Pending`. 
    Crucially, it gives a **Waker** (a handle) to the Reactor, saying: 
        *"Hey, when this socket has data, use this handle to wake me up."*

* Step 4 (The Sleep): 
    The Executor puts your task aside and works on other tasks. 
    It doesn't waste CPU time "spinning" on your empty socket.

* Step 5 (The Wakeup): 
    Minutes or milliseconds later, data arrives. 
    The Reactor sees this and calls `wake()` on your task's handle.

* Step 6 (The Re-poll): 
    Your task is moved back into the Executor's "Ready" queue. 
    The Executor calls `poll()` again. This time, the data is there, and the task finishes!

3. Why Tokio is Fast: Multi-threaded Work Stealing

Tokio isn't just a simple loop; it’s a **Work-Stealing Scheduler**.

If you have a 4-core CPU, Tokio starts 4 worker threads. Each thread has its own queue of tasks. If Thread A finishes all its work while Thread B is overwhelmed, Thread A will actually "steal" tasks from Thread B’s queue. This keeps all your CPU cores busy at all times.

---

4. Summary: The Big Picture

By combining all the concepts we've discussed, here is the "Stack" of Rust Async:

* **Async/Await:** The syntax (how you write it).
* **Futures/Coroutines:** The state machines (how the compiler stores it).
* **Wakers:** The notification system (how the task says "I'm ready").
* **Tokio (Executor):** The engine that drives the polling and manages threads.

=> Important Caution for Rust

Because Rust uses this cooperative model, **you must never block the thread** inside an `async` function.

If you call a heavy synchronous function (like `std::thread::sleep` or a massive math calculation) inside 
an `async` block, the Executor thread stops. Because it’s stopped, it can’t poll any other tasks, and 
your **entire server freezes**.

> **Pro Tip:** Always use `tokio::time::sleep` instead of `std::thread::sleep`!


5. Other popular executors:

While **Tokio** is the undisputed "giant" of the Rust ecosystem, several other executors exist:

The choice of executor often depends on whether you are building a high-performance web server, 
a tiny embedded device, or a specialized Linux-only service.

- Popular Alternatives to Tokio

| Executor | Best For... | Key Philosophy |
| --- | --- | --- |
| **smol** | Lightweight apps | Small, simple, and easy to understand (only ~1000 lines of code). |
| **Embassy** | Embedded systems | No-standard-library (`no_std`), zero-allocation, built for microcontrollers. |
| **Glommio** | High-perf storage/IO | **Thread-per-core** architecture; uses Linux `io_uring` for maximum speed. |
| **async-std** | "Standard" feel | (Legacy/Maintenance) Aimed to be an async version of the Rust `std` library. |

---

**smol: The Minimalist**

If Tokio is a heavy-duty truck, `smol` is a bicycle. 
It is designed to be as small as possible while still being very fast.

* **Why use it?** If you want to avoid the "bloat" of Tokio or if you want an executor that is easy to audit.
  It doesn't use a heavy "Work-Stealing" scheduler by default, making it more predictable for certain small 
  tasks.

**Embassy: The Embedded Hero**

In the world of microcontrollers (like STM32 or ESP32), you don't have an Operating System to manage threads.

* **The Magic:** 

    Embassy allows you to use `async/await` on chips with only a few kilobytes of RAM. 
    It treats hardware interrupts as "wakers," so your chip can sleep in a low-power mode until a button is 
    pressed or data arrives, then immediately resume the async task.

**Glommio: The Linux Specialist**

Most executors (like Tokio) are designed to be "portable" across Windows, Mac, and Linux. 
This portability sometimes leaves performance on the table.

* **The Magic:** 
    Glommio is Linux-only. It leverages `io_uring` (a modern, high-speed Linux kernel interface) and a
    "Thread-per-core" model. 

    In this model, data never moves between CPU cores, which eliminates the "cache misses" and "locking" 
    that can slow down multi-threaded executors.

- The "Function Coloring" & Compatibility Problem

There is one big "gotcha" in the Rust ecosystem: **Compatibility.**

Because Rust doesn't have a standard for things like "Async Read" or "Timers," many libraries are 
hard-coded to work with specific executors.

* If you use a library that requires **Tokio** (like the `hyper` web server), it might not work easily on 
  **smol** without a "compatibility layer" (like `async-compat`).

* This has led to a "winner-takes-all" situation where most people just use Tokio to ensure their libraries 
  all play nicely together.

Summary

* **Tokio:** The "Industrial Standard." Use it for 95% of server projects.
* **Embassy:** The "Embedded King." Use it for hardware/microcontrollers.
* **smol:** The "Tiny Alternative." Use it for CLI tools or when you want simplicity.
* **Glommio:** The "Speed Demon." Use it for high-end database or storage work on Linux.



- A comparison of how you would perform a simple async task—reading a file and printing it—using **smol** 
  (the minimalist successor) and **Tokio** (the industry giant).

1. The Code Comparison

Using `smol` (Minimalist)

`smol` is famous for being "runtime agnostic." You can often run a future just by calling a simple function
without a heavy setup macro.

```rust
use smol::fs::File;
use smol::prelude::*;

fn main() -> std::io::Result<()> {
    smol::block_on(async {
        let mut file = File::open("hello.txt").await?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).await?;
        println!("{}", contents);
        Ok(())
    })
}

```

Option B: Using `tokio` (The Standard)

Tokio usually requires the `#[tokio::main]` macro to set up its multi-threaded 
work-stealing engine.

```rust
use tokio::fs::File;
use tokio::io::AsyncReadExt;

#[tokio::main]
async fn main() -> tokio::io::Result<()> {
    let mut file = File::open("hello.txt").await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    println!("{}", contents);
    Ok(())
}

```

2. Key Differences in Behavior

While the code looks similar, the "under the hood" mechanics are quite different.

| Feature | `smol` | `tokio` |
| --- | --- | --- |
| **Setup** | Explicit `block_on` (usually) | Macro-driven `#[tokio::main]` |
| **Threading** | Single-threaded by default | Multi-threaded "Work Stealing" by default |
| **Binary Size** | Very Small | Slightly larger (due to feature set) |
| **Philosophy** | **Exposable:** You can see and touch the executor. | **Abstracted:** The executor runs in the background. |

3. Which one should you pick today?

If you are coming from `async-std`, here is the 2026 decision tree:

- "I want my code to work with every library on Crates.io": Go with **Tokio**. 
  It is the "std library" of async Rust. 
  If you use `reqwest`, `axum`, or `sqlx`, you are already using Tokio.

- "I am building a small tool and I hate 'magic' macros":
  Go with **smol**. It’s incredibly fast for its size and very easy to learn.

- "I need to run async code on a $2 microcontroller":
  Go with **Embassy**. Neither Tokio nor smol is designed for the strict memory limits 
  of embedded hardware.

 4. One Final Concept: `spawn_blocking`

Regardless of the executor you choose, you will eventually hit a wall where you need to do something "slow"
(like resizing a giant image or running a heavy math loop).

Since these executors are **cooperative**, if you run that heavy code inside an `async` function, you stop 
the whole engine. To fix this, both provide a "bridge" to a thread pool:

* Tokio: `tokio::task::spawn_blocking(|| { /* heavy work */ })`
* smol: `smol::unblock(|| { /* heavy work */ })`

This moves the heavy task to a dedicated "blocking thread" so the async executor can keep spinning.


### 2.9 Pinning:

To understand **Pinning**, we have to look at the "secret" life of a Rust Future. 
This is often considered the most difficult part of Rust's async model, but it's the key to how Rust 
achieves its high performance.

1. The Problem: Self-Referential Structs

When you write an `async` function, the compiler turns it into a **State Machine** (an enum). 
If that function has a local variable that is used across an `.await` point, the state machine must store 
that variable.

The problem arises when one var in your `async` func points to **another** variable in the same func.

```rust
async fn my_async_fn() {
    let x = 10;
    let y = &x; // y points to x
    some_other_fn().await; // The function pauses here
    println!("{}", y);
}

```

When this is turned into a struct, `y` holds a memory address of `x`. 
If you **move** that struct (e.g., passing it to another thread), the address of `x` changes, but `y` is 
still pointing to the **old** memory location. 

This is a classic "Use After Free" bug that Rust usually prevents.

2. The Solution: `Pin`

`Pin` is a wrapper that says: **"This data is no longer allowed to move in memory."**

Once a Future is "Pinned," its memory address is guaranteed to stay the same until it is dropped. 
This makes it safe for the state machine to have internal pointers (like `y` pointing to `x`).

* **Unpin:** Most types (like `i32` or `String`) are `Unpin`. They can be moved freely.
* **!Unpin:** Most generated Futures are `!Unpin` because they might contain these self-references.

---

3. The `Poll` Method

This is the low-level heartbeat of every async task. 
While we use `async/await`, the **Executor** sees the `Poll` trait.

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}

```

The Breakdown:

* `self: Pin<&mut Self>`: This ensures the Future cannot be moved while it is being polled.

* `cx: &mut Context`: This contains the **Waker**. It’s the "phone number" the Future uses to tell the 
                      Executor, "Hey, I'm ready to be polled again!"

* `Poll<Self::Output>`: This is an enum with two states:

* `Poll::Pending`: "I'm not done yet, don't call me until the Waker wakes you up."

* `Poll::Ready(val)`: "I'm finished, here is the result."


4. How it all fits together (The Grand Finale)

Imagine a web request:

1. **Executor** calls `poll()` on your Future.
2. The Future tries to read from the network. The data isn't there.
3. The Future registers the **Waker** with the Network Driver (the Reactor).
4. The Future returns `Poll::Pending`.
5. The **Executor** goes to work on something else.
6. **Data arrives!** The Network Driver calls `waker.wake()`.
7. The **Executor** sees the signal, puts the Future back in the "Ready" queue, and calls `poll()` again.
8. This time, the Future returns `Poll::Ready(data)`.

Summary Checklist

* **Fibers/Green Threads:** High-level "managed" threads (Rust used to have these, now it doesn't).
* **Futures:** Low-level state machines that represent a value-to-be.
* **Async/Await:** The syntax that makes state machines easy to write.
* **Pin:** The "glue" that keeps state machines from breaking when they move in memory.
* **Executor (Tokio/smol):** The loop that actually calls `poll`.

You've just covered the entire "Deep End" of Rust Asynchronous programming!

### 2.10 Async program lifecycle:

Complete lifecycle of a Rust asynchronous program, how the different components interact from the async {} to program finish.

#### 1. The Definition Phase (The "Blueprint")

Before the program even runs, you define your logic.

* **The Code:** You write an `async fn` or an `async {}` block.
* The Rust compiler turns this block into a **State Machine**. It calculates all the points where the code might pause (`.await`) 
  and creates a `struct` (the **Future**) to hold the variables needed at each stage.
* Result: You have a "dormant" Future. It does nothing until it is polled.

#### 2. The Spawning Phase (The "Handoff")

To start the work, you must hand the Future to a runtime.

* **Action:** You call `tokio::spawn(your_future)`.
* **The Queue:** The **Executor** (the thread pool) receives this Future and places it into its "Ready Queue."
* **Result:** The Future is now a **Task**, scheduled and waiting for a CPU thread to become available.

#### 3. The Polling Phase (The "Attempt")

This is where execution actually happens.

* **Action:** The **Executor** picks up the Task and calls `poll()`.
* **Execution:** The code runs until it hits an `.await` (e.g., waiting for a network packet).
* **The Yield:** If the data isn't ready, the Future returns `Poll::Pending`.
* **Registration:** The Task registers its **Waker** (a handle to wake it up) with the **Reactor**.
* **Result:** The Executor drops the task from its active list and moves on to help another Task. The thread is not blocked!

#### 4. The Waiting Phase (The "Suspension")

The task is now essentially "parked."

* **The Reactor's Job:** 
  The **Reactor** (the part of the runtime talking to the OS/Hardware) keeps an eye on the specific resource (e.g., a TCP socket).
* **Hardware Event:** The Operating System notifies the Reactor that data has arrived.
* **Result:** The task stays in memory but consumes **zero** CPU cycles during this phase.

#### 5. The Waking Phase (The "Notification")

The bridge between the hardware and the software.

* **Action:** The **Reactor** triggers the **Waker** associated with the suspended Task.
* **Rescheduling:** The Waker tells the **Executor**: "Task #123 has data now; put it back in the Ready Queue."
* **Result:** The Task is moved from the "Waiting" list back to the "Ready" list.

#### 6. The Re-Polling Phase (The "Resume")

* **Action:** The **Executor** eventually picks up the Task again and calls `poll()` a second time.
* **Progress:** Because the data is now available, the State Machine moves past the previous `.await` point and continues to the next part of your code.
* **Result:** The code picks up exactly where it left off, with all its local variables intact.

#### 7. The Completion Phase (The "Ready")

* **Final Poll:** Eventually, the code reaches the end of the block.
* **Result:** The `poll()` method returns `Poll::Ready(value)`.
* **Cleanup:** The Executor removes the Task entirely, and any resources (like memory) are deallocated.

#### Summary of the Cycle

1. **Poll:** Executor tries to run the task.
2. **Pending:** Task realizes it must wait and gives control back.
3. **Wait:** Reactor watches the hardware.
4. **Wake:** Reactor notifies the Executor.
5. **Repeat:** Process repeats until the task is done.

####  Example:
Normally, you'd just use `tokio::time::sleep`, but here we will build one from scratch to show how 
the **Executor**, **Reactor**, and **Waker** shake hands.

```rust 
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::thread;

// 1. THE STATE: This represents our "Reactor" state
struct TimerState {
    completed: bool,
    waker: Option<std::task::Waker>,
}

pub struct MyTimerFuture {
    state: Arc<Mutex<TimerState>>,
}

impl MyTimerFuture {
    pub fn new(duration: Duration) -> Self {
        let state = Arc::new(Mutex::new(TimerState {
            completed: false,
            waker: None,
        }));

        let thread_state = state.clone();
        
        // 2. THE REACTOR MOCK: We spawn an OS thread to simulate 
        // hardware (like a network card or clock) waiting for an event.
        thread::spawn(move || {
            thread::sleep(duration);
            let mut guard = thread_state.lock().unwrap();
            guard.completed = true;
            
            // 3. THE WAKE: If the Executor has registered a Waker, 
            // we trigger it to tell the Executor "I'm ready!"
            if let Some(waker) = guard.waker.take() {
                waker.wake();
            }
        });

        MyTimerFuture { state }
    }
}

// 4. THE FUTURE TRAIT: This is what the Executor calls
impl Future for MyTimerFuture {
    type Output = String;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut guard = self.state.lock().unwrap();

        if guard.completed {
            // If the reactor finished, we return Ready
            Poll::Ready("Timer Finished!".to_string())
        } else {
            // 5. THE REGISTRATION: If not finished, we store the 'Waker' 
            // from the Context so the Reactor knows who to notify later.
            guard.waker = Some(cx.waker().clone());
            
            // Return Pending to let the Executor know the thread is free
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let timer = MyTimerFuture::new(Duration::from_secs(2));
    
    println!("Waiting for the future...");
    
    // .await calls 'poll' initially, then goes to sleep until 'wake' is called
    let result = timer.await; 
    
    println!("{}", result);
}
```
- **Creation:** When we call `MyTimerFuture::new`, we start a separate thread (our "Mock Reactor"). It’s going to wait for 2 seconds.
- **The First Poll:** When the `main` function hits `.await`, the **Tokio Executor** calls `poll()`.
  * The `poll` function sees `completed` is still `false`.
  * It saves the **Waker** into the shared state.
  * It returns `Poll::Pending`.

- **The Yield:** The main thread is now completely free to do other things.

- **The Event:** 2 seconds pass. The "Reactor" thread sets `completed = true` and calls `waker.wake()`.

- **The Re-Poll:** The **Executor** receives the wake signal, sees that `MyTimerFuture` is ready to try again, and calls `poll()` a second time.

- **The Completion:** This time, `poll` sees `completed` is `true` and returns `Poll::Ready`.

Key Takeaway:
Notice that the `poll` function **never waits**. It checks the state and returns immediately. i
This is why async is fast: **no thread ever sits idle inside a function.**

`tokio::select!`: manage multiple futures at once:

`tokio::select!` is the perfect way to see the **Executor** in action as a coordinator. 
It allows you to wait on multiple futures simultaneously and react to whichever one finishes first. 
This is a common pattern for setting timeouts or handling "racing" web requests.

`tokio::select!` Example

In this example, we will race our custom `MyTimerFuture` against a fast-canceling task.

```rust
use std::time::Duration;

#[tokio::main]
async fn main() {
    // We create two "Tasks" (Futures)
    let slow_timer = MyTimerFuture::new(Duration::from_secs(5));
    let fast_timer = MyTimerFuture::new(Duration::from_secs(2));

    println!("Starting the race...");

    // tokio::select! polls both futures. 
    // As soon as ONE returns Poll::Ready, the other is dropped (canceled).
    tokio::select! {
        val = slow_timer => {
            println!("The slow timer won? Logic error! Result: {}", val);
        }
        val = fast_timer => {
            // This block will run because 2s < 5s
            println!("The fast timer won! Result: {}", val);
        }
    }

    println!("The race is over. The executor has cleaned up the loser.");
}

```
-. **Multiplexing:** The Executor doesn't just poll one future; it polls both. If both return `Poll::Pending`, it registers the **Wakers** for both.
-. **The First Signal:** When the 2-second timer's Reactor calls `wake()`, the Executor is notified.
-. **The Winner:** The Executor calls `poll()` on the 2-second timer. It sees `Poll::Ready`.
-. **Cancellation:** This is the most important part—the `select!` macro immediately **drops** the other future (the 5-second one). In Rust, dropping a future is how we "cancel" an async task safely.

Important: The "Blocking" Trap

To truly understand the **Task vs Thread** distinction, look at what happens if you accidentally use a
"Thread" tool inside this "Task" world:

```rust
tokio::select! {
    _ = tokio::time::sleep(Duration::from_secs(2)) => println!("Async sleep won"),
    _ = {
        // WARNING: This is an OS-level sleep!
        // It blocks the actual Worker Thread for 10 seconds.
        // The Executor cannot poll the other future because the thread is "frozen".
        std::thread::sleep(Duration::from_secs(10)); 
        async { println!("Blocking sleep won") }
    }
}

```

> **The Rule of Thumb:** If you are inside an `async` block, never use `std::thread::sleep` or 
> perform heavy file/CPU work without using `tokio::task::spawn_blocking`. 
> If you block the thread, the **Executor** cannot perform its cycle of polling and waking other tasks.

----------------
## Roadmap till writing a custom executor:

Building a custom executor is the ultimate "rite of passage" for mastering Rust's `async`. It forces you to stop seeing `async/await` as magic and start seeing it as a coordination game between queues and pointers.

Here is the 4-stage roadmap to get you from "Using Tokio" to "Building a Mini-Executor."

---

## Stage 1: The "Manual Poll" (Understand the Heartbeat)

Before building a system to manage tasks, you must understand how to drive a single one manually.

* **Goal:** Drive a `Future` to completion without using `.await` or a macro.
* **Key Skills:**
* Implementing a simple `Future` trait for a struct.
* Learning to use `std::task::Context` and `Poll`.
* **The Challenge:** Use `Waker::noop()` (a waker that does nothing) to manually call `poll` in a `while` loop until it returns `Ready`.
* **Concept to Master:** The difference between a Future that is "Done" vs "Waiting."



---

## Stage 2: The Waker & Queue (The Feedback Loop)

An executor is essentially an infinite loop that processes a **queue of tasks**.

* **Goal:** Build a mechanism where a task can put *itself* back on the queue when it’s ready.
* **Key Skills:**
* Using `std::sync::mpsc` (channels) to create a "Ready Queue."
* Wrapping a `Future` in a `Task` struct that lives in an `Arc`.
* **Implementing `ArcWake`:** This is the easiest way to create a `Waker`. You define a `wake` function that simply sends the `Task` back into the channel.
* **The Challenge:** Create a "Timer" future that spawns a thread, sleeps, and then calls `.wake()`.



---

## Stage 3: Pinning & Memory Safety (The Guardrails)

Real futures (like those generated by `async` blocks) cannot be moved in memory. Your executor must handle this.

* **Goal:** Upgrade your executor to handle "Stackless Coroutines" (the ones the compiler makes).
* **Key Skills:**
* Using `Box::pin` to lock your futures in one memory location.
* Understanding why `Pin<Box<dyn Future>>` is the standard way to store a task in a list.
* **Concept to Master:** Self-referential structs and why moving a future after its first `poll` causes a crash.



---

## Stage 4: The Reactor (Handling the Real World)

A "Small Executor" is useless if it only runs timers. It needs to handle I/O (Networking/Disks).

* **Goal:** Connect your executor to the OS (epoll/kqueue/IOCP).
* **Key Skills:**
* Learning about the **Reactor Pattern**: The "Assistant" that watches file descriptors.
* Integrating with a crate like `mio` (Metal I/O).
* **The Challenge:** Write a small loop that polls a TCP stream. When the OS says "Data is here!", find the corresponding `Waker` and trigger it.



---

## Recommended Learning Path (The "Curated" Resources)

If you follow these in order, the "magic" will disappear:

1. **The "Async Book" (Applied Section):** Read the chapter **["Applied: Build an Executor"](https://rust-lang.github.io/async-book/02_execution/04_executor.html)**. It is the gold standard for this exact goal.
2. **"Mini-Tokio":** Check out the [Tokio tutorial's Mini-Tokio implementation](https://tokio.rs/tokio/tutorial/bridging). It’s about 100 lines of code and shows how a production executor starts.
3. **The `futures` crate:** Look at the source code for `futures_executor::block_on`. It is the simplest "executor" possible.

**Would you like me to provide a 15-line "Pseudo-Executor" code snippet right now to show you the basic `while` loop structure?**
