# Traits:   

* What traits are
* How to define and implement them
* Using traits in functions
* Default method implementations
* Trait bounds

---

## What Are Traits in Rust?

**Traits** in Rust are similar to **interfaces** in other languages. 
They define **shared behavior** that multiple types can implement.

Think of a trait as a **contract**: any type that implements the trait must provide the behavior (methods) 
the trait specifies.

---

## 🛠️ Defining a Trait

Let’s start by defining a simple trait called `Summary`:

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

This defines a trait named `Summary` with a required method `summarize()` that returns a `String`.

---

## Implementing a Trait for a Type


1. 
---
Now let’s implement the `Summary` trait for a struct.

```rust
pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}
```

This tells Rust: “`NewsArticle` now implements the `Summary` trait.”

You can now call `.summarize()` on a `NewsArticle`.

```rust
fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 2.0 Released"),
        author: String::from("Ferris Crab"),
        content: String::from("It's blazing fast..."),
    };

    println!("Article summary: {}", article.summarize());
}
```
2. Another example
---

Define a Trait: ( that give a short description of themselves. )
```rust 
pub trait Describable {
    fn describe(&self) -> String;
}
```
So now any **type** implementing the `Describable` must define the method `describe`.

```rust 
pub struct MathProblem {
    pub question: String,
    pub difficulty: u8,
}
```

```rust 
impl Describable for MathProblem {
    fn describe(&self) -> String {
        format!("Math Problem (Level {}): {}", self.difficulty, self.question)
    }
}
```
Now `MathProblem` has the behavior defined by `Describable`.

Usage:
```rust 
fn main() {
    let problem = MathProblem {
        question: String::from("What is the derivative of x^2?"),
        difficulty: 3,
    };

    println!("{}", problem.describe());
}
```

---

## 💬 Default Implementations

You can provide a **default implementation** for a method in a trait:

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

Now if a type implements `Summary` but doesn't provide its own `summarize`, the default will be used.

```rust
pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {} // Uses the default summarize

fn main() {
    let tweet = Tweet {
        username: String::from("@rustacean"),
        content: String::from("Rust is great!"),
    };

    println!("Tweet summary: {}", tweet.summarize()); // (Read more...)
}
```

---

## ✨ Traits in Function Parameters

You can use traits to accept **any type that implements a trait** in a function.

### Method 1: `impl Trait` Syntax

```rust
fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
```

This function accepts any type that implements `Summary`.

### Method 2: Trait Bound Syntax (More Flexible)

```rust
fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}
```

Both versions are valid. The second gives more control when multiple generic parameters are involved.

---

## Multiple Trait Bounds

1.
---
In Rust, multiple trait bounds allow you to specify that a `generic type` `T` must implement more than one 
trait to be used in a **function or a struct definition**. 

This is a common requirement when you need a generic type `T` to support a combination of behaviors defined 
by different traits.

The Syntax
You can specify multiple trait bounds using the `+` operator.

- **In a Function Definition** :
When defining a generic function, place the trait bounds after the colon (:) and separate them with a (+).

```rust 
// The generic type T must implement BOTH TraitA and TraitB
fn my_function<T: TraitA + TraitB>(item: T) {
    // function body
}
```

- **Using the where Clause (Recommended for Clarity)** 

For functions with several generic types or many trait bounds, the where clause is generally preferred as 
it keeps the function signature cleaner.

