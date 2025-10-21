// Returning Types that Implement Traits

// Step 1: Define the Trait and Concrete Types
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

// Step 2: Implement the Factory Function
// we create a func, `make_shape`, that returns a type that implements `Shape`.
// Note that it uses `-> impl Shape`, not `-> Circle` or `-> Square`.
// This function returns *some type* that implements the Shape trait.
// fn make_shape(is_round: bool, value: f64) -> impl Shape {
//     if is_round {
//         // Here we return a concrete Circle struct
//         Circle { radius: value }
//     } else {
//         // Here we return a concrete Square struct
//         Square { side: value }
//     }
//     // IMPORTANT: The function must return only ONE concrete type, regardless
//     // of the code path. This example will NOT compile (see explanation below).
//     // Reason: When you use -> impl Trait, the compiler needs to determine one single concrete type for
//     // that function's return signature.
//     // In this example the compiler sees that the function might return a Circle or a Square.
//     // Since Circle and Square are different types, even though they both implement Shape, the compiler
//     // gets confused.
//     //
//     // The Solution is to use : Boxed Trait Objects (Box<dyn Trait>)
//     // To return different types that share a trait from a function, you must use
//     // Dynamic Dispatch with a Trait Object.
//     // This involves putting the structs in a box (Box) and using the dyn keyword:
//     // as below: make_shape_dyn() -> Box<dyn Shape> {}
// }
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

// Step 3: Call the Dynamic Function
// calling code doesn't need to know if it's dealing with a Circle or a Square;
// it only knows it has a trait object that implements Shape.
fn main() {
    println!("--- Dynamic Shape Creator ---");

    // Get a shape (a Circle, but we don't know it)
    let shape_one = make_shape_dyn(true, 5.0);

    // Get a second shape (a Square, but we don't know it)
    let shape_two = make_shape_dyn(false, 10.0);

    println!(
        "Shape 1: {} (Area: {:.2})",
        shape_one.description(),
        shape_one.area()
    );
    println!(
        "Shape 2: {} (Area: {:.2})",
        shape_two.description(),
        shape_two.area()
    );
}
