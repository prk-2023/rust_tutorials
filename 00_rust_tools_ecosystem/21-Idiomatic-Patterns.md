# Rust Idiomatic Patterns: 

## 1. Ownership and Borrowing Patterns

Rust's ownership system is central to its safety guarantees. 

Idiomatic Rust embraces ownership, borrowing, and lifetimes rather than trying to avoid them.

### Key idioms:

* **Prefer borrowing (`&T` or `&mut T`) over cloning or copying** to avoid unnecessary allocations or copies.

  ```rust
  // Non-idiomatic: cloning unnecessarily
  fn print_name(name: String) {
      println!("{}", name);
  }

  let name = String::from("Alice");
  print_name(name.clone());  // cloning not needed here
  ```

  Instead, idiomatic code borrows:

  ```rust
  fn print_name(name: &str) {
      println!("{}", name);
  }

  let name = String::from("Alice");
  print_name(&name);
  ```

* **Use `Cow` (Clone on Write) when you want to accept either borrowed or owned data efficiently.**

  ```rust
  use std::borrow::Cow;

  fn process_name(name: Cow<str>) {
      println!("{}", name);
  }

  let borrowed = "Alice";
  process_name(Cow::from(borrowed)); // borrowed
  let owned = String::from("Bob");
  process_name(Cow::from(owned));    // owned
  ```

* **Use references in function arguments unless ownership is specifically needed**.

---

## 2. Pattern Matching

Rust's powerful `match` and pattern matching is an idiomatic way to handle enums, options, and results.

* Use `match` to exhaustively handle all possible cases.

  ```rust
  enum Direction { North, South, East, West }

  fn print_dir(dir: Direction) {
      match dir {
          Direction::North => println!("Going north!"),
          Direction::South => println!("Going south!"),
          Direction::East  => println!("Going east!"),
          Direction::West  => println!("Going west!"),
      }
  }
  ```

* Prefer `if let` or `while let` for simpler matches where you only care about one variant:

  ```rust
  if let Some(x) = option {
      println!("Found {}", x);
  }
  ```

* Use pattern matching in `for` loops for destructuring tuples or structs:

  ```rust
  let points = vec![(1, 2), (3, 4)];
  for (x, y) in points {
      println!("x = {}, y = {}", x, y);
  }
  ```

---

## 3. Error Handling: `Result` and `Option`

Rust’s error handling is explicit and type-safe.

* **Idiomatic use of `Result`**: return a `Result<T, E>` from functions that can fail.

* **Use the `?` operator** to propagate errors cleanly.

  ```rust
  use std::fs::File;
  use std::io::{self, Read};

  fn read_file(path: &str) -> io::Result<String> {
      let mut file = File::open(path)?;  // Propagate error with ?
      let mut contents = String::new();
      file.read_to_string(&mut contents)?;
      Ok(contents)
  }
  ```

* **Handle `Option` idiomatically**, e.g., with `map`, `and_then`, or `unwrap_or` instead of manual `match` where appropriate:

  ```rust
  let name: Option<String> = Some("Alice".to_string());
  let len = name.as_ref().map(|s| s.len()).unwrap_or(0);
  ```

* Prefer `expect` with clear error messages only in situations where failure is truly unrecoverable or indicates a bug.

---

## 4. Iterator Patterns

Rust’s iterator trait and combinators provide a powerful, idiomatic way to handle sequences of data.

* Prefer using iterator methods (`map`, `filter`, `fold`, `collect`, etc.) over loops for transformations.

  ```rust
  let nums = vec![1, 2, 3, 4, 5];
  let evens: Vec<_> = nums.iter()
                          .filter(|&&x| x % 2 == 0)
                          .map(|&x| x * 2)
                          .collect();
  ```

* Use iterator adapters to chain operations, keeping code concise and lazy-evaluated.

* Use `for` loops for side-effectful iteration when you don’t need the collected result.

---

## 5. Struct and Enum Idioms

