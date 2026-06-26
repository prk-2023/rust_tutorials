# Samply:


## Intro:

If you have used profilers like `perf`, `Instruments` or `perf record`, `Samply` is best thought as a modern
CLI frontend that makes CPU profiling much easier and gives you a rich interactive UI via browser ( the
Firefox profiler interface )

---
Note: Firefox Profiler: 

Its a powerful web based tool developed by Mozilla that helps developers analyze the performance of their
web applications, websites and even the Firefox browser. Its a High precision microscope for the code,
records exactly what the browser is doing down to milliseconds, allowing to detect lags, stuttering or
consuming too much memory. 

Firefox profiler is split into two parts:
1. Recording ( the firefox core ): Recording phase is built directly into the browser's engine, which
   samples every millisecond and takes snapshorts. 

2. Visualizing (`profiler.firefox.com`)
This is the frontend interface: This is separate process that runs in a performance constrained env of the
tab. Visualizer takes the massive JSON file, parses it and turns it into the interface UI elements (
Timeline, Flame Graphs, Call tress ).  

3. So this Visualizer ( `profiler.firefox.com`) is an independent web app that ingests JSON. This allows it
   be used with other applications. And `Samply` uses this as its interface, 

4. `Samply`: Its developed by the Mozilla developers and systems programmers written in Rust. Which is
   specifically designed to use the Firefox profiler as its front end. 

5. `Samply`: can be used to profile native applications ( C, C++ , Rust, Python ...) from the terminal. 

6. This modular design allows to profile a wide range of applications: from applications, game engines and
   server-side databases. 

---

- `Samply` is a cli that helps to understand how it profiles, instead of injecting code into every single
  function to time it ( which slows down the application ).

- `Samply` uses **Sampling**, every few milliseconds it pauses the program looks at the call stack to see what
  functions is currently running and record it. Over time this snapshorts create a highly accurate
  statistical picture of the application with almost zero overhead. 

### Key features:

- Generally CPU profilers often involve multiple steps: 
    - Record a trace 
    - Convert or symbolize it 
    - open it in another tool 
    - try to understand stacks. 

  `Samply` compresses this into essentially one command: 
  `samply record -- targer_program`

  When the program finishes, `samply`: 
    - Records CPU samples 
    - resolves symbols 
    - starts a small local web server 
    - Opens an interactive profiler in the Firefox Profiler UI.

--- 

## Sampling Profiling: 

Unlike instrumentation profilers, Samply doesn't insert timers into every function: 

Instead it repeatedly asks: 

> "What is the CPU executing right now?"

Example:

```bash 
Sample 1
main
 └── load_data
      └── parse_json

Sample 2
main
 └── load_data
      └── parse_json

Sample 3
main
 └── load_data
      └── parse_json

Sample 4
main
 └── compute_hash
```

After thousands of samples, you might see a statistical picture of where the CPU time is being spent and
with much lower overhead than tracing every function call:

```text 
parse_json      65%
compute_hash    20%
malloc           8%
other            7%
```

## Typical work flow: 

- Append the below to `Cargo.toml`:

```toml 
[profile.release]
debug = true             # Preserves symbols so profilers show function names
debug-assertions = false # Disables overflow checks to keep production-level speed
```

or 
```toml 

[profile.release]
debug = "line-tables-only" # 
```

This is a modern way if you are worried about the release binary becoming massive or if you require a
faster compilation time.
`debug = "line-tables-only` gives enough information to know which functions and lines are running with
out bloating the binary with extra debugging data you don't need for profiling.

`cargo build --release` 


- Profile the application `samply record target/release/test_app`

- `samply` will do the below operations:

```text 
    Launch program
          ↓
    Collect samples
          ↓
    Program exits
          ↓
    Open browser
          ↓
    Interactive profile
```

- The browser view includes:
    - Flame Graph 
    - Call tree 
    - Timeline 
    - thread list 
    - source code ( when symbols are available as enabled in `Cargo.toml`)
    - assembly view ( for native code )


### `perf_event_paranoid`:

- `samply` relies on linux kernels underlying `perf` subsystem to inspect CPU is doing.

