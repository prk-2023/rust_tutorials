# Inline assembly in Rust.


Before we move to assembly in Rust we recap the assembly in C.

## Introduction of Assembly in C: ( Part #1 ):

Inline assembly is a feature provided by some compilers that allow programmers to embed low-level
assembly language instructions directly within a high-level source program ( such as C or Rust ).

Instead of writing a completely separate assembly file and linking it later, You can write machine-level
instructions exactly where you need them. 

Inline assembly is preferred in two major programming domains:
1. HW Interaction: Executing special CPU instructions that have no direct equivalent in high level
   programming languages ( eg: enable/disable Interrupt or reading specific CPU control registers. ).
2. Performance Optimization: Manual tweaking and vectorizing highly critical loops or mathematical
   functions to out-perform the computer's automatic code generation. 

As Inline assembly is an extension to the host language, its syntax varies wildly depending on the
compiler. 

- With GCC/Clang ( GNU ) Style: Uses the **"Extended Asm"** syntax. It requires explicit description of
  inputs, outputs and modified registers using colons:

```c 
    int foo = 10, bar = 5; 
    
    //Syntax: asm("Instruction": outputs, inputs: clobbers );
    asm("add %2, %1, %0",
        : "=r" (foo)    // Output: %0 represents "foo"
        : "r" (foo),    // Input: %1 represents "foo"
        : "r" (bar),    // Input: %2 represents "bar"
    )
```

While Inline assembly is powerful, it should be last resort of usage for the below reasons:

- Loss of Portability: Code written in X86_64 will not compile to run on a ARM processor. 

- Breaks Compiler Optimization: Compilers struggle to safely analyze what happens inside an assembly
  block, which can cause register allocation conflicts or unintended bugs. 

- Harder Maintenance: assembly lacks the strict type-safety and structural guardrails of modern
  programming language, making it harder to debug. 


### Forms of Assembly:
Modern C compilers ( GCC, Clang, and ICC ) use GNU Extended assembly syntax: they are Basic and Extended
Assembly:

1. Basic Assembly:
    Just a block of assembly code without any input/output or side-effect declerations;
    ```c 
    asm("nop"); // A simple no operation instruction 
    ```
Note: Basic assembly is highly discouraged for anything complex. The compiler does not know what
registers or memory are being modified, which can completely ruin compiler optimizations.

2. Extended Assembly: 
    Provides a structural way to pass C variables into assembly registers and get result back safely.
    It has the below syntax:
    ```c 
    asm [volatile] (
        "assembly code"
        : output operands   /* optional */
        : input operands    /* optional */
        : clobbered registers/memory  /* optional */
    );
    ```
    - `volatile`: Tells the compiler not to optimize away, move, or delete this assembly block.
    - `assembly code`: The string containing your instructions.
    - `output operands`: C variables where the assembly results will be stored.
    - `input operands`: C expressions or variables fed into the assembly.
    - `clobbered registers`: list of registers your assembly modifies that the compiler needs to know
      about so it doesn't try to use them for other variables.

#### Syntax ( with example )

An `x86-64` example: adding two C integers together using inline assembly.

```c 
    #include <stdio.h>

    int main() {
        int src1 = 10;
        int src2 = 20;
        int dst;

        asm volatile (
            "addl %2, %1;\n\t"
            "movl %1, %0;"
            : "=r" (dst)           // %0: Output
            : "r" (src1), "r" (src2) // %1: Input 1, %2: Input 2
            :                      // No clobbered registers explicitly altered
        );

        printf("Result: %d\n", dst); // Outputs 30
        return 0;
    }
```
- Inside the `asm` assembly string, tokens like %0, %1, and %2 act as placeholders. The compiler numbers
  them sequentially starting from 0, reading from left-to-right through the outputs first, and then the
  inputs.
  In our code:
        `%0` maps to `dst`, `%1` maps to `src1`, `%2` maps to `src2`

- `=r` and `r` strings: These are constraints that tell the compiler how to handle the variables.

#### The Clobber List: Preventing Disasters

When executing assembly instructions, you often modify registers behind the compiler's back.
If the compiler was keeping a critical C variable in `%rax`, and you suddenly run:
    `movq $0, %%rax`, your program will silently break.

```c 
    asm volatile (
        "movl $5, %%eax;"  // We are manually overwriting EAX
        "movl %%eax, %0;"
        : "=r" (dst)
        : // No inputs
        : "eax"            // Tell the compiler: "Hey, I hijacked EAX!"
    );
```
You prevent this via the Clobber List.

