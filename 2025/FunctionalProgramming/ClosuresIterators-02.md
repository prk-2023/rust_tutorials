# Closures:

A function like construct that can store in a variable. Mastering Closures helps in writing Idiomatic rust
code faster.


Closure: In FP **Closures** are anonymous functions ( like lambda functions in python ) that can capture the
surrounding environment. 


You can create the closure in one place and then call the closure elsewhere to evaluate it in a different
context. Unlike functions closure can capture values from the scope in which they're defined. 
They also allow for code reuse and behavior customization.

Closure are highly flexible and can capture variables from their surrounding environment, either by reference
of by value, depending on how they are used.


## Syntax:

```rust 
    let closure = |parameter| {
        //body of the closure
    };
```

- **The `|parameter|` syntax is used to define the closure parameter. 

- **The body of the closure** can contain any code just like a regular function.

### **Types of Closures in Rust:**

Rust has three types of closures based on how they capture variables from their environment:

1. **By reference (`&T`)**: The closure borrows a reference to the variable.
2. **By mutable reference (`&mut T`)**: The closure borrows a mutable reference to the variable.
3. **By value (`T`)**: The closure takes ownership of the variable.

- Below we can see that rust closures borrow variables from its surroundings.
- They also infer the type of parameters and return values (though rust is statically-typed language) it
  still has stricter type checking( if required you can also annotate types).
- Rust Closure can borrow variables from environment or take ownership, depending on how they are defined.
  This behavior is governed by Rust strict ownership and borrowing rules. Ex: Closure can borrow a value
  immutable or it can take ownership of the value.

  ```rust 
    let s = String::from("hello rust");
    let take_ownership = move || {println!{"",s}; //takes ownership of s};
    take_ownership();
    //println!{"{}",s};  <== this should cause compilation error because 's' has moved
  ```
  'move': is the keyword which takes ownership of all variables it takes from its surrounding scope, in the
  above example it variable 's'.

- Function Signature and Return types: Return types of closure can be inferred or can be specified
  explicitly. 

- **Closure are also types in themselves in rust (like `Fn`, `FnMut` or `FnOnce`) which represent
  different kinds of callable objects based on how they capture variables.
  These are un-nameable concrete types, and the traits listed (Fn, FnMut, FnOnce) are how Rust categorizes 
  these concrete closure types based on how they interact with the variables they capture.

- Closure Traits Explained: Fn, FnMut, and FnOnce are collectively known as the Function Traits. They define
  what a closure can do with the captured variables:

| Trait | Capture type | behavior | Analogy |
| :---  | :---         | :---     | :---    |
| Fn | Captures by immutable reference (&T).|Can be called multiple times. It only reads the captured data.| A librarian who only reads the book but doesn't write in it.|
| FnMut| Captures by mutable reference (&mut T)|Can be called multiple times and mutates the captured data.|A note-taker who can update the shared document.|
| FnOnce|Captures by value (takes ownership - T)|Can be called only once because it consumes the captured data|A cook who uses (consumes) the ingredients to make a single meal|

### **Example 1: Simple Closure**

This is a basic example of a closure that takes an argument and returns a value:

```rust
fn main() {
    let square = |x| x * x;  // A closure that squares the input
    let result = square(5);   // Calling the closure
    println!("The square of 5 is: {}", result);
}
```

**Explanation:**

* The closure `square` takes one argument `x` and returns `x * x`.
* It is called with the argument `5`, and the result `25` is printed.

### **Example 2: Capturing Variables from the Environment**

Rust allows closures to capture variables from the environment. 
Here's an example that demonstrates capturing variables by reference, mutable reference, and by value.

