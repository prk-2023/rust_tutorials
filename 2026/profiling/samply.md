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




