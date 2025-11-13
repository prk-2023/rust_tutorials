# Patterns and Matching:

 *Patterns and Matching*** — one of the most expressive, elegant, and uniquely *Rust-y* parts of the language.

If smart pointers and traits teach you how Rust *manages and organizes data*, **patterns** teach you how to *work with and extract data* safely and concisely.

---

## Tutorial: Chapter 18 — Patterns and Matching in Rust

---

## 🧠 What You’ll Learn

By the end of this tutorial, you’ll understand:

* What patterns are and where they can be used
* How `match` and `if let` work
* How to destructure structs, enums, tuples, and arrays
* How to use pattern guards (`if` conditions in matches)
* How to match with `@`, `_`, and `..` for advanced control

---

## 1️⃣ What Is a Pattern?

A **pattern** in Rust is a *structure used to match and destructure values*.
Patterns appear anywhere Rust needs to look inside a value — like `match`, `let`, `if let`, `while let`, and function parameters.

🧩 Examples of patterns:

```rust
let (x, y) = (5, 10);       // tuple destructuring
if let Some(value) = opt {  // optional pattern
    println!("Got {value}");
}
match number {              // match expression pattern
    1 => println!("One!"),
    _ => println!("Something else."),
}
```

---

## 2️⃣ `match`: The Powerhouse of Pattern Matching

Rust’s `match` is exhaustive and safe — you must handle *every* possible case.

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}

fn main() {
    let coin = Coin::Dime;
    println!("Value = {} cents", value_in_cents(coin));
}
```

✅ Each arm matches one variant.
✅ No “fallthrough” like in C or Java.
✅ Compiler checks exhaustiveness — you can’t forget a variant.

---

## 3️⃣ Patterns Can Bind Values

You can bind values inside patterns:

```rust
enum Coin {
    Quarter(String), // State name
    Dime,
}

fn describe(coin: Coin) {
    match coin {
        Coin::Quarter(state) => println!("State quarter from {state}!"),
        Coin::Dime => println!("Just a dime."),
    }
}

fn main() {
    describe(Coin::Quarter("Texas".into()));
}
```

Output:

```
State quarter from Texas!
```

🧠 You’re *extracting data* from inside the enum — one of Rust’s greatest strengths.

---

## 4️⃣ `Option<T>` Matching

A super common pattern:

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        None => None,
    }
}

fn main() {
    println!("{:?}", plus_one(Some(5))); // Some(6)
    println!("{:?}", plus_one(None));    // None
}
```

✅ `Option<T>` avoids `null`.
✅ `match` makes it explicit when something is absent.

---

## 5️⃣ `if let` — A Simpler `match`

If you only care about one case, use `if let`.

```rust
let config = Some(10);

if let Some(value) = config {
    println!("Config set to {value}");
} else {
    println!("Using default config");
}
```

Equivalent `match`:

```rust
match config {
    Some(value) => println!("Config set to {value}"),
    None => println!("Using default config"),
}
```

🧠 Use `if let` for concise single-pattern logic.

---

## 6️⃣ `while let` — Loop While a Pattern Matches

```rust
let mut stack = vec![1, 2, 3];

while let Some(top) = stack.pop() {
    println!("Popped: {top}");
}
```

✅ Keeps looping while `stack.pop()` returns `Some(value)`
✅ Automatically ends when it returns `None`

Output:

```
Popped: 3
Popped: 2
Popped: 1
```

---

## 7️⃣ Matching and Destructuring Structs

You can destructure structs directly in patterns:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x: 0, y } => println!("On the Y axis at {y}"),
        Point { x, y: 0 } => println!("On the X axis at {x}"),
        Point { x, y } => println!("On neither axis: ({x}, {y})"),
    }
}
```

✅ Each pattern describes the *shape* of the data.
✅ You can rename or ignore fields.

---

## 8️⃣ Destructuring Enums, Tuples, and Nested Data

```rust
enum Color {
    Rgb(i32, i32, i32),
    Hsv(i32, i32, i32),
}

fn print_color(c: Color) {
    match c {
        Color::Rgb(r, g, b) => println!("Red: {r}, Green: {g}, Blue: {b}"),
        Color::Hsv(h, s, v) => println!("Hue: {h}, Saturation: {s}, Value: {v}"),
    }
}

