# OOP

## Object-Oriented Programming Features of Rust

How Rust supports **Object-Oriented Programming (OOP)** concepts — but in its *own unique, safe, and flexible way*.

By the end of this tutorial, you’ll understand:

* How Rust fits into OOP’s core principles
* How to use **structs** and **traits** to model objects and behaviors
* What **trait objects** are and how they enable **dynamic dispatch**
* How to build **extensible and polymorphic** systems safely

---

## 1.0 What Is OOP, Conceptually?

Traditional OOP (as in Java, C++, Python) is built around three main principles:

| Concept           | Meaning                                                   |
| ----------------- | --------------------------------------------------------- |
| **Encapsulation** | Grouping data and methods that operate on it              |
| **Inheritance**   | Deriving new behavior or structure from existing ones     |
| **Polymorphism**  | Using shared interfaces to handle multiple concrete types |

Rust doesn’t have *class-based* inheritance, but it *does* support all three ideas — in more flexible and safer ways.

---

## 2.0 Encapsulation: Using Structs and `impl`

Encapsulation in Rust is done using **structs** (for data) and **`impl` blocks** (for methods).

```rust
pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

impl AveragedCollection {
    pub fn new() -> Self {
        Self {
            list: vec![],
            average: 0.0,
        }
    }

    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let result = self.list.pop();
        if result.is_some() {
            self.update_average();
        }
        result
    }

    pub fn average(&self) -> f64 {
        self.average
    }

    fn update_average(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.average = total as f64 / self.list.len() as f64;
    }
}
```

`list` and `average` are **private** by default — only accessible inside the module.
The API controls how data is mutated, maintaining invariants.

This is encapsulation without `class` keywords.

---

## 3.0 Inheritance Alternatives in Rust

Rust avoids “inheritance” in favor of **composition** and **traits**.

Instead of subclassing, you *compose* structs and *share behavior* via traits.

Example:

```rust
trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Drawing Button: {}", self.label);
    }
}

struct Checkbox {
    checked: bool,
}

impl Draw for Checkbox {
    fn draw(&self) {
        println!("Drawing Checkbox: {}", self.checked);
    }
}
```

Each type implements `Draw` independently — no inheritance required.

---

## 4.0 Polymorphism with Traits

Polymorphism means “one interface, many implementations.”
In Rust, traits provide exactly that.

```rust
fn render(item: &impl Draw) {
    item.draw();
}

fn main() {
    let b = Button { label: "OK".to_string() };
    let c = Checkbox { checked: true };

    render(&b);
    render(&c);
}
```

Output:

```
Drawing Button: OK
Drawing Checkbox: true
```

✅ Any type that implements `Draw` can be passed to `render`.
✅ This is **static dispatch** — the compiler knows at compile time which `draw()` to call.

---

## 5.0 Dynamic Dispatch with Trait Objects (`dyn Trait`)

Sometimes, you want a *collection* of objects that implement the same trait,
but you don’t know their types at compile time.

Enter **trait objects** — `Box<dyn Trait>` or `&dyn Trait`.

Example:

```rust
trait Draw {
    fn draw(&self);
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for component in &self.components {
            component.draw(); // Dynamic dispatch
        }
    }
}

struct Button {
    label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("Drawing Button: {}", self.label);
    }
}

struct Checkbox {
    checked: bool,
}
impl Draw for Checkbox {
    fn draw(&self) {
        println!("Drawing Checkbox: {}", self.checked);
    }
}

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(Button { label: "OK".into() }),
            Box::new(Checkbox { checked: false }),
        ],
    };

    screen.run();
}
```

Output:

```
Drawing Button: OK
Drawing Checkbox: false
```

✅ Each item in the vector is a different type.
✅ Rust uses a *vtable* (virtual table) for dynamic dispatch — similar to OOP languages.
✅ But only through **explicit trait objects**, keeping performance predictable.

---

## 6.0 Static vs Dynamic Dispatch

