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
------------------------------------------

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
Methods can be classified into types:

- Instance Methods: These methods take "self" as an argument and operate on instance of the structure.
They are defined using "&self" or "&mut self" syntax.

- Static Methods: These methods do not take "self" as an argument and opearte on the structures itself.
They are using the "Self" syntax.

- Associated functions: These functions are not methods but are associated with the structure. They are
  defined using the "fn" keywords without "self".

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


    
13. **Best Practices**
------------------

Here are some best practices to keep in mind when working with structures in Rust:

*   Use meaningful field names: Use field names that accurately describe the data they contain.
*   Use `impl` blocks: Use `impl` blocks to define methods on a structure.
*   Use traits: Use traits to define a set of methods that a structure must implement.
*   Use derive: Use the `#[derive]` attribute to automatically implement certain traits for a structure.


14. structures Example:

    ```rust 
        #![allow(warnings)]
        // Structures are used to name and package related values similar to tuples.
        fn main () {
            //tuple 
            let rect = (100,200); // width and height

            //Structure 
            struct Book {
                title     : String,
                author    : String,
                pages     : u32, 
                available :  bool,
            }

            struct User {
                active    : bool,
                username  : String,
                email     : u32, 
                sign_in_count :  u64,
            }
    
            // Instance of the structure
            let mut user1 = User {
                active      : true,
                username    : String::from("matrix"),
                email       : String::from("matrix@system.ai"),
                sign_in_count : 1,
            }

            //change user1's email:
            user1.email = String::from("newemail@system.ai");
            println!("{}", user1.email);

            // Return a structure from a function: 


            
        }

        
    ```


# Additional Material: 

---

1. Interfaces: ( CPP )

**Interfaces in C++**
======================

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

