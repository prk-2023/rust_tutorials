# Rust Traits: A Comprehensive Guide

Rust traits are a very powerful feature of the language. 

- Enables Polymorphism,
- Code Reuse 
- Type System Flexibility.


## What are traits?

Rust Traits define *shared behaviour that types can implement*. 
These are similar to interfaces in C++,Java,TypeScript ( check below References )
Rust traits are more powerful then what Interfaces provide.

```rust 
/*  --- define s trait  -- 
    Any Type that implements this triat should define 'speak' method 
*/
trait Speak {
    fn speak(&self) -> String;

}

// Implement for a type
struct Dog {
    name: String,
}
impl Speak for Dog {
    fn speak(&self) -> String {
        format!("{} says wolf", self.name)
    }
}
 struct Cat;
 impl Speak for Cat {
    fn speak (&self) -> String {
        "Meow!".to_string()
    }
 }

```
## Trait related Topics to Cover:

### 1. Basics of Trait Implementation:
- Define traits with methods.
- Implementing traits for types.
- Associated functions in traits.

To declare a trait the syntax is as:

```rust 
trait Printable {
    fn print(&self);
}
```
This declares a trait with one method `print`

Traits can also provide **Default Implementation**

```rust 
trait Printable {

    fn print(&self) {
        println!("Default print");
    }
}
```

Implementing Traits:
```rust 

struct Person {
    name: String,
}

impl Printable for Person {
    fn print(&self) {
        println!("Person: {} ", self.name);
    }
}

==> You can implelemt *traits* for enums, and other types ( even primitive types with rules ).
```

Associated functions  in traits: ( like static methods )
```rust 
trait Math {
    fn zero() -> Self;
}
```
### 2. Trait Bounds:
- Generic function with trait bounds. 
- `where` clause for complex bounds.

Traits can be used as **constraints** (bounds) for generic types 

```rust 
fn display<T: Printable> (item: T) {
    item.print();
}
```
Equivalent shorthand is by using `where` clause:
```rust 
fn display<T> (item: T)
where 
    T: Printable,
{
    item.print();
}
```
Another example:
```rust 
fn make_sound<T: Speak>(animal: &T) {
    println!("{}"animal.speak());
}

//Equivalent with where clause
fn make_sound<T> (animal: &T) 
where 
    T: Speak,
    {
        println!("{}"animal.speak());
    }
```

Multiple trait bounds: 

```rust 
fn do_stuff<T: Clone + Printable(item: T) { .... }

```

### 3. Traits and Structs together:
Using traits to define behaviour:

```rust 
trait Area {
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Area for Circle {
    fn area(&self) -> f64 {
        3.14 * self.radius * self.radius
    }
}
```
==> You can implement similar traits for enums, and other types ( even primitive types with rules ).

### 4. Trait Objects and Dynamic Dispatch:
- Dynamic Dispatch with `dyn Trait`
- Object safety requirements.

Used when you want ** runtime Polymorphism **.

```rust 
trait Drawable {
    fn draw(&self);
}

struct Button;
impl Drawable for Button {
    fn draw(&self) {
        println!("Drawing a button");
    }
}

fn render(ui: &dyn Drawable) {
    ui.draw();
}
```
- `&dyn Trait` is a *trait object*
- Enables *dynamic dispatch* ( like vtables in C++ )
- Can only be used with *object-safe* traits.

=> Object Safety: 
Trait must be *object-safe* to use with `dyn`:
1. No generic methods 
2. Method must use `&self`, `&mut self` or `self`

Allowed: 

```rust 
trait Good {
    fn do_it(&self);
}
```
❌ Not allowed:

```rust
trait Bad {
    fn new<T>() -> T;
}
```


```rust 
// Trait object for hetrogeneous collections:
let animals: Vec<&dyn Speak> = vec![&dog, &cat];
for animal in animals {
    println!("{}", animal.speak());
}
```




### 5. Supertraits:

A **trait that depends on another trait**

```rust 
trait Write {
    fn write(&self);
}

trait Log: Write {
    fn log(&self) {
        self.write(); // can call write because it's a supertrait.
    }
}
```

### 6. Auto Traits:

Automatically Implemented traits by the compiler like:
- `Send`, `Sync`, `Unpin` ...

Note: Custom auto traits are unstable as of now.

### 7. Marker Traits:
- Traits with no methods.
- `Copy`, `Sized`, `Send`, `Sync`

### 8. Derivable Traits:

Rust provides built-in **derive Macro** for common traits:

- `#[derive]` attribute
- Common traits: `Debug`, `Clone`, `Copy`, `PartialEq`, etc.

```rust 
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}
```

```rust 
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct User {
    name: String,
    age: i32,
}
```

**Common derivable traits**: 

* `Debug`
* `Clone`, `Copy`
* `PartialEq`, `Eq`
* `Ord`, `PartialOrd`
* `Hash`
* `Default`

And there are  **Common Standard Library Traits**