| Dispatch Type | Syntax                                    | Performance                      | When to Use                                                     |
| ------------- | ----------------------------------------- | -------------------------------- | --------------------------------------------------------------- |
| **Static**    | `fn f(x: &impl Trait)`                    | Fast (compile-time)              | When all types are known                                        |
| **Dynamic**   | `fn f(x: &dyn Trait)` or `Box<dyn Trait>` | Slightly slower (runtime lookup) | When you need heterogenous collections or plugin-style behavior |

🧠 Think of it as:

> Static = “templates”
> Dynamic = “virtual methods”

---

## 7.0  Example: Building a GUI Framework

Let’s combine everything:

```rust
trait Draw {
    fn draw(&self);
}

struct Button {
    label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("Rendering button: {}", self.label);
    }
}

struct Slider {
    value: i32,
}
impl Draw for Slider {
    fn draw(&self) {
        println!("Rendering slider at value: {}", self.value);
    }
}

struct Screen {
    components: Vec<Box<dyn Draw>>,
}

impl Screen {
    fn run(&self) {
        for c in &self.components {
            c.draw();
        }
    }
}

fn main() {
    let ui = Screen {
        components: vec![
            Box::new(Button { label: "Submit".into() }),
            Box::new(Slider { value: 42 }),
        ],
    };
    ui.run();
}
```

✅ `Button` and `Slider` implement the same trait.
✅ `Screen` stores them using `Box<dyn Draw>` for dynamic dispatch.
✅ Adding new UI elements doesn’t require changing `Screen`.

This is **polymorphism through composition**, not inheritance.

---

## 8.0 Trait Objects and Safety

* Only **object-safe traits** can be used as trait objects.
* A trait is **object-safe** if:

  * It doesn’t use `Self` in a way that requires knowing the concrete type.
  * All its methods have a known signature (no generic methods).

✅ Example (object-safe):

```rust
trait Drawable {
    fn draw(&self);
}
```

❌ Not object-safe:

```rust
trait Cloneable {
    fn clone_me<T>(&self) -> T; // generic method — not allowed for dyn
}
```

---

## 9.0 Summary

| OOP Concept       | Rust Equivalent        | Example              |
| ----------------- | ---------------------- | -------------------- |
| **Encapsulation** | Structs + `impl`       | `AveragedCollection` |
| **Inheritance**   | Traits + Composition   | `Button`, `Checkbox` |
| **Polymorphism**  | Traits + Trait Objects | `dyn Draw`           |

Rust favors **composition over inheritance**, giving you:

* Safety (no shared mutable base class state)
* Explicit behavior
* Zero-cost abstractions when possible

---

## 10.0 Key Takeaways

> Rust’s OOP features focus on **behavior, not hierarchy**.

* ✅ You can group data + methods (encapsulation)
* ✅ You can share behavior through traits (composition)
* ✅ You can achieve polymorphism using trait objects
* 🚫 You can’t inherit implementation — but you can build extensible, safe systems

---

## ✅ Final Example: Plugin System

```rust
trait Plugin {
    fn name(&self) -> &str;
    fn run(&self);
}

struct Logger;
impl Plugin for Logger {
    fn name(&self) -> &str { "Logger" }
    fn run(&self) { println!("Logging..."); }
}

struct Notifier;
impl Plugin for Notifier {
    fn name(&self) -> &str { "Notifier" }
    fn run(&self) { println!("Sending notification..."); }
}

fn main() {
    let plugins: Vec<Box<dyn Plugin>> = vec![
        Box::new(Logger),
        Box::new(Notifier),
    ];

    for p in plugins {
        println!("Running plugin: {}", p.name());
        p.run();
    }
}
```

Output:

```
Running plugin: Logger
Logging...
Running plugin: Notifier
Sending notification...
```

✅ Simple, extensible, polymorphic — the Rust way of OOP.

---

## 🧭 Key Idea to Remember

> Rust’s traits and trait objects give you **OOP-like power** with **Rust-like safety**.
> Instead of deep class hierarchies, you build **composable behaviors** that are safe, explicit, and efficient.

---
