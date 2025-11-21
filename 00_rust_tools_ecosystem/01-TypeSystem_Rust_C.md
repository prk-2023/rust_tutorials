# Type System 


## Type System ( Short Intro )

A **type system** is a set of rules that assigns a **type** (like integer, float, or string) to various 
constructs in a computer program, such as variables, expressions, functions, and modules.

Its primary goals are to:

1.  **Prevent errors** by ensuring operations are only performed on compatible data types (e.g., preventing 
    the addition of a string to an integer).
2.  **Organize code** and convey the programmer's intent.
3.  **Optimize performance** by giving the compiler information about the data being used.

Type systems can be broadly categorized as:

* **Static:** Type checking happens **at compile time** (e.g., C, Rust). Errors are caught early.
* **Dynamic:** Type checking happens **at run time** (e.g., Python, JavaScript). Errors may only appear when 
  the code is executed.

---

## Type Systems in C vs. Rust

While both C and Rust are **statically typed** systems programming languages, their type systems differ 
vastly in their **design philosophy** and **guarantees**, especially regarding **memory safety**. 


| Feature | C Type System | Rust Type System |
| :--- | :--- | :--- |
| **Philosophy** | Minimalist, trust the programmer, focuses on low-level representation. | Comprehensive, safety first, focuses on correctness and preventing undefined behavior. |
| **Safety** | **Unsafe by default.** Relies heavily on the programmer to ensure type and memory safety. | **Safe by default.** Guarantees memory safety and data-race freedom at compile time. |
| **Type Checking** | Relatively weak. Allows **implicit conversions** easily (e.g., `int` to `char`), which can lead to unexpected behavior and data loss. | **Strong and explicit.** Requires **explicit casting** for most conversions, minimizing unintended behavior. |
| **Pointers/References** | **Raw pointers** (`*`) are fundamental. They can be null or dangling, leading to **segfaults** and security vulnerabilities. | Uses **References** (`&`, `&mut`) governed by the **Ownership** and **Borrow Checker** rules, eliminating *null pointer dereferences* and *data races*. Raw pointers are restricted to `unsafe` blocks. |
| **Type Inference** | Limited. Types must be explicitly declared for almost all variables. | **Powerful.** Can often figure out the type of a variable without explicit annotation, improving readability without sacrificing safety. |
| **Algebraic Data Types (ADTs)** | Not natively supported. Programmers use `structs` and `unions` with manual tag fields to achieve similar, but less safe, functionality. | Core feature: **`enum`** (sum types) and **`struct`** (product types) combined with **Pattern Matching** provides powerful, exhaustive, and safe handling of complex data states (e.g., the built-in `Option<T>` for potentially missing values). |

### C's Type System: The Low-Level Model

C's type system is designed to be a thin layer over the underlying hardware.

* It's primarily about telling the compiler how many bytes to allocate for a piece of data and how to 
  interpret those bytes (e.g., $4$ bytes as a signed integer vs. $4$ bytes as a single-precision float).

* It lacks the sophistication to track complex invariants like **ownership** or **lifetime**, which is why 
  C code frequently suffers from issues like buffer overflows, use-after-free, and double-free errors. 
  The compiler mostly trusts the programmer and only enforces basic, local type compatibility.

### Rust's Type System: Safety as a Guarantee

Rust's type system is a modern, sophisticated design centered on the **Borrow Checker**.

* It uses its type system to enforce strict rules about how data can be accessed and mutated:

    * **Ownership:** Every value has a single owner.
    * **Borrowing:** You can have either one mutable reference (`&mut T`) OR any number of immutable 
      references (`&T`), but not both simultaneously.

* This system embeds **memory safety** and **concurrency safety** directly into the type-checking process, 
  moving almost all common systems programming errors from runtime to compile time. 
  If Rust code compiles without using an `unsafe` block, it is guaranteed to be free of 
  *null pointer dereferences, iterator invalidation, and data races*.