fn main() {
    print_color(Color::Rgb(255, 128, 0));
}
```

✅ Pattern matches can go deep — you can unpack any structure recursively.

---

## 9️⃣ Ignoring Values with `_` and `..`

* `_` ignores a single value.
* `..` ignores “the rest.”

```rust
fn main() {
    let triple = (1, 2, 3);

    match triple {
        (x, _, z) => println!("First = {x}, Third = {z}"),
    }

    let Point { x, .. } = Point { x: 5, y: 10 };
    println!("x = {x}");
}
```

✅ `_` avoids “unused variable” warnings.
✅ `..` is great for large structs or tuples.

---

## 🔟 Match Guards (`if` in Matches)

You can add extra conditions to a pattern:

```rust
let num = 5;

match num {
    n if n < 0 => println!("Negative"),
    n if n == 0 => println!("Zero"),
    n if n > 0 && n < 10 => println!("Small positive number"),
    _ => println!("Big number"),
}
```

✅ Guards allow additional logic without nesting `if` inside `match`.

---

## 1️⃣1️⃣ The `@` Binding Operator

Use `@` to capture a value *and* test it in the same pattern.

```rust
enum Message {
    Hello { id: i32 },
}

fn main() {
    let msg = Message::Hello { id: 5 };

    match msg {
        Message::Hello { id: id_var @ 3..=7 } => {
            println!("id in range: {id_var}");
        }
        Message::Hello { id: 10..=12 } => println!("id in another range"),
        Message::Hello { id } => println!("id = {id}"),
    }
}
```

✅ `id_var @ 3..=7` binds the value if it matches the range.

---

## 1️⃣2️⃣ Pattern Matching Everywhere

Patterns work in many contexts:

| Context             | Example                                           |
| ------------------- | ------------------------------------------------- |
| `let` statements    | `let (x, y) = (1, 2);`                            |
| Function parameters | `fn add((x, y): (i32, i32)) -> i32 { x + y }`     |
| `if let`            | `if let Some(x) = option {}`                      |
| `while let`         | `while let Some(v) = stack.pop() {}`              |
| `for` loops         | `for (index, value) in vec.iter().enumerate() {}` |
| `match` arms        | `match option { Some(v) => ..., None => ... }`    |

🧠 You can use patterns **anywhere** Rust needs to unpack data.

---

## ✅ Final Example: Matching Everything Together

```rust
#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle(f64, f64, f64),
}

fn describe(shape: Shape) {
    match shape {
        Shape::Circle { radius } => println!("Circle of radius {radius}"),
        Shape::Rectangle { width, height } if width == height => {
            println!("Square with side {width}")
        }
        Shape::Rectangle { width, height } => {
            println!("Rectangle {width}×{height}")
        }
        Shape::Triangle(a, b, c) => println!("Triangle with sides {a}, {b}, {c}"),
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle { radius: 2.5 },
        Shape::Rectangle { width: 3.0, height: 3.0 },
        Shape::Triangle(3.0, 4.0, 5.0),
    ];

    for s in shapes {
        describe(s);
    }
}
```

Output:

```
Circle of radius 2.5
Square with side 3
Triangle with sides 3, 4, 5
```

✅ You’ve used destructuring, guards, and enums — the full pattern toolkit.

---

## 🧭 Key Takeaways

> Patterns let you **safely deconstruct complex data**, **express logic concisely**, and **make invalid states unrepresentable**.

* `match` is exhaustive and powerful.
* `if let` and `while let` simplify common cases.
* Patterns can destructure any kind of data.
* Guards, `_`, and `@` make matches expressive and clear.
* They integrate deeply with Rust’s ownership and type system.

---

## ✅ Summary Table

| Feature               | Description                            |
| --------------------- | -------------------------------------- |
| `match`               | Exhaustive branching by pattern        |
| `if let`, `while let` | Simplified single-pattern control flow |
| `_` / `..`            | Ignore unused data                     |
| `@`                   | Bind and test simultaneously           |
| Guards                | Add extra conditions                   |
| Destructuring         | Break data into parts safely           |

---

Would you like me to continue with **Chapter 19 — “Advanced Features”**, where we explore lifetimes, unsafe Rust, advanced traits, macros, and raw pointers — the deep “power user” side of the language?