```rust 
fn my_function<T, U>(item_t: T, item_u: U)
where
    T: TraitA + TraitB, // T must implement TraitA AND TraitB
    U: TraitC + TraitD, // U must implement TraitC AND TraitD
{
    // function body
}
```
Example: Printing and Cloning
example where a function needs a generic type to be both printable (via the standard library's Debug trait) 
and clonable (via the Clone trait).

Step 1: **Define Traits and Struct**
Using the built-in `std::fmt::Debug` and `std::clone::Clone` traits and a simple `struct` that derives them

```rust 
#[derive(Debug, Clone)]
struct Appliance {
    name: String,
    power_watts: u32,
}
```

Step2: **Implement the Function with Multiple Trait Bounds** 

For this we define a function `print_and_clone` that takes a generic type `T`
This type must implement both `Debug` so it can be printed with `{:?}` and `Clone` so that we can create a
copy:
```rust 

// This function requires T to implement both Debug AND Clone.
fn print_and_clone<T>(item: &T) -> T
where
    T: std::fmt::Debug + std::clone::Clone,
{
    // Use the Debug trait (via the {:?} format specifier)
    println!("Original item: {:?}", item);

    // Use the Clone trait to create a copy
    let cloned_item = item.clone();

    println!("Cloned item created successfully.");

    // Return the clone
    cloned_item
}
```
Step 3: **Call the Function**
In the *main* we created an instance of our `Appliance` struct and call the generic function:

```rust
fn main() {
    let original_fridge = Appliance {
        name: String::from("Smart Fridge"),
        power_watts: 150,
    };

    println!("\n--- Function Call ---");

    // Call the function, it works because Appliance implements both Debug and Clone.
    let cloned_fridge = print_and_clone(&original_fridge);

    println!("--- Results ---");
    println!("Original: {:?}", original_fridge);
    println!("Cloned:   {:?}", cloned_fridge);

    // Verify they are separate objects by modifying the clone
    let _ = cloned_fridge.name.push_str(" (Modified)");

    println!("\nAfter modification check (Original should be unchanged):");
    println!("Original: {:?}", original_fridge);
    println!("Cloned:   {:?}", cloned_fridge);
}
```


2. 
---

What if a type needs to implement **more than one trait**?

```rust
fn notify<T: Summary + std::fmt::Display>(item: &T) {
    println!("Info: {}", item);
}
```

Or with `where` clause for readability:

```rust
fn notify<T>(item: &T)
where
    T: Summary + std::fmt::Display,
{
    println!("Info: {}", item);
}
```
---

## Returning Types that Implement Traits

Sometimes you want a function to **return something that implements a trait**.

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("@someone"),
        content: String::from("Look at this!"),
    }
}
```

This works **only if all return paths return the same type** that implements the trait.

i.e:

A function usually returns a concrete type (like u32, String, or MyStruct). 
However, there are times when you want a function to return **any type** that satisfies a certain trait 
requirement, without having to name the specific **struct** or **enum**.

This is achieved using the `impl Trait` syntax in the return position.

The Syntax:  `-> impl Trait` 

By using `-> impl TraitName` as the return type, you tell the Rust compiler:
This function will return some concrete, hidden type.
Crucially, whatever type it is, it is **guaranteed** to implement `TraitName`.

```rust 
// This function returns some type that implements the Iterator trait.
fn get_data_iterator() -> impl Iterator<Item = u32> {
    // The actual returned type might be a Vec<u32>::IntoIter,
    // but the caller only knows it's *some* iterator.
    vec![1, 2, 3].into_iter()
}
```
**Why Use impl Trait for Return Types?**
The primary benefits are:
- **Hiding Implementation Details**: 
    It allows you to change the internal concrete type you return (e.g., swapping a Vec<T> for a HashMap<T>)
    without breaking the calling code, as long as the new type still implements the required trait.

- **Working with Closures**: Closures have unique, unnameable types. 
    The `impl Fn...` syntax is the standard way to return a closure from a function.

- **Returning Complex Types**: 
    It simplifies function signatures that would otherwise involve very long, confusing combinations of 
    traits and concrete types (like complex iterators).

Example: A generic `Shape` Creator: 

Imagine we have a simple `Shape` trait, and we want a factory function that returns **some type of shape**
based on an input, but we don't want to expose the specific shape struct (Eg: `Circle` or `Square`) to the
caller

Step 1: Define the Trait and Concrete Types
We define a trait `Shape` and two distinct structs, `Circle` and `Square`, that implement it.

```rust 
// 1. Define the Trait
trait Shape {
    fn area(&self) -> f64;
    fn description(&self) -> String;
}

// 2. Define Concrete Structs
struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

// 3. Implement the Trait for the Structs
impl Shape for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
    fn description(&self) -> String {
        format!("Circle with radius {}", self.radius)
    }
}

impl Shape for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
    fn description(&self) -> String {
        format!("Square with side length {}", self.side)
    }
}

