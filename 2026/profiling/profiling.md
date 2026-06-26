# The Comprehensive Guide to Profiling Rust Applications:


Ref:
- https://nnethercote.github.io/perf-book/profiling.html
- https://oneuptime.com/blog/post/2026-01-07-rust-profiling-perf-flamegraph/view

Rust CPU profiling with simply:
- https://hotpath.rs/cpu_profiling
- https://docs.wasmtime.dev/examples-profiling-samply.html


## 1. Introduction & Mindset

- Learning to profile Rust applications is less about Rust-specific "tricks" and more about understanding
  performance analysis in systems programming. 

- Because Rust compiles directly to machine code, you get access to the same powerful low-level profiling
  ecosystem used for C/C++, plus modern, ergonomics-focused tools designed specifically for the Rust
  ecosystem.

Before touching a tool, you must establish what you are trying to measure:


|Target |Question |Type of Profiling Key Metrics to Track|
| :--- | :--- | :--- | 
| Why is my program slow? | CPU Profiling |"Execution time, CPU cycles, hotspots" |
| Why is memory usage high? | Heap Memory Profiling |"Peak heap size, allocation location, memory leaks" |
| Why are allocations frequent? | Allocation Profiling | "Total allocation count, transient/short-lived objects" |
| Why are threads waiting? | Concurrency Profiling | "Lock contention, synchronization overhead, context switches" |
| Why is async performance poor? | Async Runtime Profiling | "Task block time, task yield frequency, executor health" |
| Why is latency inconsistent? | Tracing & Observability | "Request timelines, P99 latency spikes, I/O wait" |


> ⚠️ The Golden Rule: Never optimize without measuring first. 
> Intuition about performance bottlenecks is frequently wrong in heavily optimized systems languages.

--- 

## 2. Compiler Setup: Preparing for Profiling

By default, standard cargo build modes are useless for profiling:

- **Debug** builds (cargo build): Can be 10–100× slower, leading you to optimize overhead that completely
  disappears in production.

- **Release** builds (cargo build --release): Strip debug symbols (panic locations, line numbers, and function names), leaving your profiler showing raw hex memory addresses.

### The Recommended Setup:

So the Recommended setup for profiling should be to include the symbols to release build:

- Add this specific configuration to `Cargo.toml`.  It forces Cargo to keep debugging symbols while applying
  maximum release optimizations, all without impacting execution performance:

```toml
[profile.release]
debug = true             # Preserves symbols so profilers show function names
debug-assertions = false # Disables overflow checks to keep production-level speed
```

---

## 3. CPU Profiling & Execution Visualizers

CPU profiling samples your application at high frequencies to discover which functions take up the most
execution cycles.

### A. System-Level Tools (Linux native)

- `perf` is the golden standard for Linux. 
- `perf` samples your program directly via kernel and hardware counters.

```bash 
#Capture data while your app runs a typical workload
perf record --call-graph dwarf ./target/release/myapp

#View the interactive report inside your terminal
perf report
```

**Tip** : 
- Use `perf report | rustfilt` to demangle long, cryptic compiled Rust function names into readable code.

To diagnose low-level architectural issues (cache line bouncing, branch mispredictions):

```bash 
perf stat ./target/release/myapp
```

### B. Flamegraphs

`Flamegraphs` turn your profiler data into an interactive visual map. The wider a box is in a flamegraph,
the more CPU time that function (and its children) consumed.

```bash 
# Install the cargo subcommand
cargo install flamegraph

# Run your application and automatically generate 'flamegraph.svg'
cargo flamegraph --bin myapp
```

**Common Rust Hotspots to look for**: 
- Excessive HashMap hashing operations, redundant String allocation/parsing inside loops, or deep serde
  serialization trees.

### C. Modern Alternative: samply (macOS & Linux)

`samply` is one of the cleanest, lowest-overhead sampling profilers available for Rust development.

```bash 
cargo install samply
samply record ./target/release/myapp
```

It records the execution and opens a local web-server utilizing the beautiful Firefox Profiler interface,
allowing you to easily browse call trees, sample timelines, and thread allocations.

---

## 4. Micro-Benchmarking Functions

When isolating specific algorithms, parsing logic, or critical routines, rely on a statistics-driven
benchmarking harness rather than manual timers.

`Criterion.rs`

- criterion handles CPU warmups, filters out OS background noise, detects performance regressions, and
  generates charts.

```toml 
# Cargo.toml
[dev-dependencies]
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "my_benchmark"
harness = false
```

Create a benchmark file under benches/my_benchmark.rs:

```rust 
use criterion::{criterion_group, criterion_main, Criterion};

fn bench_sort(c: &mut Criterion) {
    let mut data = vec![5, 3, 1, 4, 2];
    c.bench_function("sort_vectors", |b| {
        b.iter(|| data.sort())
    });
}

criterion_group!(benches, bench_sort);
criterion_main!(benches);
```
- Run benchmarks with: `cargo bench`

--- 
## 5. Allocation & Heap Memory Profiling

Frequent allocation triggers system calls and allocator locks, dragging down throughput even if CPU
utilization looks healthy.

### A. `dhat` (The dhat-rs crate)

Rather than installing the heavy system-wide Valgrind suite, you can use dhat directly inside your Rust code
natively. It tracks heap usage, peak memory spikes, and flags short-lived ("transient") allocations.

```toml 
# Cargo.toml
[dependencies]
dhat = "0.3"
```

Integrate it directly into your `main.rs`:

