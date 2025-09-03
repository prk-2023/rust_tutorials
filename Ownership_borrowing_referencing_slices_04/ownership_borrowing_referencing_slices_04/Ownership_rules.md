# Ownership Rules for Stack and Heap variables:

**Rust's ownership rules** and how they apply to **stack** and **heap** variables.

---

##  RUST OWNERSHIP RULES — THE CORE CONCEPTS

Rust’s ownership system is designed to ensure **memory safety** without a garbage collector. 

It's centered around these **3 core rules**:

### 1. **Each value has a single owner**

* Every piece of data has one variable that's considered its "owner."
* Once the owner goes out of scope, the value is dropped (freed).

### 2. **Ownership can be moved**

* Assigning a variable to another moves ownership (for non-`Copy` types).
* After a move, the original variable is invalid and cannot be used.

```rust
let s1 = String::from("hello");
let s2 = s1; // s1 is moved
// println!("{}", s1); // ❌ Error: s1 no longer owns the value
```

### 3. **Ownership can be borrowed**

* You can **borrow** data via references:

  * `&T` for immutable borrow (read-only)
  * `&mut T` for mutable borrow (read/write)
* At any given time:

  * You can have **either**:

    * One mutable reference
    * Or any number of immutable references
  * But **not both at the same time**

---

## COPY vs MOVE (applies to ownership)

###  Types that implement `Copy` (e.g., integers, floats, bools, chars):

```rust
let a = 5;
let b = a; // value is copied
println!("{}", a); // ✅ OK
```

* Ownership is **not** moved
* Both variables are valid
* These are typically **stack-only values**

###  Types that do **not** implement `Copy` (e.g., `String`, `Vec<T>`, custom types):

```rust
let s1 = String::from("hi");
let s2 = s1; // value is moved
// println!("{}", s1); // ❌ Error
```

* Ownership is **moved**
* Original variable becomes invalid unless you `clone()` it

---

## STACK vs HEAP — MEMORY MODEL

| Feature         | Stack                                              | Heap                                        |
| --------------- | -------------------------------------------------- | ------------------------------------------- |
| Speed           | Fast                                               | Slower                                      |
| Allocation      | Compile-time                                       | Runtime (via `Box`, `Vec`, `String`)        |
| Size            | Fixed                                              | Dynamic                                     |
| Examples        | `i32`, `f64`, `bool`, `char`, tuples of Copy types | `String`, `Vec<T>`, `Box<T>`, large structs |
| Ownership Rules | ✅ Yes                                             | ✅ Yes                                      |

### Stack Variables

* Stored directly on the stack
* Usually implement `Copy`
* Small, fixed-size data
* Freed when they go out of scope

### Heap Variables

* The actual data lives on the heap; the variable holds a pointer on the stack
* Complex or dynamically-sized data
* Freed when their owner goes out of scope

```rust
let x = 5; // stack
let s = String::from("hello"); // s (pointer + metadata) on stack, data on heap
```

---

## Drop and Cleanup

* When a variable goes out of scope, Rust automatically calls `drop()` for cleanup.
* Happens for both heap and stack values, but more visible for heap-allocated data.

```rust
{
    let s = String::from("bye"); // allocated on heap
} // `s` goes out of scope, memory is freed
```

---

## FINAL SUMMARY: KEY POINTS TO REMEMBER

| Concept        | Summary                                                            |
| -------------- | ------------------------------------------------------------------ |
| Ownership      | Each value has one owner; dropped when owner goes out of scope     |
| Move semantics | Assigning a non-`Copy` value moves ownership                       |
| Borrowing      | Use `&T` or `&mut T` for references instead of moving ownership    |
| Stack          | Fast, fixed-size, often `Copy`, data lives and dies with the scope |
| Heap           | Dynamic size, needs allocation/deallocation, ownership critical    |
| Copy types     | Simple types like `i32`, `bool`, `char`, etc.                      |
| Non-Copy types | `String`, `Vec<T>`, and most user-defined types                    |

---

## programming usage:

>  **You don’t need to *manually* track whether data is on the stack or heap.**

Rust handles that under the hood. **What you *do* need to keep track of is whether a 
type implements the `Copy` trait.**

---

##  Guiding Principle:

> **In Rust, ownership, moving, and copying behavior depends on the type, not where the data lives 
  (stack or heap).**

### So don’t ask:

> "Is this variable on the stack or heap?"

### Instead, ask:

> "Does this type implement `Copy`, or does it move by default?"

---

## Here's how to think about it:

| Question to Ask                                      | Why It Matters                                           | Example                      |
| ---------------------------------------------------- | -------------------------------------------------------- | ---------------------------- |
| ❓ Does this type implement `Copy`?                   | If yes, assignment **copies** the value.                 | `i32`, `bool`, `char` — copy |
| ❓ Does this type own heap data or have dynamic size? | If yes, assignment **moves** the value (unless cloned).  | `String`, `Vec<T>` — move    |
| ❓ Do I need to use a reference (`&`) instead?        | If I don't want to move ownership or copy a large struct | Borrow instead of move       |

---

## Examples — How to Think Through Assignments

### Stack, `Copy` Type:

```rust
let x = 42;
let y = x; // Copy
println!("{}", x); // ✅ OK
```

* You don't care that it’s on the stack — you care that `i32` implements `Copy`.

---

### Heap-backed, Move Type:

