# Asynchronous Programming:


## Introduction:
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

### Key Concepts and Benefits:

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

### Definitions:

- Synchronous: Tasks happening one after another. Is task A is slow database call, then task B gets
  blocked till task A finishes. 


- Asynchronous: Programming model where you can start a task and then move on to something else before
  that task finishes. It's about non-blocking flow.

- Concurrency: Its about dealing with many things at once. It's scheduling property. On a Single-code
  processor you have concurrency by rapidly switching between tasks ( making progress on all , but only
  running one task at a time).

- Parallelism: Doing many things at once. This requires multiple CPU cores physically running different
  pieces of code at the exact same moment. 


### Bottlenecks: 

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


## History:

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