* **Use tuple structs** for simple wrappers without named fields.

  ```rust
  struct Inches(i32);
  ```

* **Use `new` constructor idioms** for initializing structs:

  ```rust
  struct User {
      name: String,
      age: u32,
  }

  impl User {
      fn new(name: String, age: u32) -> Self {
          Self { name, age }
      }
  }
  ```

* **Use `derive` macros** for common traits (`Debug`, `Clone`, `PartialEq`, `Eq`, `Hash`) instead of implementing manually.

---

## 6. Trait and Generics Idioms

* Prefer using **traits for abstraction** instead of `enum` or `void*`-style tricks.

* Use **trait bounds** on generic functions to specify required behavior.

  ```rust
  fn print_debug<T: std::fmt::Debug>(item: T) {
      println!("{:?}", item);
  }
  ```

* Use **associated types** in traits to simplify signatures.

---

## 7. Concurrency Idioms

* Use **channels (`std::sync::mpsc`)** or **shared state with locks (`Mutex`, `RwLock`)** idiomatically for communication and synchronization.

* Prefer **`Arc<T>`** for shared ownership across threads.

* Use **`async` / `await` idioms** for asynchronous programming (if using async Rust).

---

## 8. Miscellaneous Idioms

* **Use `const` and `static` for global constants.**

* **Use `unwrap_or_else` with closures** for deferred computation on `Option` or `Result`.

* Use **`lazy_static` or `once_cell`** crates for expensive global initialization.

---

# Summary: Idiomatic Rust is About Embracing Ownership, Patterns, and Abstractions

Rust idioms often revolve around:

* **Leveraging ownership and borrowing properly** to avoid unnecessary cloning.
* **Using pattern matching** for expressive control flow.
* **Returning and propagating errors with `Result` and `?`**.
* **Using iterators and combinators** instead of manual loops.
* **Writing clear abstractions with traits and generics**.
* **Embracing concurrency with safe, idiomatic primitives**.

---

* Idiomatic **Error Handling** with `Result` and `Option`
* Idiomatic **Iterator Usage** and chaining
* Idiomatic **Ownership and Borrowing** patterns
* Idiomatic **Pattern Matching** and control flow
* Idiomatic **Traits and Generics**
---

## Idiomatic Error Handling with `Result` and `Option`

Rust does **not** have exceptions like many other languages. 
Instead, it has explicit types for errors (`Result`) and optional values (`Option`), making error handling 
explicit and safer.

### Why Rust’s approach is idiomatic:

* Encourages **explicit handling** of errors.
* Avoids hidden failures and crashes.
* Uses types to document which functions can fail.

---

### 1. The `Result` Type

`Result<T, E>` is an enum with two variants:

* `Ok(T)`: success, contains a value of type `T`.
* `Err(E)`: failure, contains an error of type `E`.

Example:

```rust
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
```

---

### 2. Using `Result` idiomatically

#### Propagating errors with the `?` operator

