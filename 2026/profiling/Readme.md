# Profiling:


## 0. Introduction 
- Learning to profile Rust applications is less about Rust specific tricks and more about understanding
  performance analysis in systems programming. 
- Rust gives you access to the same profiling ecosystem used for C/C++, plus a few Rust-focused tools.


## 1. Start with a profiling mindset

Before choosing tools, identify what you're trying to measure:

| Question                      | Type of profiling       |
| ----------------------------- | ----------------------- |
| Why is my program slow?       | CPU profiling           |
| Why is memory usage high?     | Memory profiling        |
| Why does startup take long?   | Startup/trace profiling |
| Why are threads waiting?      | Concurrency profiling   |
| Why are allocations frequent? | Allocation profiling    |
| Why is latency inconsistent?  | Tracing and flamegraphs |


A common mistake is optimizing without measuring first.

---

## 2. Build in release mode

Always profile optimized builds:

```bash
cargo build --release
cargo run --release
```

Debug builds can be 10–100× slower and produce misleading results.

For profiling, many developers use:

```toml
[profile.release]
debug = true
```

This preserves symbols so profilers can show function names.

---

## 3. CPU Profiling

### Linux: perf

The standard Linux profiler is:

```bash
perf record ./target/release/myapp
perf report
```

This samples the program while it runs.

Useful commands:

```bash
perf stat ./myapp
```

Shows:

* CPU cycles
* instructions
* cache misses
* branch misses

This helps answer:

* Is code CPU-bound?
* Is cache behavior poor?

---

### Flamegraphs

Flamegraphs are one of the best ways to learn profiling.

Install:

```bash
cargo install flamegraph
```

Run:

```bash
cargo flamegraph
```

Generates:

```text
flamegraph.svg
```

The widest blocks are where the program spends most time.

Example insights:

* HashMap lookups dominate
* String parsing dominates
* Serialization dominates

Project:

[cargo-flamegraph repository](https://github.com/flamegraph-rs/flamegraph?utm_source=chatgpt.com)

---

## 4. Benchmarking Individual Functions

Use Rust's benchmarking ecosystem.

### Criterion

Most popular Rust benchmarking library.

```toml
[dev-dependencies]
criterion = "0.5"
```

Example:

```rust
fn bench_sort(c: &mut Criterion) {
    c.bench_function("sort", |b| {
        b.iter(|| {
            my_sort();
        })
    });
}
```

Run:

```bash
cargo bench
```

Criterion provides:

* statistical analysis
* variance detection
* regression detection
* charts

Project:

[Criterion.rs](https://bheisler.github.io/criterion.rs/book/index.html?utm_source=chatgpt.com)

---

## 5. Allocation Profiling

Sometimes CPU isn't the issue.

You may be allocating excessively.

### DHAT

Part of:

[Valgrind](https://valgrind.org?utm_source=chatgpt.com)

Run:

```bash
valgrind --tool=dhat myapp
```

Shows:

* allocation counts
* allocation hotspots
* heap growth

---

### heaptrack

Very popular memory profiler.

Project:

[heaptrack](https://github.com/KDE/heaptrack?utm_source=chatgpt.com)

Run:

```bash
heaptrack ./myapp
```

Shows:

* who allocated memory
* lifetime of allocations
* allocation frequency

---

## 6. Rust-specific Allocation Tracking

A useful crate:

```toml
dhat = "0.3"
```

Example:

```rust
let _profiler = dhat::Profiler::new_heap();
```

Run:

```bash
cargo run --release
```

Produces detailed heap reports.

Project:

[dhat-rs](https://github.com/nnethercote/dhat-rs?utm_source=chatgpt.com)

---

## 7. Tracing and Instrumentation

For services and async applications, tracing is often more valuable than CPU profiling.

## tracing

```toml
tracing = "0.1"
```

Example:

```rust
#[tracing::instrument]
async fn process_request() {
}
```

This allows visibility into:

* request latency
* async tasks
* execution flow

Project:

[tracing crate documentation](https://docs.rs/tracing/latest/tracing/?utm_source=chatgpt.com)

---

### Chrome Trace Viewer

Generate traces:

```json
{
  "ph": "X"
}
```

Open in:

[Chrome Tracing Viewer](https://ui.perfetto.dev?utm_source=chatgpt.com)

Useful for:

* async runtimes
* thread scheduling
* request timelines

---

## 8. Profiling Tokio Applications

If you're using Tokio:

Look for:

* blocked executors
* excessive task spawning
* lock contention

Tools:

* tracing
* tokio-console

Project:

[tokio-console](https://github.com/tokio-rs/console?utm_source=chatgpt.com)

Features:

* live task inspection
* polling frequency
* task wait times
* executor health

---

## 9. System-Wide Profiling

Sometimes your Rust code is fine.

The problem may be:

* disk I/O
* network I/O
* scheduler delays
* page faults

Tools:

### Linux

* `perf`
* `iostat`
* `vmstat`
* `pidstat`
* `strace`

### macOS

* Instruments

### Windows

* Windows Performance Analyzer

---

## 10. Advanced Profiling

## Callgrind

Part of Valgrind.

```bash
valgrind --tool=callgrind ./myapp
```

Visualize with:

KCachegrind

Excellent for:

* call trees
* instruction counts
* algorithm analysis

---

### eBPF Profiling

Modern Linux systems increasingly use eBPF.

Tools include:

* bpftrace
* Parca
* Pyroscope

Useful in production environments because overhead is often low.

---

## Recommended learning path

#### Week 1: Basics

1. Learn release builds.
2. Learn `perf stat`.
3. Generate flamegraphs.
4. Profile a toy CPU-heavy application.

#### Week 2: Benchmarks

1. Learn Criterion.
2. Benchmark:

   * sorting
   * parsing
   * hashing
3. Compare implementations.

#### Week 3: Memory

1. Learn heaptrack.
2. Learn DHAT.
3. Find allocation hotspots.

#### Week 4: Async/Services

1. Learn tracing.
2. Learn tokio-console.
3. Analyze a small web service.

#### Week 5+: Production Profiling

1. Learn eBPF tools.
2. Learn continuous profiling.
3. Profile real workloads under load.

### A practical exercise

Build a simple Rust application that:

* Reads a large JSON file
* Parses it with serde_json
* Stores data in a `HashMap`
* Performs searches

Then:

1. Benchmark with Criterion.
2. Generate a flamegraph.
3. Profile allocations with heaptrack.
4. Add tracing spans.
5. Compare before/after optimizations.

This single project will expose you to most of the profiling workflow you'll use in real Rust backend,
systems, and CLI applications.