```rust 
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    // Start the profiler; it hooks the global allocator
    let _profiler = dhat::Profiler::new_heap();

    // Your application code here
    execute_app_logic(); 
    
    // When _profiler drops at the end of main, it writes 'dhat-heap.json'
}
```
Open the generated file using the [DHAT Viewer](https://www.google.com/search?q=https://valgrind.org/gviewer) website to analyze your memory hotspots.

### B. heaptrack (Linux)
heaptrack explicitly instruments memory allocation tracking without severely slowing down application runtime performance.

```bash
heaptrack ./target/release/myapp
heaptrack_gui heaptrack.myapp.XXXX.gz
```

## 6. Tracing, Asynchronous, and Distributed Systems
Standard CPU sample profilers fall apart when dealing with async Rust (async/await) because tasks constantly yield to the runtime executor, shattering call stacks across threads.

### A. The tracing Ecosystem
Instead of intermittent print statements, structure your observability using structured events and spans.

```toml 
# Cargo.toml
[dependencies]
tracing = "0.1"
tracing-subscriber = "0.3"
```

```rust 
use tracing::{info, span, Level};

#[tracing::instrument] // Automatically creates a span for this execution block
async fn process_request(id: u64) {
    info!(id = id, "Processing step initiated.");
    // Async yield points are correctly tracked here
}
```

### B. Tokio Console (Async-Specific Diagnostics)
For heavy tokio multi-threaded applications, tokio-console is an interactive real-time dashboard resembling top but built for async tasks.

1. Enable the tracking feature flags in your dependency configuration:

```toml 
[dependencies]
tokio = { version = "1", features = ["full", "tracing"] }
console-subscriber = "0.4"
```

2. Initialize the console subscriber at the very top of your main function:

```rust 
fn main() {
    console_subscriber::init();
    // ...
}
```

3. Run your application with special compiler flags enabled:

```bash
 RUSTFLAGS="--cfg tokio_unstable" cargo run --release
```

4. In another terminal window, launch the UI listener to watch tasks yield, identify long poll times, and catch blocked executors:

```bash 
tokio-console
```

## 7. Advanced: System-Wide and Low-Overhead Production Tools

### A. Callgrind & KCachegrind

When execution counts must be deterministic (e.g., counting the exact number of instruction executions
independent of execution timing variations):

```bash 
valgrind --tool=callgrind ./target/release/myapp
kcachegrind callgrind.out.<pid>
```

### B. Production eBPF Profiling
In live cloud-native production environments, traditional profilers cause too much application lag. Modern
deployment setups rely on eBPF (Extended Berkeley Packet Filters) to capture highly accurate sample data
with practically zero overhead.

Tools to consider: bpftrace, Parca, and Pyroscope.

### C. Profile-Guided Optimization (PGO)
Once you have collected data paths, you can feed performance logs back into the Rust compiler. The compiler
will re-optimize its binary layout based on real-world execution paths.

```bash 
# 1. Install standard PGO instruments
cargo install cargo-pgo

# 2. Build with instrumentation hooks
cargo pgo build

# 3. Run a representative workload to generate data profiles
./target/x86_64-unknown-linux-gnu/release/myapp

# 4. Recompile utilizing the collected data profile for optimized binaries
cargo pgo optimize
```

---
# simply :

Ref:
- https://github.com/mstange/samply
- https://profiler.firefox.com/docs/#/



`samply` operates with your source code and how it processes data:

---

## 1. Is the source code strictly mandatory to run `samply`?

**No, it is not mandatory to run the profiler, but it is mandatory to see the code *inside* the browser interface.**

To clarify how `samply` resolves your code, understand that there are two distinct parts:

* **Debug Symbols (Mandatory for Names):** When you compile with `debug = true`, the compiler embeds mapping
  data (function names, file paths, line numbers) into your binary. **`samply` requires this.** Without it,
  you will only see memory addresses (e.g., `0x7fff81a03f4`).
* **Source Code Files (Optional for UI):** `samply` reads the file paths stored in those debug symbols and
  looks for those `.rs` files on your hard drive.
* If the source code files *are* present, `samply` serves them locally so you can click a function name and
  view the lines of code with their execution counts.
* If you run `samply` on a compiled binary *without* the source files nearby (for example, profiling a
  binary on a server where the source isn't cloned), **it will still work perfectly**. The timelines, flame
  graphs, and function names will display normally; you just won't be able to look at the inline source code
  view.



---

## 2. Does it profile in real time, or only after the application stops?

**It captures data in real time, but processing and visualization happen strictly *after* the application stops.**

`samply` follows a **"Record Now, Analyze Later"** model.

### Why it works this way:

1. **Zero-Overhead Philosophy:** Real-time web streaming requires a lot of CPU cycles and memory allocations
   just to format and transmit the data. If a profiler did this in real time, it would drastically distort
   your application's actual performance (a phenomenon known as the *observer effect*).
2. **The Workflow:** While your application runs, `samply` quietly and quickly streams raw, compact binary
   data (the stack samples) into a temporary local file or RAM buffer.
3. **The Trigger:** The second your application calls `std::process::exit`, finishes its execution, or you
   stop it with `Ctrl+C`, `samply` intercepts the exit, processes the recorded data chunk, starts its local
   symbol server, and pops open the Firefox Profiler browser tab.

### If you need real-time updates:

If your use case absolutely demands watching a live dashboard while the code is actively executing (for
example, keeping an eye on a live web server), you should bypass `samply` and use **`tokio-console`** or a
dedicated tracing dashboard. They are instrumented specifically to stream diagnostic events out of a running
application in real time.