Instead of manually matching and returning errors, use `?` to propagate errors upwards:

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> io::Result<String> {
    let mut file = File::open("username.txt")?;  // If error, return early
    let mut username = String::new();
    file.read_to_string(&mut username)?;
    Ok(username)
}
```

* `?` unwraps the `Ok` value or returns early if `Err`.

#### Handling errors with `match` or combinators

```rust
let result = divide(10.0, 0.0);
match result {
    Ok(val) => println!("Result: {}", val),
    Err(e) => println!("Error: {}", e),
}
```

Or using combinators:

```rust
result.unwrap_or_else(|e| {
    println!("Error: {}", e);
    0.0  // default value on error
});
```

---

### 3. The `Option` Type

`Option<T>` is used when a value might be absent.

Variants:

* `Some(T)` for present value.
* `None` for absence.

Example:

```rust
fn find_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num);
        }
    }
    None
}
```

---

### 4. Idiomatic `Option` usage

* Use `if let` or `match` for explicit handling:

```rust
if let Some(even) = find_even(&[1, 3, 5, 6]) {
    println!("Found even number: {}", even);
} else {
    println!("No even number found.");
}
```

* Use combinators like `map`, `and_then`, `unwrap_or`, etc.:

```rust
let doubled = find_even(&[1, 3, 5, 6]).map(|x| x * 2).unwrap_or(0);
println!("Doubled even number or 0: {}", doubled);
```

---

### 5. When to use `Option` vs `Result`

* Use `Option` when **absence of value is expected** and **not an error**.
* Use `Result` when failure is **exceptional** and should carry an error message.

---

### Summary:

* Use `Result<T, E>` for fallible operations, propagating errors with `?`.
* Use `Option<T>` when values may be missing.
* Use pattern matching and combinators to handle both idiomatically.
* Avoid `unwrap` except in tests or where you are 100% sure it won't fail.
* Use `expect` with descriptive messages to clarify failure reasons.

---
## Idiomatic Iterator Usage and Chaining in Rust

---

### Why iterators are idiomatic in Rust

* They allow **lazy, composable, and efficient** processing of sequences.
* They make code **concise and expressive**.
* They avoid manual index tracking or mutation-heavy loops.

---

### 1. Basic Iterator usage

You get an iterator from collections with `.iter()` or `.into_iter()`

```rust
let numbers = vec![1, 2, 3];
for num in numbers.iter() {
    println!("{}", num);
}
```

---

### 2. Iterator adapters: chaining transformations

Instead of manually writing loops, idiomatic Rust chains iterator methods:

* `.map()` transforms each element
* `.filter()` filters elements by predicate
* `.collect()` gathers the results into a collection

Example:

```rust
let numbers = vec![1, 2, 3, 4, 5];
let evens_squared: Vec<_> = numbers.iter()
    .filter(|&&x| x % 2 == 0)
    .map(|&x| x * x)
    .collect();

println!("{:?}", evens_squared); // [4, 16]
```

---

### 3. Avoid mutable state when possible

Idiomatic iterator chains minimize mutable variables and side effects, improving safety and clarity.

---

### 4. Using `.enumerate()` for indices

If you need index and value:

```rust
for (i, val) in numbers.iter().enumerate() {
    println!("Index {}: Value {}", i, val);
}
```

---

### 5. Consuming vs. non-consuming iterators

* `.iter()` borrows elements.
* `.into_iter()` consumes the collection.
* `.iter_mut()` gives mutable references.

---

### 6. Using `.fold()` for complex reductions

Example: sum of squares

```rust
let sum_squares = numbers.iter()
    .fold(0, |acc, &x| acc + x * x);
```

---

### 7. Laziness of iterators

Iterators are lazy: nothing happens until you consume with `.collect()`, `.fold()`, or a `for` loop.

---

### Summary:

* Prefer iterator chains over manual loops.
* Use `.filter()`, `.map()`, `.fold()` for common operations.
* Use `.collect()` to gather results into collections.
* Use `.enumerate()` to get indices.
* Understand borrowing semantics of `.iter()`, `.into_iter()`, `.iter_mut()`.

---
## Idiomatic Ownership and Borrowing Patterns in Rust

---

### Why ownership and borrowing are at Rust’s core

* Rust enforces **memory safety** without a garbage collector.
* Ownership rules control **who can access and modify data** and when.
* Borrowing lets you access data **without taking ownership**, avoiding unnecessary copies.

---

### 1. Ownership basics

* Each value has a **single owner**.
* When the owner goes out of scope, value is dropped.
* Moving ownership transfers responsibility.

Example:

```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 is moved, can't use s1 anymore
// println!("{}", s1); // Error!
println!("{}", s2);
```

---

### 2. Borrowing: references (`&T`) and mutable references (`&mut T`)

* Borrowing **allows temporary access without ownership**.
* Immutable references (`&T`) allow multiple readers.
* Mutable references (`&mut T`) allow exactly one writer.

Example:

```rust
fn print_str(s: &String) {
    println!("{}", s);
}