In essence: **C** uses its type system for **basic structure and memory layout**.
**Rust** uses its type system for **basic structure, memory layout, and guaranteed safety properties.**

## Ownership and Borrowing Rules:

**Ownership and Borrowing** system is the heart of what makes Rust's type system so powerful and safe.

The Ownership and Borrowing system is a set of rules checked by the **Borrow Checker** at **compile time**.
Its purpose is to manage memory and prevent common bugs like **data races** and **use-after-free** errors 
*without* needing a garbage collector or relying on manual deallocation (like in C).

---

### 1. Ownership (The Core Rule)

**Ownership** dictates when a variable is created and when its memory is cleaned up.

* **Rule:** Every value in Rust has a variable called its **owner**.
* **Rule:** There can only be **one owner** at a time.
* **Rule:** When the owner goes **out of scope**, the value is automatically dropped, and its memory is 
  reclaimed (this is often called "Resource Acquisition Is Initialization" or **RAII**).

=> Rust compiler checks the code to generates instructions to drop variables once their scope ends, the job
of manually deallocation like in "C" is now done by the compiler.

#### How Ownership Transfers (Move Semantics)

When a variable holding a complex value (like a `String` or a `Vec<T>`) is assigned to another variable, 
Rust considers this a **move**.

| Code | Action | Explanation |
| :--- | :--- | :--- |
| `let s1 = String::from("hello");` | **Ownership Established** | `s1` owns the memory holding "hello". |
| `let s2 = s1;` | **Move Occurs** | Ownership is **moved** from `s1` to `s2`. The memory is no longer valid for `s1`. |
| `// println!("{}", s1);` | **Compile Error** | Using `s1` after it moved would be a **use-after-move** error, which is caught by the compiler. |

---

### 2. Borrowing (Temporary Access)

To allow multiple parts of your code to interact with a value *without* giving up ownership, Rust uses 
**references**, a process called **Borrowing**. A reference is like a temporary lease on the data. 

* References do not take ownership.
* A variable that lends a reference is still the owner.
* Borrowing is governed by the **"One Writer or Many Readers"** rule.

#### The Borrow Checker Rules:

At any given time, you can have **one of two** things, but **not both**:

1.  **Multiple Immutable Borrows (Readers):**
    * You can have any number of **immutable references** (`&T`).
    * This allows multiple parts of the program to **read** the data simultaneously.
    * The data **cannot be changed** while these references exist.

2.  **One Mutable Borrow (Writer):**
    * You can have **only one mutable reference** (`&mut T`).
    * This allows one part of the program to **change** the data.
    * Having only one mutable reference **prevents data races**—a condition where two threads try to modify
      the same data at the time, leading to unpredictable results.

#### Example of a Mutable Borrow

| Code | Rule Check | Outcome |
| :--- | :--- | :--- |
| `let mut s = String::from("Hi");` | **Owner** established. | `s` owns the string data. |
| `let r1 = &mut s;` | **One mutable borrow** allowed. | `r1` can modify `s`. |
| `// let r2 = &mut s;` | **Violation!** | A second mutable borrow (`r2`) is not allowed while `r1` is active. **Compile Error!** |
| `r1.push_str(" there.");` | **Modification** allowed. | `s` is updated via `r1`. |

---

### 3. Lifetimes (The Duration of Borrows)

The Borrow Checker also enforces **Lifetimes**. 

A lifetime is a specific scope or duration for which a reference is valid.

* **Rule:** A borrowed value (a reference) **cannot outlive** the data it is referencing (the owner).

* This rule prevents **dangling references** (pointers that point to memory that has already been freed), 
  which is a major source of bugs in C and C++.

The compiler automatically infers most lifetimes, but when a function returns a reference, you sometimes 
have to explicitly tell the compiler the relationship between the input and output lifetimes using a syntax 
like `<'a>`.

**In short:** The **Ownership, Borrowing, and Lifetime** rules elevate Rust's type system from simply 
validating data formats (like C) to **validating memory and concurrency safety** at compile time.

