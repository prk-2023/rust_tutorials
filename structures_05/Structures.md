# Structures: 

1. **Rust Structures**
=====================

A structure (or struct) is a way to group related data together. 
Structures are similar to classes in other languages, but they do not support inheritance.

- Like in C, CPP Rust supports custome structures.

- Like in CPP and unline in C, no `typedef` is needed to define a structure.

- Unlike in CPP structures in Rust do not support inheritance.

- Different types of structures are suppoted in Rust:

    - Zero-size structure: `struct Foo`, maybe useful while implementing traits on some type, but they hold
      no data you want to store in its values.
    - tuple structures: used when field names are not imporatant.
- special syntax: `..`
    struct Person {name: String, age: u8}
    let player1 = Person { name: String::from("spiderman"),age: 39 };
    let player2 = Person { name: String::from("batman"), ..player1 } 

    ..player1: allows us to copy majority of fields from the old structure without having to explicitly
    type it out. But this must always be the last element .


2. **Defining a Structure**
-------------------------

To define a structure, you use the `struct` keyword followed by the name of the structure and the fields 
it contains. Here's an example:

    ```rust
        struct Person {
            name: String,
            age: u32,
        }
    ```

3. **Instantiating a Structure**
-----------------------------

To create a new instance of a structure, you use the `Person` keyword followed by the values for each field.
You can do this in any order relative to the order of fields in the struct decleration:
Here's an example:

    ```rust
        let person = Person {
            name: "John".to_string(),
            age: 30,
        };
    ```
3.1 Creating Instance from other instances:

Rust struct has update syntax to copy some or all of the values from one instance to another:

    ```rust
        let p1 = Person {
            name: "John".to_string(),
            person
        };
    ```

4. **Accessing Structure Fields**
------------------------------

To access a field of a structure, you use the dot notation. Here's an example:

    ```rust
        println!("{}", person.name); // prints "John"
        println!("{}", person.age); // prints 30
    ```

Declare the instance as mutable, allows to change its fields:

    ```rust 
    let mut person1 = Person {
        name: String::from("username"),
        age: 14,
    };
    person1.age = 24;
    ```
Note: Mutablity is applicable for entire structure. Individual structure elements can not be declared
mutable and immutable.

5. Returning a structure from a function:

    You can also construct a struct instance as the last expression in a function to implicitly return it:

    ```rust 
        fn new_user (user_name: String, age: i32 ) -> Person {
            Person {
                name: user_name,
                age,
            }
        }
    ```
6. **Structure Methods**
----------------------

You can define methods on a structure using the `impl` keyword. Here's an example:

    ```rust
        impl Person {
            fn new(name: String, age: u32) -> Person {
                Person { name, age }
            }

            fn greet(&self) {
                println!("Hello, my name is {} and I am {}", self.name, self.age);
            }
        }
    ```

7. **Structure Traits**
---------------------

Traits are similar to interfaces in other languages. 
They define a set of methods that a structure must implement. Here's an example:

    ```rust
        trait Greeter {
            fn greet(&self);
        }

        impl Greeter for Person {
            fn greet(&self) {
                println!("Hello, my name is {} and I am {}", self.name, self.age);
            }
        }
    ```

8. **Tuple Structures**
---------------------

Tuple structures are a type of structure that uses the `struct` keyword followed by a tuple of fields. 
Here's an example:

    ```rust
        struct Point(u32, u32);
    ```

9. **Unit Structures**
-------------------

Unit structures are a type of structure that has no fields. They are often used as markers or flags. 
Here's an example:

    ```rust
        struct Unit;
    ```

10. **Structure Derive**
---------------------

Rust provides a way to automatically implement certain traits for a structure using `#[derive]` attribute. 
Here's an example:

    ```rust
    #[derive(Debug)]
    struct Person {
        name: String,
        age: u32,
    }

    fn main() {
        let person = Person {
            name: "John".to_string(),
            age: 30,
        };
        println!("{:?}", person); // prints "Person { name: \"John\", age: 30 }"
    }
    ```

11. **Structure Pattern Matching**
------------------------------

Rust provides a way to pattern match on structures using the `match` keyword. Here's an example:

    ```rust
        struct Point(u32, u32);
        
        fn main() {
            let point = Point(1, 2);
            match point {
                Point(0, 0) => println!("Origin"),
                Point(0, y) => println!("On the y-axis at {}", y),
                Point(x, 0) => println!("On the x-axis at {}", x),
                Point(x, y) => println!("At ({}, {})", x, y),
            }
        }
    ```

12. Structs and Ownership:

    Struct fields often own their data like the String type, which owns its contents.
    If we need to include references in your struct, you must use lifetimes to ensure that the data
    referred to by the struct is valid for the lifetime of the struct. This ensures safety and prevents data
    race or dangling referenecs.


    
**Best Practices**
------------------

Here are some best practices to keep in mind when working with structures in Rust:

*   Use meaningful field names: Use field names that accurately describe the data they contain.
*   Use `impl` blocks: Use `impl` blocks to define methods on a structure.
*   Use traits: Use traits to define a set of methods that a structure must implement.
*   Use derive: Use the `#[derive]` attribute to automatically implement certain traits for a structure.