```rust 
    fn main() {
        // 1. By reference (&T) - immutable borrow
        let x = 10;
        let print_x = || println!("x = {}", x); // **borrows x immutably**
        print_x();
        println!("x is still accessible: {}", x); // x is still usable here as x is immutable 
        println!("As Closure only borrows immutably and does not take ownership");
        
        // 2. By mutable reference (&mut T) - mutable borrow
        let mut y = 20;
        let mut increment_y = || {
            y += 1; // borrows y mutably
            println!("y inside closure: {}", y);
        };
        increment_y();
        // println!("y after closure: {}", y); // This would cause error!
        // We can't use y here because mutable borrow is still active
        
        // Let's demonstrate mutable reference properly
        let mut z = 30;
        {
            let mut modify_z = || z += 5;
            modify_z();
        } // mutable borrow ends here
        println!("z after mutable closure: {}", z); // Now we can use z again
        
        // 3. By value (T) - takes ownership
        let name = String::from("Alice");
        let consume_name = move || { // `move` keyword transfers ownership
            println!("Hello, {}!", name);
            // name is moved into the closure
        };
        consume_name();
        // println!("name: {}", name); // This would cause error!
        // name is no longer accessible here because ownership was transferred
        
        // Demonstrating move with primitive types (Copy trait)
        let age = 25;
        let take_age = move || {
            println!("Age: {}", age);
        };
        take_age();
        println!("age is still accessible: {}", age); // Works because i32 is Copy
        
        // Complex example showing the differences
        let vec1 = vec![1, 2, 3];
        let vec2 = vec![4, 5, 6];
        let mut vec3 = vec![7, 8, 9];
        
        // By reference
        let sum_vec1 = || {
            let total: i32 = vec1.iter().sum();
            println!("Sum of vec1: {}", total);
        };
        sum_vec1();
        println!("vec1 is still available: {:?}", vec1);
        
        // By mutable reference
        let mut clear_vec3 = || {
            vec3.clear();
            vec3.push(10);
            println!("Modified vec3: {:?}", vec3);
        };
        clear_vec3();
        // vec3 is now modified and we can use it again
        println!("vec3 after modification: {:?}", vec3);
        
        // By value
        let consume_vec2 = move || {
            println!("Consuming vec2: {:?}", vec2);
            // vec2 is moved here
        };
        consume_vec2();
        // println!("vec2: {:?}", vec2); // Error - vec2 was moved
    }
```

Additional function to demonstrate closure types
```rust 
    // Additional function to demonstrate closure types
    fn demonstrate_closure_types() {
        let counter = 0;
        
        // Fn closure - borrows immutably
        let read_counter = || println!("Counter: {}", counter);
        
        let mut mutable_data = 100;
        
        // FnMut closure - borrows mutably
        let mut update_data = || {
            mutable_data += 1;
            println!("Updated data: {}", mutable_data);
        };
        
        let owned_data = String::from("test");
        
        // FnOnce closure - takes ownership
        let consume_data = move || {
            println!("Consuming: {}", owned_data);
            // owned_data is dropped here
        };
        
        read_counter();
        update_data();
        consume_data();
        // consume_data(); // This would error - closure can only be called once
    }

    // Running the additional demonstration
    fn main() {
        // ... previous main content ...
        
        println!("\n--- Additional Demonstration ---");
        demonstrate_closure_types();
    }

```
This example demonstrates:

**1. By reference (`&T`)**:
- The closure borrows the variable immutably
- The original variable remains accessible after the closure
Note: the type `i32` implements `Copy`, When a type implements `Copy` instead of being **moved** when passed
to a function or when a closure captures it, a **bit-for-bit copy** of the value is made. 
NOTE: If `x` was of type that did not implement `Copy` like `String` and the closure needed an immutable
borrow (&T), The original `x` would still be **accessible after the closure call** as closures did not take
ownership. The differences is if the closure needed a *mutable borrow* or *took ownership* of a non-copy
type, then `x` would be inaccessible afterward.

So `Copy` trait ensures that even if a closure moved the value the original variable ( now a copy ) would
remain valid.
In short: The code works because the closure borrows x (not taking ownership), and since i32 is a Copy type,
its low-level behavior is efficient and safe for multiple accesses.

- Uses `Fn` trait

**2. By mutable reference (`&mut T`)**:
- The closure borrows the variable mutably  
- The original variable cannot be used while the closure exists ( i.e borrowing is active )
- Uses `FnMut` trait
- Requires the closure to be declared as `mut`. 
  The variable holding the closure needs to be declared as `mut` if you intend to call the closure multiple 
  times (i.e., if it implements `FnMut`). However, the closure itself doesn't need a `mut` keyword in its 
  definition.
  ```rust 
      let mut num = 5;

      // The closure mutably borrows `num`. It implements FnMut.
      let mut increment = || { // The closure definition itself has no `mut` keyword
          num += 1;
      };

      // The variable holding the closure (`increment`) MUST be `mut`
      // because calling it changes its internal captured state (`num`).
      increment(); 
      increment(); 

      println!("{}", num); // Output: 7
  ```

**3. By value (`T`)**:
- The closure takes ownership of the variable using the `move` keyword
- The original variable is no longer accessible after being moved
- Uses `FnOnce` trait
- For types that implement `Copy`, the original remains accessible

Key points:
- Primitive types that implement `Copy` behave differently with `move`
- The closure's trait (`Fn`, `FnMut`, `FnOnce`) depends on how it captures variables
- Mutable borrows require the closure variable itself to be `mut`
- `move` keyword forces ownership transfer regardless of whether it's necessary

