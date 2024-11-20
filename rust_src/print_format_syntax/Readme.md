# print format syntax:


## Format Syntax:

- Format syntax is used to create formatted strings.
- Format syntax is  based on std::fmt module and provides a way to insert values into a string template.

## Module std::fmt 

- Utility for formating and printing Strings.

- Module contains the runtime support for the "format!" syntax extension. This macro is implemented in the
  compiler to emit calls to this module in order to format arguments at runtime into string.

- **Basic Format Syntax**

    The basic format syntax in Rust is `format!`. It takes a string template as an argument, and you can 
    insert values into the template using `{}` placeholders.

    ```rust 
    let name = "John";
    let age = 30;

    let formatted_string = format!("My name is {} and I am {} years old.", name, age);
    println!("{}", formatted_string);
    ```

Usage: "format!" macro extenstions are:

    - format!("HELLO");
    - format!("hello, {}", "world");
    - format!("{:?}", (3,4));
    - format!("{value}", value=4);
    - format!("{:04}", 42);  // => "0042" with leading zeros
    - format!("{:#?}", (100,200)); //  =>   "( 
                                   //           100, 
                                   //           200, 
                                   //        )"


## **Format Specifiers**

- Rust provides several format specifiers that can be used to customize the formatting of values. 
  Some common format specifiers:

  1. `{:?}`: Debug format specifier, used to print the debug representation of a value.
  2. `{:#?}`: Pretty debug format specifier, used to print the debug representation of a value with
     indentation and line breaks.
  3. `{}`: Default format specifier, used to print the default representation of a value.
  4. `{:#}`: Pretty format specifier, used to print the default representation of a value with indentation
     and line breaks.
  5. `{:.precision}`: Precision format specifier, used to specify the number of decimal places to print for
     floating-point numbers.
  6. `{:.width}`: Width format specifier, used to specify the minimum width of the output.
  7. `{:.width.precision}`: Width and precision format specifier, used to specify both the minimum width and
     the number of decimal places to print for floating-point numbers.


### **Examples**

Here are some examples of using format specifiers:

    ```rust
    let pi = 3.14159265359;

    println!("Pi is {:.2}", pi); // Output: Pi is 3.14
    println!("Pi is {:#.2}", pi); // Output: Pi is 3.14 (with indentation and line breaks)
    println!("Pi is {:10.2}", pi); // Output: Pi is     3.14 (with minimum width of 10)
    ```

### **Named Arguments**

- Rust also supports named arguments in the format syntax. 
- You can use the `format!` macro with named args to specify the values to be inserted into the template.

    ```rust
    let name = "John";
    let age = 30;

    let formatted_string = format!(name = name, age = age, "My name is {name} and I am {age} years old.");
    println!("{}", formatted_string);
    ```

### **Custom Format Implementations**

- Rust also allows you to implement custom format implementations for your own types. 
- You can implement the `std::fmt::Display` trait to provide a custom format implementation for your type.

    ```rust
    struct Person {
        name: String,
        age: u32,
    }

    impl std::fmt::Display for Person {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            write!(f, "Person {{ name: {}, age: {} }}", self.name, self.age)
        }
    }

    let person = Person {
        name: String::from("John"),
        age: 30,
    };

    println!("{}", person);
    ```

### **{:?}** format syntax to print structures and enums

    this syntax is called "debug" format specifier, and used in debug representation of value:

    ```rust 

    #[derive(Debug)]
    struct Person {
        name:   String,
        age:    i32,
    }

    fn main () {
        let person = Person { 
            name: String::from("spidername"),
            age: 30,
        };

        println!("{:?}", person);
    }
    ```

Note: if we do not derive the "Debug" trait for the Person structure would give error.

### **Custom implementation of "Debug" trair for structure and enum**

- Rust allows us to implement the "debug" trair for a struct or enum :

    ```rust 

    use std::fmt;

    struct Person {
        name:   String,
        age:    i32,
    }

    impl fmt::Debug for Person {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Person {{name :{}, age: {} }}", self.name , self.age)
        }
    }
    fn main () {
        let person = Person {
            name : String::from("abcd"),
            age: 30,
        };

        println!("{:?}, person");
    }

    ```

