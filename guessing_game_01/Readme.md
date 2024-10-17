# Guessing Game: 

- Introduction to Crates, traits ...
- Covers Rust programming syntax and basic programming constructs.

- Program generate a random integer in range 1-100.
- Read user input ( guessing number ).
- Compare the user input number and Upon entering our guess.
- Print High Low or congratulate on correct guess.


-- 

## Using Crates and functionality: 

### Crates: 

- In Rust there is no standard libc as with C programs. Instead Rust comes with tons of publically hosted
  Crates that can be defined in the Cargo.toml file to be used with the program. 

- Crates are packages that contain libraries or executables. 

- When we add a crate to Cargo.toml file, we can use the functionality of that crate in the program.

ex: In this guessing Game we need to generate a random number in the range of 0 to 100. For this we can
import the "rand crate". (https://crates.io/crates/rand) the related document on how to use the crate can be
accessed at (https://rust-random.github.io/book/)

- Cargo.toml file: ( nvim : use crates.nvim For automatic version check  from within Cargo.toml file.

[dependencies]
rand = "0.8.5"

- To use the Crate from rust program:
(~/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/share/doc/rust/html/std/keyword.use.html)

main.rs: ( import the crate to the rust program using the "use" keyword )

    `use rand`

keyword "use": To import or rename items from other crates or modules.

use keyword can appear in Modules, blocks, and even functions.

Generally defined at the top section of the rust source file (similar to header files in C,C++ programs)

---
## Crates:

- A crate in Rust is a package that contains a collection of related code. 
- A crate can contain various components, including modules, traits, functions, and types. 
    A typical structure of a crate:
    1. Modules:
        Modules are used to organize code within a crate. 
        They are essentially namespaces that contain a set of definitions, such as functions, types, and
        traits.
        Modules can be nested, allowing for a hierarchical organization of code.
    2. Traits: 
        Traits define a contract that a type must fulfill. 
        They are used to specify a set of methods that a type must implement. 
        Traits are often used to define interfaces or abstract base classes.
    3. Functions:
        Functions are blocks of code that perform a specific task. 
        They can be defined within modules or at the crate level.
    4.  Types: 
        Types define the structure and behavior of data. 
        They can be defined within modules or at the crate level.

- Crate Hierarchy:
    Crate can have a hierarchical structure, with modules nested within other modules. 
    This allows for a logical organization of code and makes it easier to manage large projects.

example: Crate Hierarchy:
    ```
    // Crate
    crate::my_crate {
        // Module
        mod my_module {
            // Submodule 
            mod my_submodule {
                // Function
                fn my_function() {
                    println!("Hello, world!");
                }
            }
            // Trait
            trait MyTrait {
                fn my_method(&self);
            }
            // Type
            struct MyType {
                x: i32,
                y: i32,
            }
        }
        // Function
        fn my_crate_function() {
            println!("Hello, crate!");
        }
    }

    ```
    my_crate crate -> 
        contains a module called my_module, which in turn contains ->  
            - a submodule called my_submodule. 
            - a trait called MyTrait and 
            - a type called MyType. 
        The crate itself defines a function called my_crate_function.

- Visibility:
    visibility is used to control access to definitions within a crate. 
    There are three levels of visibility:

    - Public: Public definitions are visible from outside the crate and can be used by other crates.
    
    - Private: Private definitions are only visible within the current module and cannot be accessed from
      ourside the module.
    
    - Internal: Internal definitions are visible within the current crate but not from outside the crate.
    
    By default, definitions within a crate are private. 
    To make a definition public, you can use the "pub" keyword.

    Example:
    ```
    // Crate
    crate::my_crate {
        // Public module
        pub mod my_module {
            // Public function
            pub fn my_function() {
                println!("Hello, world!");
            }
        }
        // Private function
        fn my_private_function() {
            println!("Hello, private!");
        }
    }
    
    my_module module and the my_function function are public, 
    while the my_private_function function is private.

#### Using crates functionality:

- After adding to import the crates functionality using "use" keyword: we can use the functionality of that
  crate: example we can use "thread_rng" functionality to generate a random number:

  ```
    let mut rng = rand::thread_rng();
    let random_num: u32 = rng.gen();
    println!("The random number is : {random_num}");
  ```

- Crates can contain multiple modules, we can import a specific module using the same 'use' keyword as
  below: ( rand crate contains a distribution module that provides various probability distributions) :

  ```
  use rand::distributions::{Distribution, Standard};
  fn main() {
    let mut rng = rand::thread_rng();
    let standard: Standard = Standard;
    let random_number: f64 = standard.sample(&mut rng);
    println!("Random number: {}", random_number);
  }
  ```

#### Traits vs. Modules in rust 

- Rust, traits and modules are two distinct concepts that serve different purposes.

    - Traits:
        Traits are a way to define a set of methods that a type can implement. 
        Similar to interfaces in other languages. 
        Traits define a contract that a type must fulfill, but they do not provide any implementation.
        Traits are used to define a common set of methods that can be used by multiple types.

    - Modules:
        Modules, are a way to organize code in Rust. 
        They are used to group related functions, types, and traits together. 
        Modules can be thought of as namespaces that contain a set of definitions. 
        Modules can be used to define a scope for a set of definitions, making it easier to manage and reuse
        code.

- Key differences:

    1. Purpose: Traits define a contract that a type must fulfill, while modules are used to organize code.
    2. Implementation: Traits do not provide any implementation, while modules can contain implementations
       of functions and methods.
    3. Scope: Traits define a scope for a set of methods, while modules define a scope for a set of
       definitions.

    Example: To illustrate the difference, 

    ```rust
        // Define a trait
        trait Printable {
            fn print(&self);
        }

        // Define a module
        mod printer {
            // Define a struct
            pub struct Printer;

            // Implement the Printable trait for the Printer struct
            impl Printable for Printer {
                fn print(&self) {
                    println!("Printing...");
                }
            }

            // Define a function 
            pub fn print_message(message: &str) { 
                println!("{}", message);
            }
        }
    ```

    `Printable` trait defines a contract that a type must fulfill, 
    while the `printer` module defines a scope for a set of definitions, 
    including the `Printer` struct and the `print_message` function.
---

- Use the rand crate:

    use rand::rng;
    fn main() {
        // random_num in range 0 to 100
        let rand_num: u32 = rand::thread_rng().gen_range(1..101);
    }

- read keyboard input:

    `let mut guess = String::new();` // create a empty mutable new string.
    // Use the std crate's io module
    `std::io::stdin().read_line(&mut guess).expect("Failed to read line");`

    - std::io::stdin() : returns a handle to the standard input stream (keyboard).
    - read_line(&mut guess): This method reads a line of input from the std input stream and stores it in
      the guess variable. 
      The &mut keyword means that guess is a mutable reference, which allows the read_line method to modify
      its contents with out touching ownership of guess.
    - expect("Failed to read line"): This method is called on the Result returned by read_line. 
        If the Result is Ok, it returns the value inside the Ok. 
        If the Result is Err, it panics with the message "Failed to read line".

    However, in a real-world apps, you would probably want to handle the error more robustly, rather than 
    just panicking with a message. 
    You might use a match statement to handle the Result returned by read_line, like this:

    ```
        use std::io;
        fn main() {
            loop {
                let mut guess = String::new();
                println!("Please enter your guess:");

                match io::stdin().read_line(&mut guess) {
                    Ok(_) => println!("You guessed: {}", guess),
                    Err(e) => println!("Error reading input: {}", e),
                }
            }
        }
    ```

- comparing  numbers and break the loop if numbers match:

    use std::cmp::Ordering;
    loop {
        ...
        match guess.cmp(&rand_num) {
            Ordering::Less => println!("smaller"),
            Ordering::Greater => println!("Larger"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }

    - loop { ... }: This is an infinite loop that will keep running until it encounters a break statement.
    - match guess.cmp(&rand_num) { ... }: 
        This is a match statement that compares the user's guess (guess) to a random number (rand_num). 
        cmp method returns Ordering type.
    - Ordering type can take the following values:
        - Ordering::Less = guess is less than rand_num.
        - Ordering::Greater = guess is greater than rand_num 
        - Ordering::Equal = the numbers match.
    Since we are in a infinit loop we break the program loop when the numbers match.


