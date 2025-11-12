# Implicit Borrowing in Rust macros:

When using macros like "println!()" you will notice something subtle but very important about **Rust’s 
ownership and borrowing model**, and how the **`println!()` macro** works under the hood.

## 🦀 The Short Answer

Yes : sometimes you pass a reference (`&x`), sometimes the value (`x`), and yet you can **still use `x` afterward**.

That’s because:

> `println!()` almost always takes **a reference to your value**, even if you don’t explicitly write `&`.

So even if you write:

```rust
let x = String::from("hello");
println!("{}", x); // looks like you moved x
println!("{}", x); // ✅ still works
```

you are **not** moving ownership of `x`.
Instead, `println!()` automatically **borrows** `x` as `&x` under the hood.

## Why It Works — The Trait System

The reason this works lies in **Rust’s formatting traits** — specifically the `std::fmt` traits that power 
macros like `println!`.

Let’s look at what happens with:

```rust
println!("{}", x);
```

When you use `{}`, the macro expands to something roughly like:

```rust
std::fmt::Display::fmt(&x, &mut formatter);
```

Notice that `&x` is automatically taken!
That means `println!()` **borrows** your variable, not moves it.

So the compiler implicitly inserts a reference — it’s as if you wrote:

```rust
println!("{}", &x);
```

That’s why you can still use `x` afterward — you never gave up ownership.


## Important Exception — When Ownership *Is* Moved

There *are* situations where ownership is transferred — but not because of `println!()`.
It happens when you use the **`{:?}` or `{:#?}`** format specifiers on a type that does **not implement 
`Copy`** and is **moved into a format string manually**.

Example:

```rust
#[derive(Debug)]
struct Dog(String);

fn main() {
    let dog = Dog(String::from("Lica"));
    println!("{:?}", dog);  // ✅ fine — println! borrows dog
    println!("{:?}", dog);  // ✅ fine again
}
```

Now contrast with:

```rust
#[derive(Debug)]
struct Dog(String);

fn main() {
    let dog = Dog(String::from("Lica"));
    let s = format!("{:?}", dog);  // ✅ borrows, no move
    println!("{}", s);
    // dog is still usable
}
```

In short: all the format macros (`println!`, `format!`, `write!`, etc.) borrow their arguments.


## What About Primitive Types?

For primitives like integers or booleans, it *looks* like you’re passing by value:

```rust
let n = 42;
println!("{}", n);
println!("{}", n);
```

But that’s fine too because integers implement the **`Copy` trait**, meaning the value is *copied*, not moved.
So either way — borrow or copy — `n` stays usable.

## 🧾 Summary Table

| Expression                           | What Actually Happens | Is Ownership Moved? | Variable Usable After? |
| ------------------------------------ | --------------------- | ------------------- | ---------------------- |
| `println!("{}", x);`                 | `Display::fmt(&x, …)` | ❌ No                | ✅ Yes                  |
| `println!("{:?}", x);`               | `Debug::fmt(&x, …)`   | ❌ No                | ✅ Yes                  |
| `println!("{}", &x);`                | Explicit borrow       | ❌ No                | ✅ Yes                  |
| `println!("{}", n);` (`n` = integer) | Copy semantics        | ✅ Moved (by copy)   | ✅ Yes                  |
| `drop(x);`                           | Ownership moved       | ✅ Yes               | ❌ No                   |


## TL;DR

* ✅ `println!()` automatically **borrows** your arguments.
* ✅ You can reuse variables after printing because ownership was not moved.
* ⚙️ The macro expands to code that calls `Display::fmt(&arg, …)` or `Debug::fmt(&arg, …)`.
* 🧠 Primitive types are `Copy`, so even passing them by value doesn’t affect ownership.


**In plain terms:**

> Rust makes printing ergonomic — you don’t have to manually write `&` because the formatting system
> automatically borrows values.

# Implicit Borrowing with other macros:

Rust’s macros often look like they “do magic,” but the truth is:

> Most of the time, they **borrow**, **copy**, or **expand** your arguments in predictable, safe ways.

Below some **major macros** that have similar behavior to `println!()`, how they treat ownership, and what 
to keep in mind when writing or using them.