Note: 
    - In GCC inline assembly, when naming registers inside the assembly string explicitly (like `eax` you must prefix them with 2 "%%eax' so the compiler doesn't confuse them with operand placeholder like `%0`.

#### The "memory" Clobber
If your assembly instruction modifies an arbitrary location of systems memory (ex: if you are writing a
custom memcpy) you should add "memory" to you clobber list. This forces the compiler to flush asll cached
register values back to RAM before running your assembly and reload them afterward.


#### Named Operands:
Reading %0, %1, %2 can quickly become confusing in larger blocks. 
Fortunately, you can name your operands using a [name] syntax:

```c 
    int val1 = 40, val2 = 2, result;

    asm volatile (
        "imull %[v2], %[v1];"
        "movl %[v1], %[res];"
        : [res] "=r" (result)
        : [v1] "r" (val1), [v2] "r" (val2)
    );

```
This performs `result=val1×val2`, and is significantly easier to read and maintain.


### GNU C Inline Assembly Basics

A simple GNU C inline assembly statement looks like:

```c
__asm__("nop");
```

or

```c
asm("nop");
```

This tells the compiler:

> Insert the assembly instruction `nop` at this point.

---

### Why `volatile` Exists

Consider:

```c
void foo() {
    __asm__("nop");
}
```

The compiler sees:

* no inputs
* no outputs
* no visible effect

So it may decide:

> This instruction does nothing useful; I'll remove it.

To prevent that:

```c
void foo() {
    __asm__ __volatile__("nop");
}
```

Now the compiler must keep the instruction.

`volatile` means:

> This assembly has important side effects. Don't delete it or optimize it away.

---

### Inputs and Outputs

A more realistic example:

```c
int x;

__asm__(
    "mov %1, %0"
    : "=r"(x)
    : "r"(42)
);
```

Structure:

```c
asm(
    "assembly template"
    : outputs
    : inputs
    : clobbers
);
```

#### Outputs

```c
: "=r"(x)
```

means:

* allocate a register (`r`)
* assembly writes to it (`=`)
* store result into `x`

#### Inputs

```c
: "r"(42)
```

means:

* place value `42` into a register
* make it available as operand `%1`

Compiler might generate:

```asm
mov w0, #42
mov w1, w0
```

and then store the result into `x`.

---

### Clobbers

Suppose your assembly modifies a register:

```c
asm("mov x0, #0");
```

The compiler doesn't automatically know that.

You must tell it:

```c
asm(
    "mov x0, #0"
    :
    :
    : "x0"
);
```

The clobber list says:

> These registers get destroyed by this assembly.

Without this information the compiler may incorrectly assume the register still contains an old value.

---

### Memory Clobber

This becomes important for barriers.

Example:

```c
asm volatile("dsb sy");
```

The CPU executes the barrier.

But the **compiler** might still reorder memory accesses around it because it doesn't know the assembly affects memory ordering.

So kernel code often uses:

```c
asm volatile("dsb sy" ::: "memory");
```

Notice:

```c
::: "memory"
```

The `"memory"` clobber means:

> Assume any memory location may be affected.

This prevents the compiler from moving memory operations across the asm block.

Example:

```c
a = 1;

asm volatile("dsb sy" ::: "memory");

b = 2;
```

Compiler cannot reorder the stores around the barrier.

---

### Understanding `dsb sy`

On ARM:

```asm
dsb sy
```

means:

#### DSB

Data Synchronization Barrier

#### SY

Full system scope

It guarantees:

> Every explicit memory access before the barrier has completed before any later instruction can continue.

Example:

```c
device_reg = value;

asm volatile("dsb sy" ::: "memory");

control_reg = ENABLE;
```

Without the barrier, the CPU could potentially allow the enable write to become visible before the data write.

The barrier prevents that.

---

### Mapping This to Rust

Now consider:

```rust
unsafe {
    asm!("dsb sy", options(nostack));
}
```

This is roughly analogous to:

```c
asm volatile("dsb sy");
```

Differences:

#### GNU C

```c
asm volatile("dsb sy");
```

uses the keyword `volatile` to indicate side effects.

#### Rust

Rust's `asm!` assumes side effects by default.

So:

```rust
asm!("dsb sy");
```

already behaves closer to:

```c
asm volatile("dsb sy");
```

than to plain:

```c
asm("dsb sy");
```

---

#### GNU C Clobbers

```c
asm volatile("dsb sy" ::: "memory");
```

Rust expresses similar information differently:

```rust
asm!(
    "dsb sy",
    options(nostack)
);
```

Rust does not use the `: : :` syntax. Instead it has:

* `in(...)`
* `out(...)`
* `inout(...)`
* `lateout(...)`
* `options(...)`

which provide a more structured way to describe inputs, outputs, and behavior.

---

#### `options(nostack)`

GNU C has no direct equivalent keyword.

It tells Rust:

> This assembly does not push/pop or otherwise modify the stack pointer.

That gives LLVM more freedom to optimize.

---

So the mental mapping is roughly:

| GNU C                 | Rust                                                              |
| --------------------- | ----------------------------------------------------------------- |
| `asm("nop")`          | `asm!("nop")`                                                     |
| `asm volatile("nop")` | `asm!("nop")` (default side effects)                              |
| Output operands       | `out(...)`                                                        |
| Input operands        | `in(...)`                                                         |
| Input/output          | `inout(...)`                                                      |
| Clobbers              | explicit register operands/options                                |
| `"memory"` clobber    | memory effects are expressed through Rust asm constraints/options |
| N/A                   | `options(nostack)`                                                |

The biggest conceptual difference is that GNU inline assembly is based on the older 
`"template : outputs : inputs : clobbers"` model, while Rust's `asm!` uses named operands 
and explicit options to make the compiler's understanding of the assembly more precise.

--- 

## Inline Assembly in Rust:

Rust handles inline assembly in a much more modern, structured, and developer-friendly way. 

Rust uses `asm!` macro ( standardized in 2022), Instead of GCC cryptic, colon-separated syntax,
Rust uses clear, key-value style syntax that borrows heavily from `format!`. 

---

### The Core Differences at a Glance

| Feature | C (GCC Extended Inline Asm) | Rust (`asm!` macro) |
| --- | --- | --- |
| **Safety** | Assumed unsafe (silently compiles, crashes at runtime) | Explicitly wrapped in an `unsafe {}` block |
| **Syntax** | `asm("code" : outputs : inputs : clobbers);` | `asm!("code", inputs/outputs, options);` |
| **Register Clobbers** | Must be manually listed by the developer | **Automatically managed** by the compiler |
| **Register Flavor** | Typically AT&T syntax by default on GCC | **Intel syntax** by default on x86/x86-64 |

---

### Translating C to Rust: Step-by-Step

Let's take the basic addition example we did in C and translate it to Rust.

### The C Way:

```c
asm volatile (
    "addl %2, %1;\n\t"
    "movl %1, %0;"
    : "=r" (dst)
    : "r" (src1), "r" (src2)
);

```

### The Rust Way:

```rust
use std::arch::asm;

fn main() {
    let src1: i32 = 10;
    let mut src2: i32 = 20;
    let dst: i32;

    unsafe {
        asm!(
            "add {src2_reg}, {src1_reg}",
            "mov {dst_reg}, {src2_reg}",
            src1_reg = in(reg) src1,
            src2_reg = inout(reg) src2,
            dst_reg = out(reg) dst,
        );
    }

    println!("Result: {}", dst); // Outputs 30
}

```

---

### Key Concepts in Rust Inline Assembly

#### 1. Intel Syntax by Default

Notice that the instruction order is reversed from standard GCC. In Rust x86 assembly, it is 
`instruction destination, source`.

* C (AT&T): `addl %2, %1` (Adds %2 to %1)
* Rust (Intel): `add {src2_reg}, {src1_reg}` (Adds `src1_reg` to `src2_reg`)

### 2. Explicit Operand Classes

Instead of C's cryptic `"r"`, `"=r"`, and `"+r"` constraints, Rust uses explicit directional keywords:

* **`in(<reg_class>) <expr>`**: Input operand. The compiler loads the value into a register before the 
  assembly runs.

* **`out(<reg_class>) <expr>`**: Output operand. The compiler writes the register contents to this variable
  after the assembly finishes.

* **`inout(<reg_class>) <expr>`**: Combined read-write operand (replaces C's `+` constraint).

* **`inlateout` / `outlateout**`: Advanced constraints that tell the compiler it can reuse an input register
  for an output register if the input is read before the output is written.

#### 3. Automatic Clobbering

In C, if you use a register, you have to remember to list it in the clobber section. 

In Rust, **you don't write clobber lists.** If you use an `out` or `inout` constraint, Rust automatically 
knows that register is being modified. 

If you need to scratch a specific register without tying it to a variable, you can just use an underscore:

```rust
unsafe {
    asm!(
        "xor eax, eax",
        out("eax") _, // Tells Rust we overwrote EAX, but we don't care about the result
    );
}

```

---

### Rust Options (The Flags)

Rust replaces C's `volatile` keyword with an explicit `options(...)` argument at the end of the macro.

```rust
unsafe {
    asm!(
        "nop",
        options(nostack, nomem)
    );
}

```

Some common options include:

* **`nomem`**: Tells the compiler the assembly doesn't read or write system memory (allowing the compiler to keep variables cached in registers across the assembly block).
* **`nostack`**: Tells the compiler the assembly doesn't push anything to the stack (allowing Rust to optimize stack alignments).
* **`pure`**: Tells the compiler the assembly has no side effects and its output depends purely on its inputs. This allows the compiler to optimize the entire block away if the output variable isn't used!
* **`readonly`**: Tells the compiler the assembly reads from memory but doesn't write to it.

By default, `asm!` is considered volatile (it won't be optimized away unless you pass `options(pure, nomem)`).

---

### Quick Summary Checklist for Moving to Rust

1. **Wrap it:** Your assembly must always live inside an `unsafe {}` block.
2. **Flip your thinking:** Switch from AT&T syntax (C default) to Intel syntax (Rust default) if you are working on x86/x86-64.
3. **Drop the colons:** Replace the triple-colon format (`:::`) with trailing comma-separated arguments.
4. **Forget Clobbers:** Let Rust manage your clobbers automatically using `out("reg") _` or your variable definitions.

Would you like to see how to implement a specific low-level C assembly trick (like atomic operations or CPUID fetching) over in Rust?
