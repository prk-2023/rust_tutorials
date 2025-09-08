# Rust closures:

## Introduction:

Closures are a bit tricky but once the concept is clear they can be powerful tool.

In simple terms a `closure` in Rust is a *anonymous function* 
    - These functions can be assigned to a variable.
    - Or passed as an argument. 
Which is what all functions can be do, but `closure` are differences between regular and closure functions:
    - closure function can *capture* and *use* variables from its surrounding environment, a concept called
      *lexical scope*. 

## Characteristics of Rust closure:

1. Anonymous and stored in a variable:

Regular functions have names, but closure are nameless. 
We define them and often assign them to a variable:

```rust 
// A simple regular function 
fn add_one(x: i32) -> i32 {
    x + 1
}
// A equivalent closure 
let add_one_closure = |x: i32| -> i32 {
    x + 1 
};

//calling both :
let result1 = add_one(5);
let result2 = add_one_closure(5);

println!("Regular function {}", result1 );
println!("Closure function {}", result2 );
```
Notice the syntax: 

    |parameters| -> return_type { function_body }

The parameters are enclosed in pipes |...|, and the rest looks very similar to a function.

2. Type Inference ( The Magic part )
Rust compiler does type inference for you. 
We do not need to explicitly write out the type for the parameters and the return value:

```rust 
let add_closure = |x, y| x + y; // The compiler infers that x,y are the same type and returns same type. 
let sum = add_closure(10, 20);
println!("The sum is: {}", sum); // The sum is: 30
```
NOTE: This is common source of confusion: Once the compiler infers the type for the closure, they are locked 
in, and you can not use the function with different type. 
```rust 
let example = |x| x;
let int_result = example(5);     // compiler infers type as i32
// let string_result = example("hello"); // This will cause a compilation error!
```

3. Capturing the Environment ( the Superpower )

This is the most important feature of closures. 
They can use variables from the scope in which they are defined.

```rust 
let favorite_number = 42;

let is_my_favorite = |num| {
    num == favorite_number // `favorite_number` is captured from the outer scope
};

let result = is_my_favorite(42);
println!("Is 42 my favorite? {}", result); // Output: Is 42 my favorite? true
```

When closures capture a variable, it does so in one of the 3 ways corresponding to Rust's ownership system:
    `FnOnce`, `FnMut`, and `Fn`
1. `FnOnce` : This closures consumes the captured variables. The closure can only be called once because it 
              takes ownership of the data. This happens when the captured variable is moved into the closure 
              (ex: if you are passing a `String` by value )
2. `FnMut` : This closure can *mutably borrow* the captured variable. We can call this closure multiple times
             and it can modify the captured data. 
3. `Fn` : This closure can *immutable  borrow* the captured variable. You can call this closure multiple times,
          but it can not modify the captured data.

The Rust compiler automatically determines which of these traits your closure implements based on how you use the captured variables.

## When to use Closures:

Closures are extremely common and useful in Rust, especially with *iterators* and *high-order functions*. 

Example with iterators:
```rust 
//use closure to filter out odd numbers 
let numbers = vec![1, 2, 3, 4, 5];

// Use a closure to filter out odd numbers
let even_numbers: Vec<i32> = numbers
    .into_iter()
    .filter(|&x| x % 2 == 0) // The closure takes an immutable reference to each item
    .collect();

println!("Even numbers: {:?}", even_numbers); // Output: Even numbers: [2, 4]
```

Example with `thread::spawn`
When creating new thread, you often pass a closure to define the code the new thread should run. 
The `move` keyword is used here to force the closure to take ownership of the captured variable, as the new thread needs to own its data:
```rust 
use std::thread;

let my_data = String::from("Hello from the main thread!");

thread::spawn(move || {
    // The closure captures and moves 'my_data' into the new thread
    println!("New thread says: {}", my_data);
}).join().unwrap();
```
The move keyword is used to explicitly tell the compiler to move ownership of my_data into the closure, which is necessary because the new thr`ead might outlive the main function's scope.

### Summary for Beginners
- Closures are anonymous functions. They don't have a name and are often assigned to a variable.

- Syntax: |parameters| { body }. The pipes |...| are a key visual cue.

- They can capture variables from their environment. This is their most powerful feature.

- They are smart about types. The compiler often infers the types for you.

- They are widely used``````````. You will see them everywhere in Rust, especially with iterators, event handlers, and parallel programming.

Think of them as a lightweight, flexible function you can create on the fly that remembers and uses data from where it was created.