let s = String::from("hello");
print_str(&s);  // borrow immutably
println!("{}", s); // s still usable here
```

---

### 3. Prefer borrowing over cloning

* Cloning allocates and copies data — expensive.
* Use references when possible to avoid unnecessary copies.

Non-idiomatic:

```rust
fn greet(name: String) {
    println!("Hello, {}", name);
}

let name = String::from("Alice");
greet(name.clone());  // unnecessary clone
```

Idiomatic:

```rust
fn greet(name: &str) {
    println!("Hello, {}", name);
}

let name = String::from("Alice");
greet(&name);  // borrow, no clone
```

---

### 4. Mutable references rules

* Only **one mutable reference** allowed at a time.
* No immutable references coexist with a mutable reference.

```rust
let mut s = String::from("hello");

let r1 = &mut s;
// let r2 = &mut s;  // Error: second mutable borrow
println!("{}", r1);
```

---

### 5. Slices for borrowing parts of data

Use slices (`&[T]` or `&str`) to borrow subsections without copying:

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

---

### 6. Lifetimes: explicit and implicit

* Lifetimes ensure references remain valid.
* Usually, Rust infers lifetimes.
* Use explicit lifetime annotations in complex cases (e.g., functions returning references).

---

### 7. `Cow` (Clone-on-write) for flexible borrowing/ownership

```rust
use std::borrow::Cow;

fn print_name(name: Cow<str>) {
    println!("{}", name);
}

let borrowed = "Alice";
print_name(Cow::from(borrowed)); // borrowed

let owned = String::from("Bob");
print_name(Cow::from(owned));    // owned
```

---

### Summary

* Embrace ownership: one owner per value.
* Prefer borrowing references (`&T`, `&mut T`) over cloning.
* Follow Rust’s borrowing rules: one mutable or many immutable refs.
* Use slices to borrow parts efficiently.
* Understand lifetimes for reference validity.
* Use `Cow` for flexible API design.

---
## Idiomatic Pattern Matching and Control Flow in Rust
---

### Why pattern matching is idiomatic in Rust

* Pattern matching is **exhaustive and safe**.
* It enables **clear, declarative handling** of different data shapes and control flows.
* It integrates tightly with enums, options, and results.

---

### 1. Using `match` for exhaustive control flow

Rust forces you to handle all cases explicitly:

```rust
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn action(light: TrafficLight) {
    match light {
        TrafficLight::Red => println!("Stop"),
        TrafficLight::Yellow => println!("Slow down"),
        TrafficLight::Green => println!("Go"),
    }
}
```

---

### 2. Using `if let` for simpler matches when you care about one variant

Instead of a full `match`, `if let` handles one variant concisely:

```rust
let some_option = Some(5);

if let Some(x) = some_option {
    println!("Value: {}", x);
} else {
    println!("No value");
}
```

---

### 3. Using `while let` for repeated pattern matching in loops

```rust
let mut stack = Vec::new();

stack.push(1);
stack.push(2);
stack.push(3);

while let Some(top) = stack.pop() {
    println!("{}", top);
}
```

---

### 4. Destructuring in `match`, `if let`, and function arguments

You can destructure tuples, structs, and enums:

```rust
let point = (3, 7);

match point {
    (0, y) => println!("On the y axis at {}", y),
    (x, 0) => println!("On the x axis at {}", x),
    (x, y) => println!("At point ({}, {})", x, y),
}
```

Or destructure struct fields:

```rust
struct Person { name: String, age: u8 }

let p = Person { name: "Alice".into(), age: 30 };

match p {
    Person { name, age: 30 } => println!("{} is 30 years old", name),
    Person { name, age } => println!("{} is {} years old", name, age),
}
```

---

### 5. Using `_` and `..` to ignore parts

* `_` ignores a value.
* `..` ignores remaining parts in structs/tuples.

```rust
match some_tuple {
    (x, _) => println!("First: {}", x),
}