```rust
let s1 = String::from("hello");
let s2 = s1; // Move
// println!("{}", s1); // ❌ Error: s1 was moved
```

* `String` doesn’t implement `Copy` → it moves.
* Even though the pointer is on the stack, the ownership rule applies to the **whole value**.

---

### Clone to Explicitly Copy Heap Data:

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // Explicit deep copy
println!("{}", s1); // ✅ OK
```

---

## How to Keep Track Efficiently

###  Memorize or Google Common Traits:

* `Copy` trait = Copy on assignment
* Not `Copy` = Move on assignment (unless cloned)
* Most primitive types are `Copy`
* Most heap-backed types (like `String`, `Vec`, `Box`, custom structs) are not

### Use the Compiler

* Rust’s compiler **tells you** when you’re using a moved value. Just follow the message and adjust.

### IDE Tools Help

* Tools like **rust-analyzer** in VSCode show whether a type implements `Copy`, and where moves happen.

---

## Practical Mindset When Writing Code

Ask yourself:

1. Do I want to **transfer ownership**?
   → Assign directly (`let y = x;`)

2. Do I want to **borrow** temporarily?
   → Use reference (`let y = &x;` or `&mut x`)

3. Do I want to **copy the data**?
   → Use `.clone()` for heap types, or nothing for `Copy` types

---

## TL;DR

| Focus On                                       | Not This                                   |
| ---------------------------------------------- | ------------------------------------------ |
| ❗ Whether a type is `Copy` or not              | ✖ Whether the variable is on stack or heap |
| ❗ Whether you're moving, borrowing, or cloning | ✖ Manual memory layout                     |
| ❗ What the compiler tells you                  | ✖ Trying to guess memory behavior          |

---

## **quick reference table** 

Commonly used types in Rust and how they behave in terms of **ownership**, **copying**, and **moving**.

---

## Common Rust Types: Copy vs Clone vs Move

| Type                            | `Copy`?                              | `Clone`?                   | Default Assignment Behavior          |
| ------------------------------- | ------------------------------------ | -------------------------- | ------------------------------------ |
| `i32`, `u32`, `f64`, etc.       | ✅ Yes                                | ✅ Yes                      | Copy                                 |
| `bool`, `char`                  | ✅ Yes                                | ✅ Yes                      | Copy                                 |
| `&T` (reference)                | ✅ Yes                                | ✅ Yes                      | Copy (just copies the pointer)       |
| `&mut T` (mutable ref)          | ✅ Yes                                | ✅ Yes                      | Copy                                 |
| `[T; N]` (array of Copy)        | ✅ Yes                                | ✅ Yes                      | Copy                                 |
| `String`                        | ❌ No                                 | ✅ Yes                      | Move                                 |
| `Vec<T>`                        | ❌ No                                 | ✅ Yes                      | Move                                 |
| `Box<T>`                        | ❌ No                                 | ✅ Yes                      | Move                                 |
| `Option<T>`                     | ✅ if `T` is Copy                     | ✅ if `T` is Clone          | Inherits behavior from `T`           |
| `Result<T, E>`                  | ✅/❌                                  | ✅ if `T` and `E` are Clone | Same as above                        |
| Custom Struct (all Copy fields) | ✅ if marked `#[derive(Copy, Clone)]` | ✅ if derived               | Depends on whether you derive traits |
| `&str` (string slice)           | ✅ Yes                                | ✅ Yes                      | Copy                                 |
| `str` (unsized)                 | ❌ No                                 | ✅ Yes                      | Usually behind reference (`&str`)    |

---

## How to Know if a Type is `Copy` or `Clone`

* ✅ **If a type is `Copy`**, then assignment like `let b = a;` **copies the value**, and both are valid.
* ❌ **If not**, assignment **moves** the value, and the original is invalid (unless you `.clone()` it).

---

### Example: Comparing Copy vs Move

```rust
// Copy example (i32)
let x = 10;
let y = x;
println!("{}", x); // OK — Copy

// Move example (String)
let s1 = String::from("hello");
let s2 = s1;
// println!("{}", s1); // ❌ Error — moved
```

To fix the move:

```rust
let s1 = String::from("hello");
let s2 = s1.clone(); // Explicit deep copy
println!("{}", s1); // ✅ OK
```

---

## Rule of Thumb

| If you want to...                 | Do this                                           |
| --------------------------------- | ------------------------------------------------- |
| Avoid ownership transfer          | Use references (`&T` or `&mut T`)                 |
| Explicitly copy heap data         | Use `.clone()`                                    |
| Work with a simple copyable value | Just assign it (`let b = a`)                      |
| Keep ownership but pass around    | Use `&T` or `Rc<T>` / `Arc<T>` (shared ownership) |

---

## How to Check if a Type is `Copy` or `Clone` in Code

Use this trick:

```rust
fn needs_copy<T: Copy>(_val: T) {}
fn needs_clone<T: Clone>(_val: T) {}

let x = 5;
needs_copy(x); // ✅ Compiles

let s = String::from("hi");
// needs_copy(s); // ❌ Won’t compile
needs_clone(s); // ✅ Compiles
```

---

## Summary

* **Don't track memory layout manually (stack/heap)** — Rust handles that.
* **Track type traits** like `Copy`, `Clone`, or whether it’s `Move`.
* **Use the compiler** — it’s your ally and will guide you on ownership issues.

---