- And generally as default `/proc/sys/kernel/perf_event_paranoid` blocks .

  `/proc/sys/kernel/perf_event_paranoid`: is a setting that controls who can use performance monitoring
  tools and what they are are allowed to see. Since profiler has to peek deep into the CPU registers and
  memory addresses to see what functions are running, and Linux treats it as a potential security risk. 

- setting can scale from -1 to 3 ( more paranoid = more restrictive )
    - `2`: ( default ) High security, Unprevileged user cannot sample CPU events at all.
    - `1`: Moderate security Unprivileged users can sample CPU events, but only for their own processes.
           They cannot spy on other users or kernel-level activities.
    - `0` or `-1`: Very low security. Allows unprivileged users to sample almost anything, including raw
      kernel data. (Not recommended unless you are debugging the Linux kernel itself).


## Sample to profile a process that is already running :

- `samply` can be attached to a running process:

    ```bash 
    samply record --pid 12345
    ```

  This is useful for:
    - servers 
    - game engines 
    - GUI applications 
    - Long-running services 

### reading Flamegraph:

- suppose flame graph shows: 
```text 
    ████████████████ parse_json
    ██████           serde_json
    ██               malloc
    █                memcpy
```
  This means:
    - Width represents how often the function appeared in samples.
    - Height represents the call stack depth, not time.

A very wide box is usually a good optimization target.

### Threads: 

- One strength of `Sampl`y is that it records multiple threads automatically.

  Example :
  ```
      Main Thread
      Render Thread
      Worker 1
      Worker 2 
      GC Thread
  ```

- You can switch between threads or inspect them together. This makes it useful for multithreaded apps. 

## Python support:

- Samply can profile Python too. ( With newer versions of Python) :

```bash 
    samply record PYTHONPERFSUPPORT=1 python myscript.py
```
  Or for python builds that support it:

```
  python -X perf myscript.py 
```
  This allows python function names to appear in the profiler instead of only native interpreter frames. 

### Rust support:

- Rust works well because debug symbols are readily available.

Example:
```bash 
cargo build --release

samply record target/release/myapp
```

No code changes are required.


## How it compares with other profilers

| Tool                          | Best for                                     | Interface                  |
| ----------------------------- | -------------------------------------------- | -------------------------- |
| Samply                        | General CPU profiling with an interactive UI | Browser (Firefox Profiler) |
| `perf`                        | Low-level Linux performance analysis         | Terminal                   |
| `Instruments`                 | macOS development                            | Native macOS GUI           |
| `perf record` + `perf report` | Linux experts                                | Terminal                   |
| `gprof`                       | Older instrumentation-based profiling        | Text output                |

Samply often serves as a friendlier front end while still leveraging platform profiling capabilities under
the hood. 