| Trait               | Purpose                  |
| ------------------- | ------------------------ |
| `Debug`             | For printing with `{:?}` |
| `Clone` / `Copy`    | Duplicate values         |
| `Default`           | Create default values    |
| `PartialEq`, `Eq`   | Equality                 |
| `PartialOrd`, `Ord` | Ordering                 |
| `Iterator`          | Iteration                |
| `Into`, `From`      | Conversions              |
| `AsRef`, `Borrow`   | References and borrowing |
| `Deref`, `Drop`     | Smart pointers & cleanup |


### 9. Associated Traits:

Instead of generic parameters, traits can use **associated types**:

```rust
trait Iterator {
    type Item;

    fn next(&mut self) -> Option<Self::Item>;
}
```

Usage:

```rust
struct Counter;

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        Some(42)
    }
}
```

### 10. Trait Objects Vs Generics 
- Static vs dynamic dispatch 
- Performance trade-offs


### 11. Advanced Trait Features: 
- **Higher-Rank Trait Bounds (HRTB)**
- **Trait Objects with multiple traits**
- **Trait aliases**  (nightly)
- **Specialization** (nightly)

Allows more specific trait impls to override more general ones.

```rust
default fn do_thing(&self) {
    println!("Default");
}
```

HRTB: Used when a trait must work for **all lifetimes**.

```rust
fn do_something<F>(f: F)
where
    F: for<'a> Fn(&'a str),
{
    f("hello");
}
```


Currently **unstable**, only available with nightly Rust.

### 12. Common Standard Library Traits:
- `From`/`Into` : for conversions 
- `Deref`/`DerefMut` for smart pointers 
- `Drop` for destructors 
- `Iterator` for collections 
- `Display`/`Debug` for formatting

### 13. Trait Coherence and Orphan Rules
- Where traits can be implemented
- Avoiding conflicting implementations

What is Orphan Rule?

Rust doesn’t allow you to implement **foreign traits on foreign types**.

```rust
// Not allowed:
impl Display for Vec<u8> {} // Both Display and Vec are foreign
```

Only allowed if:

* You own the trait
* Or you own the type

### 14. Trait Objects and Object Safety
- Requirements for trait objects
- `Sized` considerations


### 15. Blanket Implementations

Useful for applying traits to **all types meeting a condition**:

```rust
trait Printable {
    fn print(&self);
}

impl<T: Debug> Printable for T {
    fn print(&self) {
        println!("{:?}", self);
    }
}
```

This makes **all `T: Debug` types** also implement `Printable`.


### 16. Implementing Traits for External Types:

Wrap the external type in a **newtype**"

```rust 
struct MyVec(Vec<u8>); // this is tuple struct
impl Printable for MyVec {
    fn print(&self) {
        println!("{:?}",self.0 );
    }
}
```
full example:

```rust 

struct Manu {
    name: String,
}

// since rust prevents to define global string directly
struct Msg(String); // we put the string inside a tuple struct

trait Hariom {
    fn speak(&self);
}

impl Hariom for Manu {
    fn speak(&self) {
        println!("{}Hari Om Tat Sat", self.name);
    }
}
impl Hariom for Msg {
    fn speak(&self) {
        println!("Encoded message : {}", self.0); // self.0 is to reach the elements inside tuple
                                                  // struct
    }
}
fn main() {
    let manu = Manu {
        name: "manush".to_string(),
    };
    manu.speak();
    let x = Msg("Hello from Rust".to_string());
    x.speak();
}
```

### 17. Test Traits (Mocking, etc.)

You can use traits to write **mockable** and **testable** code by abstracting behavior.

```rust
trait Database {
    fn query(&self, sql: &str) -> String;
}
```

Then in tests:

```rust
struct MockDb;

impl Database for MockDb {
    fn query(&self, _: &str) -> String {
        "mock result".into()
    }
}
```

Summary Table

| Concept           | Description                                         |
| ----------------- | --------------------------------------------------- |
| Traits            | Define shared behavior                              |
| `impl Trait`      | Abstract parameters and return types                |
| Trait Bounds      | Restrict generic types                              |
| Trait Objects     | Runtime polymorphism                                |
| Default Methods   | Shared default implementations                      |
| Associated Types  | Define internal type placeholders                   |
| Supertraits       | Require one trait to implement another              |
| Blanket Impls     | Apply trait to all types matching condition         |
| Orphan Rule       | Prevents implementing foreign trait on foreign type |
| Auto Traits       | Built-in marker traits                              |
| Procedural Macros | Auto-implement traits with custom macros            |
...

## Complete Example: 