## 1. The Formatting Family (`println!`, `print!`, `eprintln!`, `format!`, `write!`, `writeln!`)

### Common behavior

All of these macros expand to calls to the `std::fmt` traits (`Display`, `Debug`, etc.), just like `println!()`.

They **borrow their arguments**, even if you don’t explicitly write `&`.

Example:

```rust
let name = String::from("Lica");

println!("{}", name);     // borrows name
print!("Hi {}!", name);   // borrows name
eprintln!("{:?}", name);  // borrows name
let s = format!("{}", name); // borrows name
```

✅ Ownership **not moved**
✅ Safe to reuse `name`
🧠 Works because `Display::fmt` and `Debug::fmt` take `&self`

## 2. `vec![]`, `format!()`, and `vec_of!()` like Macros

These *construct* new data, and thus **consume** their arguments by value.

Example:

```rust
let a = 5;
let v = vec![a];  // a is Copy, so fine
let s = String::from("dog");
let v2 = vec![s]; // moves s
// println!("{}", s); // ❌ s moved into vec!
```

### Rule:

* If a type is `Copy` → value is duplicated.
* If not `Copy` → value is moved into the new container.

## 3. `dbg!()` — Debug Helper

The `dbg!()` macro prints a debug representation **and returns** the value.

```rust
let name = String::from("Lica");
let result = dbg!(name);
println!("{}", result);
```

✅ You can still use the value afterward because `dbg!()` **returns** it (after borrowing it for printing).

### Internally:

`dbg!()` expands roughly to:

```rust
{
    let tmp = &name;
    eprintln!("[file:line] {} = {:?}", stringify!(name), tmp);
    name
}
```

So the ownership of `name` is returned back to you.

🧠 **Tip:**
Use `dbg!()` liberally while debugging — it’s zero-overhead and never affects ownership.

## 4. `assert!()`, `assert_eq!()`, `assert_ne!()`

These macros check conditions and **borrow** their arguments.

```rust
let a = String::from("abc");
let b = String::from("abc");
assert_eq!(a, b); // borrows for comparison (but may move if `PartialEq` takes ownership)
```

**Important detail:**
If your type’s `PartialEq` implementation **takes ownership** (rare), it can move the values.
For built-in and most standard types, they only borrow — so no move occurs.

✅ Generally safe to use after `assert!`
⚠️ But if you define custom types with custom traits, check the signature.

## 5. `matches!()`

Pattern matching macro that **borrows** its input:

```rust
enum Dog { Happy, Sad }
let d = Dog::Happy;
assert!(matches!(d, Dog::Happy)); // borrows d
```

✅ `d` still usable after
🧠 Pattern matching works by reference unless you explicitly `match *d`

## 6. `todo!()`, `unimplemented!()`, `panic!()`

These macros **abort execution immediately** (panic), but they don’t return or move data.

```rust
fn f() {
    panic!("Something went wrong");
}
```

No ownership issues — they never reach the next line.

🧠 **Tip:**
Use `todo!()` or `unimplemented!()` as placeholders, not for real errors.

## 7. `thread::spawn()` and `move` Closures

Technically not a macro issue, but similar ownership behavior applies.

```rust
let s = String::from("dog");
let handle = std::thread::spawn(move || {
    println!("{}", s); // moved into closure
});
```

Here the `move` keyword **transfers ownership** into the closure.

✅ Needed because closures may outlive the scope they were created in
⚠️ You can’t use `s` after spawning the thread.

## 8. Custom Macros

If you write your own macros (`macro_rules!` or `macro`), *you* control whether arguments are moved or borrowed.

For example:

```rust
macro_rules! my_print {
    ($x:expr) => {
        println!("{}", $x);
    };
}
```

This macro borrows because `println!` borrows.
But if you instead call `drop($x)`, it will **move** ownership into `drop`.

🧠 **Rule of thumb:**

> Think about how the inner expressions are used — macros don’t have hidden semantics; they just expand code.

## 🧾 Summary Table

