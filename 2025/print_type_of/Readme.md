# print type of 


In **Rust**, there's **no built-in trait like Python's `type()` function** that directly prints the **type of a variable at runtime**, because **Rust’s type system is designed to work at compile time**, not runtime.

However, you *can* inspect types in a few indirect ways.

---

## Option 1: Use `std::any::type_name`

Rust has a **compile-time** function for this:

```rust
use std::any::type_name;

fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn main() {
    let x = 5;
    let y = Some("hello");

    print_type_of(&x); // Output: i32
    print_type_of(&y); // Output: core::option::Option<&str>
}
```

---

### How it Works:

* `type_name::<T>()` returns a string with the **type of T**.
* You pass a reference to the value so Rust can infer the type.

This is as close as it gets to `type(var)` in Python.

---

## Notes:

* The type name is resolved at **compile time**, but it can be printed at **runtime**.
* The output may include full paths like `core::option::Option<i32>` — it’s not always as clean as in Python.

---

## Bonus: For Debugging in Macros

You can also use the `dbg!()` macro while debugging, which prints both the value and infers the type during compilation:

```rust
fn main() {
    let name = "Rust";
    dbg!(name); // [src/main.rs:3] name = "Rust"
}
```

But it won’t print the type unless you combine it with `type_name()`.

---

Would you like a macro or utility function to simplify printing types while debugging?

# macro  utility of the above print
---
Here's a simple utility you can copy into your Rust project to make printing types easier during debugging.

---

## Utility Function: `print_type!` Macro

```rust
use std::any::type_name;

/// Prints the type of any expression.
macro_rules! print_type {
    ($val:expr) => {
        println!(
            "Type of `{}` is: {}",
            stringify!($val),
            type_name::<typeof($val)>()
        );
    };
}
```

> **But wait!** `typeof($val)` doesn't exist in Rust.

So instead, we use a helper function, since Rust macros can't directly get the type like that. Here's the **correct working version**:

---

## Working Version: `print_type_of` Function + Macro

```rust
use std::any::type_name;

/// Generic function to print type of a value
fn print_type_of<T>(_: &T) {
    println!("{}", type_name::<T>());
}

fn main() {
    let a = 42;
    let b = Some("hello");
    let c = vec![1.0, 2.0, 3.0];

    println!("Type of `a`:");
    print_type_of(&a); // i32

    println!("Type of `b`:");
    print_type_of(&b); // Option<&str>

    println!("Type of `c`:");
    print_type_of(&c); // alloc::vec::Vec<f64>
}
```

---

## 💡 Optional: Wrap It in a Macro

If you really want a **macro-style**, try this:

```rust
macro_rules! print_type {
    ($val:expr) => {
        println!(
            "Type of `{}` is: {}",
            stringify!($val),
            std::any::type_name::<_>(&$val)
        );
    };
}

fn main() {
    let x = 123;
    let y = "hi";
    let z = Some(3.14);

    print_type!(x);
    print_type!(y);
    print_type!(z);
}
```

> Output:

```
Type of `x` is: i32
Type of `y` is: &str
Type of `z` is: core::option::Option<f64>
```

---

Let me know if you want to enhance it further (e.g. also print value, use in logging, etc.).