match some_struct {
    Person { name, .. } => println!("Name: {}", name),
}
```

---

### 6. Guards: add extra condition to patterns

```rust
let num = Some(4);

match num {
    Some(x) if x < 5 => println!("Small number: {}", x),
    Some(x) => println!("Large number: {}", x),
    None => println!("No number"),
}
```

---

### 7. Control flow with `if`, `else if`, and `else`

Idiomatic Rust uses `match` for exhaustive branching, and `if` for simple conditions.

---

### Summary:

* Use `match` for exhaustive and safe control flow.
* Use `if let` and `while let` for concise conditional matching.
* Destructure data to extract values clearly.
* Use `_`, `..` to ignore unused parts.
* Use guards to add conditional checks inside patterns.

---
## Idiomatic Traits and Generics in Rust
---

### Why traits and generics are idiomatic

* Traits enable **polymorphism and abstraction** without inheritance.
* Generics enable **code reuse** across types.
* Combined, they provide powerful ways to write flexible, reusable, and safe code.

---

### 1. Traits: defining shared behavior

Traits define method signatures that types can implement.

```rust
trait Speak {
    fn speak(&self);
}

struct Dog;

impl Speak for Dog {
    fn speak(&self) {
        println!("Woof!");
    }
}
```

---

### 2. Using traits for abstraction and polymorphism

You can write functions that accept any type implementing a trait:

```rust
fn make_speak<T: Speak>(animal: T) {
    animal.speak();
}
```

Or using `impl Trait` syntax for simpler cases:

```rust
fn make_speak(animal: impl Speak) {
    animal.speak();
}
```

---

### 3. Trait bounds on generics

Specify what traits generic types must implement:

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
```

---

### 4. Associated Types vs Generic Parameters

Traits can define **associated types** to reduce complexity:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Instead of:

```rust
trait Iterator<T> {
    fn next(&mut self) -> Option<T>;
}
```

Associated types help make trait bounds easier to read.

---

### 5. Default implementations in traits

Traits can provide default method implementations that types can override:

```rust
trait Animal {
    fn speak(&self) {
        println!("Animal sound");
    }
}

struct Cat;

impl Animal for Cat {
    fn speak(&self) {
        println!("Meow");
    }
}
```

---

### 6. Blanket implementations

You can implement a trait for any type that satisfies certain bounds:

```rust
impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}
```

---

### 7. Using trait objects for dynamic dispatch

When you want to store heterogeneous types implementing the same trait:

```rust
fn speak_all(animals: &[&dyn Speak]) {
    for animal in animals {
        animal.speak();
    }
}
```

Note: trait objects come with runtime cost (dynamic dispatch).

---

### Summary

* Use traits to define behavior.
* Use generics with trait bounds to write reusable code.
* Prefer associated types over generic parameters in traits where applicable.
* Use default method implementations to reduce boilerplate.
* Use trait objects for dynamic dispatch if needed, but prefer static dispatch for performance.

---


## Examples : 
Sure! Here are some **short examples** and **exercises** on traits and generics.

---

### Example 1: Simple Trait and Implementation

```rust
trait Greet {
    fn greet(&self);
}

struct Person {
    name: String,
}

impl Greet for Person {
    fn greet(&self) {
        println!("Hello, {}!", self.name);
    }
}

fn say_hello<T: Greet>(greeter: T) {
    greeter.greet();
}

fn main() {
    let p = Person { name: "Alice".into() };
    say_hello(p);
}
```

---

### Example 2: Generic Function with Trait Bound

```rust
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let nums = vec![3, 5, 2, 8, 1];
    println!("Largest: {}", largest(&nums));
}
```

---

### Exercise 1: Implement a trait `Summary` with a method `summary` that returns a `String`. Implement it for a struct `Article` with fields `headline` and `author`.

---

### Exercise 2: Write a generic function `print_pair` that takes a tuple of two elements implementing the `Display` trait and prints them.

---