```rust 
use std::fmt::Display;

//Basic trait with default implementation 
trait Animal: Display { 
    fn name(&self) -> &str;

    fn make_sound(&self) -> String {
        "Some generic animal sound!!".to_string()
    }

    // Associated function 
    ////fn animal_type() -> String {
    ////    "Animal".to_string()
    ////}
    /* Comment this error as the above trait is not object safe 
        Object Safe requiers: 
        - All methods called on the trait object must have receiver (self, &self, or &mut self)
        - No generic methods 
        - No methods that return `Self` ( the concrete implementor type).
        - No associated functions (static methods) that you might try to call on the trait object.
       
    */
}

//Supertrait
trait Pet: Animal {
    fn owner(&self) -> &str;
}

struct Dog {
    name: String,
    owner: String,
}

impl Display for Dog {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Dog: {} (owned by {})", self.name, self.owner)
    }
}

impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name 
    }

    // fn make_sound(&self) -> String {
    //    "Woof!".to_string()
    //}
}

impl Pet for Dog {
    fn owner(&self) -> &str {
        &self.owner 
    }
}

//Generic Function with trait bounds:
fn introduce<T: Pet>(pet:&T) {
    println!("This is {}, owned by {}", pet.name(), pet.owner());
    println!("It says: {}", pet.make_sound());
}

// Trait objects for dynamic dispatch
fn process_animals(animals: &[&dyn Animal]) {
    for animal in animals {
        println!("{} - {}", animal, animal.make_sound());
    }
}

fn main() {
    let dog = Dog {
        name: "Buddy".to_string(),
        owner: "Alice".to_string(),
    };

    introduce(&dog);

    let animals: Vec<&dyn Animal> = vec![&dog];

    let animals: Vec<&dyn Animal> = vec![&dog];
    process_animals(&animals);
}
```


## Key Concepts to Remember

1. **Trait Bounds**: Compile-time polymorphism with monomorphization
2. **Trait Objects**: Runtime polymorphism with dynamic dispatch
3. **Object Safety**: Traits must not return `Self` or use generic methods to be object-safe
4. **Coherence Rules**: You can only implement traits for types you own
5. **Zero-Cost Abstractions**: Traits provide abstraction without runtime overhead

Traits are fundamental to Rust's type system and enable much of the language's safety, performance, and expressiveness.

## References:
---------------------------------------------------------------------------------- 
1. Interfaces: ( CPP )
Interfaces in C++
=================

In C++, an interface is a class that contains only pure virtual functions and no data members. 
It is used to define a contract that must be implemented by any class that inherits from it. 
Interfaces are useful for defining a common set of methods that must be implemented by a group of related 
classes.

**Example: Shape Interface**
---------------------------

    ```cpp
        // shape.h
        #ifndef SHAPE_H
        #define SHAPE_H

        class Shape {
        public:
            // Pure virtual function to calculate area
            virtual double area() = 0;

            // Pure virtual function to calculate perimeter
            virtual double perimeter() = 0;

            // Virtual destructor to ensure proper cleanup
            virtual ~Shape() {}
        };

        #endif  // SHAPE_H
    ```

In this example, the `Shape` class is an interface that defines two pure virtual functions: `area()` 
and `perimeter()`. 

These functions must be implemented by any class that inherits from `Shape`.

**Implementing the Interface: Circle and Rectangle**
---------------------------------------------------

    ```cpp
        // circle.h
        #ifndef CIRCLE_H
        #define CIRCLE_H

        #include "shape.h"

        class Circle : public Shape {
        private:
            double radius_;

        public:
            Circle(double radius) : radius_(radius) {}

            // Implement the area() function
            double area() override {
                return 3.14159 * radius_ * radius_;
            }

            // Implement the perimeter() function
            double perimeter() override {
                return 2 * 3.14159 * radius_;
            }
        };

        #endif  // CIRCLE_H
    ```

    ```cpp
        // rectangle.h
        #ifndef RECTANGLE_H
        #define RECTANGLE_H

        #include "shape.h"

        class Rectangle : public Shape {
        private:
            double width_;
            double height_;

        public:
            Rectangle(double width, double height) : width_(width), height_(height) {}

            // Implement the area() function
            double area() override {
                return width_ * height_;
            }

            // Implement the perimeter() function
            double perimeter() override {
                return 2 * (width_ + height_);
            }
        };

        #endif  // RECTANGLE_H
    ```

The `Circle` and `Rectangle` classes inherit from the `Shape` interface and implement the `area()` and 
`perimeter()` functions.

**Using the Interface**
-----------------------

    ```cpp
        // main.cpp
        #include "circle.h"
        #include "rectangle.h"

        int main() {
            Circle circle(5.0);
            Rectangle rectangle(3.0, 4.0);

            Shape* shapes[] = {&circle, &rectangle};

            for (Shape* shape : shapes) {
                std::cout << "Area: " << shape->area() << std::endl;
                std::cout << "Perimeter: " << shape->perimeter() << std::endl;
                std::cout << std::endl;
            }

            return 0;
        }
    ```

we create an array of `Shape` pointers and store the addresses of `Circle` and `Rectangle` objects. 
iterate over the array and call the `area()` and `perimeter()` functions on each object, without knowing 
the actual type of the object.

This demonstrates the power of interfaces in C++, which allow us to write generic code that can work with  
a variety of classes that implement a common interface.
---------------------------------------------------------------------------------- 