```

Step 2: Implement the Factory Function
Now we create a function, `make_shape`, that returns a **type that implements Shape**. 
Note that it uses `-> impl Shape`, not `-> Circle` or `-> Square`.
```rust 
// This function returns *some type* that implements the Shape trait.
fn make_shape(is_round: bool, value: f64) -> impl Shape {
    if is_round {
        // Here we return a concrete Circle struct
        Circle { radius: value }
    } else {
        // Here we return a concrete Square struct
        Square { side: value }
    }
    // IMPORTANT: The function must return only ONE concrete type, regardless
    // of the code path. This example will NOT compile (see explanation below).
}
```
The Compiler Error and Solution
The function above will cause a compiler error!

```bash
error[E0308]: mismatched types
   |
11 |     if is_round {
   |     ------------ expected because of this
12 |         Circle { radius: value }
   |         ----------------------- this is found to be of type `Circle`
13 |     } else {
14 |         Square { side: value }
   |         ^^^^^^^^^^^^^^^^^^^^^ expected `Circle`, found `Square`
   |
   = note: expected type `Circle`
              found type `Square`
```

Reason: 
When you use `-> impl Trait`, the compiler needs to determine one single concrete type for that function's 
return signature. 
In the example above, the compiler sees that the function might return a Circle or a Square. 
Since Circle and Square are different types, even though they both implement Shape, the compiler gets 
confused.

The Solution: Boxed Trait Objects (Box<dyn Trait>)
To return *different* types that share a trait from a function, you must use **Dynamic Dispatch** with a 
**Trait Object**. 
This involves putting the structs in a box (Box) and using the dyn keyword: 
```rust 
// Solution: Use Box<dyn Trait> when returning different types that share a trait.
fn make_shape_dyn(is_round: bool, value: f64) -> Box<dyn Shape> {
    if is_round {
        // Box::new puts the Circle on the heap
        Box::new(Circle { radius: value })
    } else {
        // Box::new puts the Square on the heap
        Box::new(Square { side: value })
    }
}
```

Step 3: Call the Dynamic Function
Now, the calling code doesn't need to know if it's dealing with a Circle or a Square; it only knows it 
has a trait object that implements Shape.
```rust 
fn main() {
    println!("--- Dynamic Shape Creator ---");

    // Get a shape (a Circle, but we don't know it)
    let shape_one = make_shape_dyn(true, 5.0);

    // Get a second shape (a Square, but we don't know it)
    let shape_two = make_shape_dyn(false, 10.0);

    println!("Shape 1: {} (Area: {:.2})", shape_one.description(), shape_one.area());
    println!("Shape 2: {} (Area: {:.2})", shape_two.description(), shape_two.area());
}
```
Summarize:
Syntax, `-> impl Trait`, **Static Dispatch** (Compile-time), this must return one concrete type across all
code paths. This should be used when Returning closures or simplifying iterator types. (Faster)

`-> Box<dyn Trait>` **Dynamic Dispatch** (Run-time), This can return multiple different concrete types that 
implement the trait. Should be used when Factory functions, returning different structs based on logic. 
(Slightly Slower).


---

##  Example

```rust
trait Summary {
    fn summarize(&self) -> String;
}

struct BlogPost {
    title: String,
    author: String,
    body: String,
}

impl Summary for BlogPost {
    fn summarize(&self) -> String {
        format!("{} by {}", self.title, self.author)
    }
}

fn print_summary(item: &impl Summary) {
    println!("Summary: {}", item.summarize());
}

fn main() {
    let post = BlogPost {
        title: String::from("Traits in Rust"),
        author: String::from("Jane Doe"),
        body: String::from("Traits are a way to define shared behavior..."),
    };

    print_summary(&post);
}
```

Output:

```
Summary: Traits in Rust by Jane Doe
```

---

# Summary

| Concept                      | Explanation                          |
| ---------------------------- | ------------------------------------ |
| `trait`                      | Defines shared behavior              |
| `impl TraitName for Type {}` | Implements the trait for a type      |
| Default Methods              | Optional default implementations     |
| `impl Trait` in params       | Accept any type implementing a trait |
| Trait Bounds                 | Combine traits with `+` or `where`   |
| `impl Trait` in returns      | Return types that implement traits   |

---

Would you like interactive exercises or quiz questions based on this section?
