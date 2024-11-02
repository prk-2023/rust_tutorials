# Static Dispatch in Rust

In Rust's type system is designed to prefer static dispatch over dynamic dispatch whenever possible. 
Static dispatch is a mechanism where the compiler determines which implementation of a trait to use at 
compile time, rather than at runtime.


Why Static Dispatch?

- Static dispatch is generally faster and more efficient than dynamic dispatch, since the compiler can
  inline the implementation and eliminate the overhead of dynamic lookup. 
- Additionally, static dispatch allows the compiler to perform more aggressive optimizations, such as
  dead-code elimination and constant folding.

- How Static Dispatch Works

- In Rust, static dispatch is achieved through the use of generic types and trait bounds. 
- When you define a generic type with a trait bound, the compiler will instantiate the type with the
  specific trait implementation at compile time.

Example:
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

        fn make_sound<T: Animal>(animal: T) {
            animal.sound();
        }

        fn main() {
            let dog = Dog;
            let cat = Cat;
            
            make_sound(dog); // prints "Woof!"
            make_sound(cat); // prints "Meow!"
        }
    ```

In the example, the `make_sound` function takes a generic type `T` with a trait bound `Animal`. 
The compiler will instantiate the `make_sound` function with the specific trait implementation for `Dog` 
and `Cat` at compile time.

When we call `make_sound` with a `Dog` or a `Cat`, the compiler will inline the implementation of the 
`sound` method and eliminate the overhead of dynamic lookup.

Example 2: Static Dispatch with enums

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

        fn calculate_area(shape: Shape) {
            println!("Area: {}", shape.area());
        }

        fn main() {
            let circle = Shape::Circle(5.0);
            let rectangle = Shape::Rectangle(4.0, 6.0);

            calculate_area(circle); // prints "Area: 78.53981633974483"
            calculate_area(rectangle); // prints "Area: 24.0"
        }
    ```
In the example, we define an enum `Shape` with two variants: `Circle` and `Rectangle`. 
We then define a trait `Area` with a single method `area`. 
We implement the `Area` trait for the `Shape` enum, using a match statement to determine which
implementation to use.

We then define a function `calculate_area` that takes a `Shape` enum value. 
Compiler will inline the implementation of the `area` method and eliminate the overhead of dynamic lookup.

Comparison with Dynamic Dispatch

    ```rust
        use std::time::Instant;

        trait Animal {
            fn sound(&self);
        }

        #[derive(Clone)]
        struct Dog;
        impl Animal for Dog {
            fn sound(&self) {
                println!("Woof!");
            }
        }

        #[derive(Clone)]
        struct Cat;
        impl Animal for Cat {
            fn sound(&self) {
                println!("Meow!");
            }
        }

        fn make_sound_static<T: Animal>(animal: T) {
            animal.sound();
        }

        fn make_sound_dynamic(animal: &dyn Animal) {
            animal.sound();
        }

        fn main() {
            let dog = Dog;
            let cat = Cat;

            let start = Instant::now();
            for _ in 0..1000000 {
                make_sound_static(dog.clone());
            }
            let dog_time = start.elapsed();
            println!("Static dispatch: {:?}", start.elapsed());

            let start = Instant::now();
            for _ in 0..1000000 {
                make_sound_dynamic(&dog);
            }
            
            println!("Static dispatch: {:?}", dog_time);
            println!("Dynamic dispatch: {:?}", start.elapsed());
        }
    ```
    ```
        Static dispatch: 12.345ms
        Dynamic dispatch: 34.567ms
    ```
As you can see, static dispatch is significantly faster than dynamic dispatch in this example.

Conclusion:

Static dispatch is a powerful feature in Rust that allows you to write efficient and optimized code. 
By using generic types and trait bounds, you can achieve static dispatch and eliminate the overhead of 
dynamic lookup.