| Macro                                                  | Ownership Behavior      | Notes                 |                    |                          |
| ------------------------------------------------------ | ----------------------- | --------------------- | ------------------ | ------------------------ |
| `println!`, `print!`, `eprintln!`, `format!`, `write!` | Borrows (`&T`)          | Safe to reuse         |                    |                          |
| `vec![]`, `array![]`, etc.                             | Moves if not `Copy`     | Values consumed       |                    |                          |
| `dbg!()`                                               | Borrows + returns value | Great for debugging   |                    |                          |
| `assert!`, `assert_eq!`, `matches!`                    | Borrows                 | Safe for most types   |                    |                          |
| `panic!`, `todo!`, `unimplemented!`                    | N/A (no return)         | Execution stops       |                    |                          |
| `thread::spawn(move                                    |                         | ...)`                 | Moves into closure | Needed for thread safety |
| Custom macros                                          | Depends on expansion    | You control ownership |                    |                          |

## ⚙️ Key Things to Keep in Mind

1. **Rust macros are hygienic** — they don’t silently capture or mutate your variables.
2. **Formatting macros borrow automatically** — so don’t overuse `&`.
3. **Constructing macros like `vec!` or `collect()` consume values** — treat them as ownership transfers.
4. **Check trait signatures** (`Copy`, `Clone`, `Display`, `PartialEq`) to predict ownership behavior.
5. **Macros don’t have “special” powers** — they expand into regular Rust code. Ownership rules still apply.

### In summary

> Most macros like `println!` and `dbg!` are **borrowing macros** — they make your life easier by automatically referencing values when formatting or debugging.
>
> Construction macros like `vec!` or `format!` are **moving macros** — they consume data to build something new.
>
> If you ever wonder whether a macro moves or borrows, you can **expand it with**:
>
> ```bash
> cargo expand
> ```
>
> to see exactly what Rust code it generates.

---


# Assesment:

Using `cargo expand` to see what macros expand into is an excellent deep-dive exercise.
You’ll get to **see the actual code Rust generates**, which makes the borrowing and ownership behavior 
completely transparent.

A quick guide for assignment:

### Setup for `cargo expand`

If you don’t already have it:

```bash
cargo install cargo-expand
```

Then, in your project directory:

```bash
cargo expand
```

This shows the **fully expanded Rust code** after all macros (like `println!`, `dbg!`, etc.) are processed.

---

### Suggested Macros to Investigate

You can explore these and observe ownership vs borrowing behavior:

| Macro                   | What to Look For                                        |       |                                                   |
| ----------------------- | ------------------------------------------------------- | ----- | ------------------------------------------------- |
| `println!()`            | Check if it automatically inserts `&` when formatting   |       |                                                   |
| `dbg!()`                | Notice how it returns the original expression           |       |                                                   |
| `vec![]`                | See how it constructs a `Vec` by consuming its elements |       |                                                   |
| `assert_eq!()`          | Look at how it uses `==` and `panic!` internally        |       |                                                   |
| `thread::spawn(move     |                                                         | ...)` | Observe how the `move` closure captures ownership |
| `format!()`             | Check how it uses the `fmt` machinery without printing  |       |                                                   |
| A custom `macro_rules!` | Try writing your own and see how the expansion looks    |       |                                                   |


### What to Pay Attention To

While inspecting, make notes on:

* Whether the macro **takes a reference (`&x`)** or **moves (`x`)**.
* If the expanded code **creates temporary variables**.
* Whether it **returns a value** or just executes code.
* If the compiler **inserts type inference hints** (sometimes you’ll see `_` types in expansions).


### Example Mini Task

Try this:

```rust
fn main() {
    let s = String::from("Rust");
    println!("{}", s);
    println!("{}", s);
}
```

Then run:

```bash
cargo expand
```

See how the first `println!` expands — you’ll find something like:

```rust
::std::io::_print(::core::fmt::Arguments::new_v1(&[""], &[::core::fmt::ArgumentV1::new_display(&s)]));
```

Notice the `&s` — that’s the compiler automatically borrowing your value!

### Optional Write-Up Tips

If this is for a graded assignment, you can include:

1. A brief **macro behavior summary** (what it expands to).
2. Screenshots or snippets of the expanded code.
3. A short explanation of **why ownership wasn’t moved** or **why it was**.
4. A comparison table (borrowed vs moved macros).

