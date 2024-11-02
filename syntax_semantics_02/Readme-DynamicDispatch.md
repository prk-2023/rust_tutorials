# Rust Dynamic Dispatch

- Dynamic Dispatch:
  It's a mechanism that allows the program to determine which implementation of a trait to use at runtime, 
  rather than at compile time. 
  This is in contrast to static dispatch, where the compiler determines which implementation to use.

- Why Dynamic Dispatch?
  Dynamic dispatch is useful when you need to write code that can work with different types of data, without
  knowing the exact type at compile time. This is often the case when working with trait objects, which are
  values that implement a trait but whose concrete type is unknown.

- How Dynamic Dispatch Works:   
  In Rust, dynamic dispatch is achieved through the use of trait objects. 
  A trait object is a value that implements a trait, but whose concrete type is unknown. 
  When you call a method on a trait object, the program looks up the implementation of that method at 
  runtime, rather than at compile time.

Ex example:
    ```rust
        trait Animal {
            fn sound(&self);
        }

        struct Dog;
        impl Animal for Dog {
            fn sound(&self) {
                println!("Woof!");
            }
        }

        struct Cat;
        impl Animal for Cat {
            fn sound(&self) {
                println!("Meow!");
            }
        }

        fn make_sound(animal: &dyn Animal) {
            animal.sound();
        }

        fn main() {
            let dog = Dog;
            let cat = Cat;

            make_sound(&dog); // prints "Woof!"
            make_sound(&cat); // prints "Meow!"
        }
    ```

In the above example, the `make_sound` function takes a trait object `animal` of type `&dyn Animal`. 
The `dyn` keyword indicates that the type is a trait object, and the `Animal` trait is the trait that it 
implements.

When we call `make_sound` with a `Dog` or a `Cat`, the program looks up the implementation of the `sound` 
method at runtime, and calls the correct implementation.

Example 2: Dynamic Dispatch with Enums

    ```rust 
        enum Shape {
            Circle(f64),
            Rectangle(f64, f64),
        }

        trait Area {
            fn area(&self) -> f64;
        }

        impl Area for Shape {
            fn area(&self) -> f64 {
                match self {
                    Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
                    Shape::Rectangle(width, height) => width * height,
                }
            }
        }

        fn calculate_area(shape: &dyn Area) {
            println!("Area: {}", shape.area());
        }
        fn main() {
            let circle = Shape::Circle(5.0);
            let rectangle = Shape::Rectangle(4.0, 6.0);
            calculate_area(&circle); // prints "Area: 78.53981633974483"
            calculate_area(&rectangle); // prints "Area: 24.0"
        }
    ```
In the example, we define an enum `Shape` with two variants: `Circle` and `Rectangle`. 
We then define a trait `Area` with a single method `area`. 
We implement the `Area` trait for the `Shape` enum, using a match statement to determine which 
implementation to use.

We then define a function `calculate_area` that takes a trait object `shape` of type `&dyn Area`. 
We can call this function with a `Shape` enum value, and the program will look up the implementation 
of the `area` method at runtime.


- Dynamic dispatch is a powerful feature in Rust that allows you to write flexible and generic code. 

- By using trait objects and dynamic dispatch, you can write code that can work with different types of
  data, without knowing the exact type at compile time.
