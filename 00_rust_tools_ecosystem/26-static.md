# `static` 

Keyword "static" can be a bit confusing because it serves two distinct purposes depending on how its used:
- as a declaration for a global variable or 
- as a lifetime for a reference.

The `static` keyword is used to denote a **static variable**, which has a few important characteristics that 
differentiate it from other types of variables.

Here’s a breakdown of its effects:

### 1. **Lifetime:**

* The most significant effect of `static` is that it gives the variable a **'static lifetime**. 
  This means the variable exists for the entire duration of the program, from start to finish.

* In other words, a `static` variable is **globally available** and **never deallocated** until the program 
  terminates.

Example:

```rust
static HELLO: &str = "Hello, world!";

fn main() {
    println!("{}", HELLO); // Accessing a static variable
}
```

### 2. **Mutability:**

* By default, static variables are **immutable**. If you want to make them mutable, you have to use the `mut` keyword, but there are restrictions on mutable static variables in Rust, especially around concurrency.
* To make a static variable mutable, you would need to use `unsafe` code.

Example:

```rust
static mut COUNTER: i32 = 0;

fn main() {
    unsafe {
        COUNTER += 1;
        println!("{}", COUNTER);
    }
}
```

### 3. **Global Access:**

* Static variables are globally accessible in the scope of the program. 
  You can access them from any function in the program, and they maintain their state throughout the entire 
  runtime of the program.

* They can be useful for things like global constants, or in situations where you need to maintain some 
  persistent state across different parts of your program.

When you put `static` before a variable name, you are creating a global variable. 
This means the value is allocated at compile time and exists for the **entire duration** of the program.

    * **Fixed Memory Location:** 
        Unlike `let` bindings, a `static` item has a fixed address in memory.

    * **Initialization:** 
        It must be initialized with a constant expression (something the compiler can calculate before the 
        program runs).

    * **Mutability:** 
        By default, they are immutable. While you can create a `static mut`, accessing or modifying it is
        **unsafe** because Rust cannot guarantee thread safety for a global variable that anyone can change.

```rust
static GREETING: &str = "Hello, world!";

fn main() {
    println!("{}", GREETING);
}

```

### 4. **Thread Safety:**

* Static variables are not inherently thread-safe. 
  If you need a thread-safe static variable, you would need to use synchronization primitives like `Mutex`,
  `RwLock`, or atomic types (`AtomicBool`, `AtomicI32`, etc.).

* For example, if you have a mutable static variable and want to access it from multiple threads, you should 
  protect it with a `Mutex` or similar type to avoid race conditions.

### 5. **Use in `static` References:**

* The `static` keyword is also used when defining **static references**. 
  These references are essentially **pointers** to data with a `'static` lifetime, which is the longest 
  possible lifetime in Rust.

Example:

```rust
static HELLO: &str = "Hello, world!";
```

When you see `&'static T`, you are dealing with a **lifetime**. 
This tells the compiler that the data being pointed to will live as long as the program runs.

    * **String Literals:** 
        All string literals have a `'static` lifetime because they are baked into the binary's data segment.

    * **Ownership:** 
        If you own a `String` and want to turn it into a `&'static str`, you usually have to "leak" the 
        memory (using `Box::leak`), which tells Rust, 
        "I am never going to free this memory, so it's safe to treat it as static."

### Key Takeaways:

* **`static`** gives variables a **static lifetime**, meaning the variable is available for the entire 
  duration of the program.
* **Immutable by default** but can be made **mutable with `unsafe`** code.
* Static variables can be accessed globally and persist across function calls.
* For thread safety in concurrent environments, use synchronization mechanisms.


## 6. The `T: 'static` Bound (Trait Bounds)

This is where most beginners get stuck. 
If a function requires `T: 'static`, it **does not** mean that `T` must live forever. 
It means that **`T` cannot contain any non-static references.**

* **Owned Data:** 
    Types like `String`, `i32`, or `Vec<u8>` satisfy the `'static` bound because they own their data 
    completely. They *could* live forever if you wanted them to.
* **Borrowed Data:** 
    A struct like `User<'a>` that contains a reference `&'a str` **does not** satisfy the `'static` bound
    (unless `'a` is specifically `'static`).

**Common Use Case:**
    When you spawn a new thread using `std::thread::spawn`, Rust requires the closure to be `'static`. 
    This is because the compiler has no way of knowing how long the new thread will run, so it forbids you
    from passing in any references that might expire while the thread is still working.

---

## Summary Comparison

| Feature | `static` Item | `'static` Lifetime |
| --- | --- | --- |
| **Purpose** | Defines a global variable. | Describes how long a reference is valid. |
| **Storage** | Stored in the binary's data segment. | Can be in the binary or "leaked" on the heap. |
| **Requirement** | Must be initialized with a `const`. | Must outlive the entire program execution. |

Would you like to see an example of how to fix a "borrowed value does not live long enough" error when dealing with `'static` bounds?
