### **What is Dynamic Dispatch?**


- In Rust, **dynamic dispatch** is a mechanism used to determine which method to call at runtime, as 
  opposed to **static dispatch**, where the method is determined at compile-time. 

- Dynamic dispatch is typically associated with polymorphism, which is a feature of object-oriented 
  programming, and is used when you want to call methods on types that are unknown at compile-time but known
  at runtime.

- Dynamic dispatch occurs when the exact type of an object isn't known until the program is running, and 
  the correct method or function is selected based on the actual type of the object. 
  This decision-making process is often done using **trait objects** in Rust.

#### **How Dynamic Dispatch Works in Rust**

Rust achieves dynamic dispatch by using **trait objects**. A trait object is created by using a reference 
or pointer to a type that implements a trait. 
The actual type of the object that implements the trait isn't known at compile-time but is determined at 
runtime.

When you use a **trait object** (e.g., `&dyn Trait` or `Box<dyn Trait>`), Rust uses dynamic dispatch to 
look up the appropriate method based on the type of the object at runtime.

### **Example with Dynamic Dispatch**

Let’s consider an example using traits and dynamic dispatch.

```rust
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a Circle with radius: {}", self.radius);
    }
}

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing a Square with side length: {}", self.side);
    }
}

fn draw_shape(shape: &dyn Draw) {
    shape.draw();
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 10.0 };

    draw_shape(&circle);  // Dynamic dispatch
    draw_shape(&square);  // Dynamic dispatch
}
```

In this example, the `draw_shape` function accepts a **trait object** (`&dyn Draw`). 
At runtime, Rust uses dynamic dispatch to determine which `draw` method to call based on the actual 
type of the `shape` (either `Circle` or `Square`).

### **Key Concepts Involved in Dynamic Dispatch**

1. **Trait Objects**: A trait object is a reference (`&dyn Trait`) or smart pointer (`Box<dyn Trait>`) 
   that points to an object implementing a trait. Rust uses this object to determine the method to call at 
   runtime.

2. **Vtable (Virtual Table)**: When you create a trait object, Rust creates a **vtable** (virtual method 
   table) that holds pointers to the methods that are available for that trait. 
   When dynamic dispatch occurs, Rust looks up the method in the vtable and calls the correct method for the
   specific object type.

3. **Performance Overhead**: Since dynamic dispatch happens at runtime, it introduces a small performance 
   overhead compared to static dispatch (where the method is determined at compile time). 
   The overhead comes from looking up the method in the vtable instead of directly invoking a function.

### **Static Dispatch vs. Dynamic Dispatch**

- **Static Dispatch**: 
  Rust can determine the method to call at compile time when the type of the object is known. 
  This is the default behavior when you use generic types or when you explicitly specify the type of the 
  object. The compiler can optimize away the method call by generating code that directly calls the 
  appropriate function.

- **Dynamic Dispatch**: Occurs when the exact type of the object is not known at compile time, and a 
  trait object is used. The method is looked up at runtime using a vtable.

### **Example of Static Dispatch**

This example illustrates static dispatch using generics:

```rust
trait Draw {
    fn draw(&self);
}

struct Circle {
    radius: f64,
}

struct Square {
    side: f64,
}

impl Draw for Circle {
    fn draw(&self) {
        println!("Drawing a Circle with radius: {}", self.radius);
    }
}

impl Draw for Square {
    fn draw(&self) {
        println!("Drawing a Square with side length: {}", self.side);
    }
}

fn draw_shape<T: Draw>(shape: T) {
    shape.draw();
}

fn main() {
    let circle = Circle { radius: 5.0 };
    let square = Square { side: 10.0 };

    draw_shape(circle);  // Static dispatch
    draw_shape(square);  // Static dispatch
}
```

Here, `draw_shape` is a **generic function** that is determined at compile time based on the type passed in.
The Rust compiler can generate specific code for each type (`Circle` or `Square`) without the need for 
dynamic dispatch.

### **When to Use Dynamic Dispatch**

Dynamic dispatch is useful when:
- You don't know the exact type of the object at compile time.
- You need polymorphism (i.e., the ability to treat different types in a uniform way).
- You want to write functions or data structures that work with many types that implement a common trait.

### **When to Avoid Dynamic Dispatch**

If performance is critical and you know the types at compile-time, **static dispatch** is typically more 
efficient because it avoids the overhead of the vtable lookup.

### **Summary**

- **Dynamic dispatch** occurs when Rust uses trait objects (e.g., `&dyn Trait` or `Box<dyn Trait>`) to look 
  up and call methods at runtime, based on the actual type of the object.
- It allows for polymorphism, enabling you to treat different types in a unified way.
- While it provides flexibility, it comes with a slight performance overhead compared to static dispatch, 
  where the method is determined at compile time.

Understanding dynamic dispatch is crucial when working with polymorphic code in Rust, especially in the 
context of systems programming, where you may need to handle various types of objects in a flexible and 
efficient manner.


