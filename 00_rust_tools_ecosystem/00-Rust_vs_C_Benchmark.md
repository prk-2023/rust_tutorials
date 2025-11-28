Limitations of standard C vs. Rust benchmarks.

The core argument for Rust's design philosophy: 
    The safety features are largely "zero-cost abstractions," 
    which means they impose minimal to no runtime overhead compared to C's approach.
    
And a true fair comparison would be to compare Idiomatic Rust code against Security-hardened C code. 

Std C lang provides minimal built-in safety, allowing compilers to make aggressive
optimizations that assume the programmer has written correct code
(i.e., code free of Undefined Behavior), this is one reason C appears to be faster in
simple benchmarks.

Rust on the other hand provides its safety guarantees in 2 main ways:
1. Compiler-time safety: Borrow Checker prevents entire class of error ( data races and
   use-after-free ) at compilation time imposing zero runtime cost.

2. Runtime Safety ( This addes overhead ) Features like bounds checking on array/slice 
   accesses are performed at runtime. This is the primary source of Rust's performance 
   difference in micro-benchmarks.
   
A Fair comparison would be:
Hardening C: Programs ( benchmark ) should manually add checks or relay on compiler/lib 
features, which introduce overhead. Some key manual additions in C that correspond to Rust
runtime checks are:

1. Slice Bounds checking: Manually checking the length before every array access. 

    if (index < length ) {
        array[index]; 
    } else {
        /* Crash on Error */
    }

2. Integer Overflow: Checking 
   if (a > INT_MAX - b) before addition, 
   or relying on -fwrapv (which limits optimization).
   
3. Safe string handling: 
   Using functions like strlcpy (if available) or manually tracking buffer sizes, 
   instead of unsafe functions like strcpy
   
Adding this extra constrains to make C more safe will decrease its performance edge it had over idiomatic
Rust code.

Compiler Hardening Flags:
In professional C environments, developers often enable compiler hardening flags to
catch or mitigate many security issues. These flags enforce security checks that 
add runtime overhead:
1) Address Sanitizer (-fsanitize=address): 
  Detects memory errors like buffer overflows and use-after-free. (Significant Overhead)
2) Undefined Behavior Sanitizer (-fsanitize=undefined): 
  Detects issues like integer overflow and misaligned pointers. (Moderate Overhead)
3) Stack Smashing Protection (-fstack-protector): Adds a canary value to the stack to
  detect buffer overflows. (Minimal Overhead)
  
If the C programs in the benchmark were compiled with these hardening flags, the
performance difference relative to Rust would likely shrink or even reverse in some cases.

Benchmarks:  ( http://www.roylongbottom.org.uk/ ) and on his youtube channel.

• Dhrystone — pointer-heavy, string-heavy, logic-heavy
• Whetstone — floating-point and math operations
• MemSpeed — memory throughput, scaling, and stalls
• Mandelbrot — algorithmic complexity and numerical accuracy
• Livermore Loops (coming soon) — the ultimate compiler stress test

These are excellent for measuring raw instruction performance and compiler optimization 
for specific algorithms, but they are incomplete for assessing the practical performance 
of safely written code.

So standard C generally Wins in micro-benchmarks by relying on the programmer to ensure
correctness and avoiding runtime checks.

Idiomatic Rust is generally on par with or faster than Security-Hardened C because 
Rust's core safety (the Borrow Checker) is handled at compile time, leaving only 
necessary checks (like array bounds) at runtime.

=> In real-world apps with complex data structures and concurrency, 
   Rust often outperforms C because its language guarantees allow the compiler to make stronger 
   optimizations (like enabling LLVM's noalias optimization) and make it easier for the programmer to write 
   efficient, concurrent code without introducing subtle bugs.
 
----------------------------------------------------------------------- 
Or:
Use `unsafe` Rust to disable the safety checks that primarily account for the minor 
runtime overhead relative to C.

`unsafe` Rust disables the safety checks that primarily account for the minor runtime 
overhead relative to C.

A comparison between Unsafe Rust and Standard C is the most direct way to measure 
the performance of the underlying LLVM-generated machine code, as both environments 
make the same assumptions: the programmer knows what they are doing and will not 
violate memory safety

The main source of runtime performance difference between standard Rust and C
is array/slice bounds checking.

In safe Rust, an access like my_vec[i] always includes a check to ensure i is less 
than my_vec.len(), if this fails program panics (safely abort)
In `unsafe` Rust you can skip these checks:
```rust 
//Unsafe Rust: 
    let my_vec: Vec<u32> = vec![1, 2, 3];
    let index = 10; // Out of bounds, but compiler won't check
    let value = unsafe {
        // This is the C equivalent: trust the programmer
        *my_vec.get_unchecked(index)
    };
```
    
The Performance Effect
When the compiler knows the access is safe (either through compiler optimization or 
by using an unsafe operation), it removes the bounds check, often leading to 
performance that is identical to C for low-level arithmetic and data manipulation loops.

In fact, some benchmarks show that idiomatic safe Rust (using iterators or methods 
like get_mut() which the compiler is smart about) can already generate code that is 
just as fast as C or Unsafe Rust by optimizing away the bounds checks itself.

However, if you want to ensure the comparison is apples-to-apples—stripping
away all runtime safety overhead—using unsafe is the definitive method.

The Fair Comparision:
There are three ways to frame a C vs. Rust benchmark, and each tells a different story:

|Benchmark Type |Rust Implementation |C Implementation |Result Focus |
| :-- | :-- | :-- | :-- |
| Raw Performance (Your initial thought) |Idiomatic Safe Rust|Standard C (No safety checks)|C wins slightly due to unchecked array access.|
| Safety-Equivalency (Your previous proposal)|Idiomatic Safe Rust|"Hardened C (Manual checks, Sanitizers)"|"Rust wins or achieves near-parity, as its checks are often zero-cost or highly optimized."|
| Theoretical Limit (Your newest proposal)| Unsafe Rust (Manual unchecked access)| Standard C (Manual unchecked access)|"Performance is virtually identical, showing that the core performance of the two languages is equivalent."|