On Linux, for example, it can also import `perf.data` files for visualization. 
[Firefox Source Docs](https://firefox-source-docs.mozilla.org/performance/jit_profiling_with_samply.html?utm_source=chatgpt.com)


## Common commands

Record a program:

```bash
samply record -- ./test_prog
```

Record a Cargo project:

```bash
samply record cargo run --release
```

Attach to a process:

```bash
samply record --pid 1234
```

Import an existing `perf` profile:

```bash
samply import perf.data
```


## When should you use Samply?

Samply is a strong choice when you want to answer questions like:

* "Which function is using the most CPU?"
* "Why is my Rust program slow?"
* "Which thread is busy?"
* "Where is my Python application spending time?"
* "What part of my workload should I optimize first?"

Because it uses statistical sampling rather than instrumenting every function call, it generally has
relatively low overhead while still providing detailed, interactive views of execution. For many Rust,
C/C++, and Python workflows, it's an excellent first profiler to reach for before moving to more specialized
tools. 


## Firefox Profiler Interface 

Firefox profiler interface used by Samply is an incredibly powerful, multi-faceted tool. When `samply` opens 
web browser, it breaks down your performance data into several distinct tabs and panels. 

 Each panel looks at the exact same execution data but slices it differently to help spot various types of 
 performance bugs.

 1. Stack Chart (Timeline):
    - The Stack Chart (often located in the top panel as part of the timeline) is a chronological,
      left-to-right view of your program's execution.

    - The horizontal axis ($X$) is linear time. The vertical axis ($Y$) represents the depth of the call
      stack at that exact millisecond.

    - A tall spike in the view, means the program dug deep into nested function calls (e.g., main() called 
      run(), which called parse(), which called read_char()).
      If you see a long, flat plateau, it means your program was stuck inside a single function (or a
      shallow loop) for a long time.

    - This stack chart is best for spotting phases of execution and stuttering. 
      Ex: "What was the app doing 2 seconds after it started?" or "Why did the program freeze for 200ms
           right in the middle of execution?"

2. Flame Graph:
    - Flame Graph aggregates all the sampled data across the entire run (or a selected time range) to show
      you where the most cumulative time was spent.

    - Unlike the Stack Chart, the horizontal axis is NOT time. Instead, the width of a box represents the
      percentage of total CPU time spent in that function. The functions are stacked vertically ($Y$) based 
      on ancestry (caller on the bottom, callee on top).

    - Look for the widest boxes at the top of the "flames." If a function has a very wide box, it means the
      CPU spent a massive chunk of its life inside that function (or its children).

    - This is best to identifying heavy resource hogs. It strips away the concept of when things happened
      and answers: "In total, which functions took up 80% of my application's processing power?"

3. Call Tree: The Call Tree is a text-based, hierarchical spreadsheet of your program's execution paths,
   organied from the top-down (starting at main or thread roots).

   It represents a nested list of function calls accompanied by statistical metrics, primarily Running Time,
   Self Time, and Total Time:

    - Total Time (or Total %): The time spent in this function plus all the other functions it called.

    - Self Time (or Self %): The time spent exclusively inside this specific function's own code, excluding
      any child functions it invoked.
    
   The Rows can expand (like folders in a file explorer) to follow the "Hot Path" the trail of highest 
   percentages. If main takes 100%, and expands into process_data taking 90%, you follow process_data. 
   If you suddenly find a function with low Total Time but high Self Time, you've found the exact function 
   containing a slow loop or heavy math operations.

   This is best used for precision engineering and code-level tracking. To answers: "What specific line or 
   function is responsible for the bottleneck I saw in the Flame Graph?"

4. Marker Chart: ( Markers sometimes called "Events") represent discrete, non-sampling occurrences that
   happened during the program's lifecycle, such as I/O operations, garbage collection, thread 
   synchronization, or file access.

    - It represents a timeline overlay showing individual events as colored blocks or icons. A marker can be
      a point in time (e.g., a user click) or a duration (e.g., waiting 15ms for a hard drive read to
      finish).

    - How to read it: It runs parallel to your Stack Chart. If your CPU usage drops to 0% (a valley in
      Stack Chart) but you see a long, solid bar in the Marker Chart labeled File I/O, it means the program
      wasn't burning CPU because it was trapped waiting for the operating system to read a file.

    - Marker charts are best used for: Diagnosing blocking issues and system overhead. It can answer: 
      "Is my app slow because my Rust code is poorly optimized, or because it is waiting on the disk, 
      network, or OS memory allocation?"

5. Marker Table: The Marker Table takes all the visual data from the Marker Chart and flattens it into a 
  searchable, sortable list.

  - It represents a tabular log of every single event marker recorded during the profile session. Columns
    typically include Marker Name, Start Time, Duration, and Details (like the specific file path being 
    read or the network socket being used).

  - To read it you can sort by the "Duration" column to instantly surface the longest-running pauses, or 
    use the search bar to filter for specific types of events (like Mutex lock or Network).

  - It's best used for: Quantitative analysis of non-CPU bottlenecks. It answers: "Exactly how many times 
    did my app request memory from the OS, and what was the maximum latency of those requests?"


Recap: When optimizing a application you will usually use them in a workflow sequence:
1. Look at the Stack Chart to find a lag spike or a wide plateau of interest.
2. Highlight that specific time region to filter your data.
3. Switch to the Flame Graph to visually spot which function is taking up the most horizontal width in that
   time frame.
4. Move to the Call Tree to pinpoint the exact function and its Self Time percentage to see if the code
   itself is slow.
5. Check the Marker Chart/Table to ensure the slowness isn't actually caused by your program waiting on 
   external things like terminal printing (println!) or file reading.
