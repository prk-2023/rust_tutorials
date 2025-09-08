# Error Handling: 

---
- Ref:  https://www.youtube.com/watch?v=j-VQCYP7wyw

## Introduction
---

Rust prioritizes safety and reliability. Error handling is one of the key features that enables this is its 
robust.

Topics covered Rust error handling, exploring the concepts, mechanisms, and best practices that make it so 
effective.


### Why Error Handling Matters
    
    1. Error handling is crucial in any prog lang, it allows developers to anticipate and respond to
       unexpected events events or conditions that may arise during program execution.
 
    2. Rust, error handling is particularly important due to its focus on memory safety and preventing
       common errors like null pointer dereferences or buffer overflows.

### Error Types in Rust
---

Rust has two primary types of errors:
    1.  **Recoverable Errors**: 
        Errors that can be recovered from, such as file not found or network connection errors. 
        Recoverable errors are typically represented using the ** `Result` ** type.
 
    2.  **Unrecoverable Errors**: 
        These are errors that cannot be recovered from, such as out-of-memory errors or invalid memory 
        accesses. Unrecoverable errors are typically represented using the ** `panic!` ** macro.


- **The `Result` Type**

    The `Result` type is a built-in enum in Rust that represents a value that may or may not be present. 
    has two variants:

        *   `Ok(value)`: Represents a successful outcome with a value.
        *   `Err(error)`: Represents a failed outcome with an error.

    
    Here's an example of using the `Result` type:

    ```rust
    fn divide(x: i32, y: i32) -> Result<i32, &'static str> {
        if y == 0 {
            Err("Division by zero!")
        } else {
            Ok(x / y)
        }
    }
    fn main() {
        match divide(10, 2) {
            Ok(result) => println!("Result: {}", result),
            Err(error) => println!("Error: {}", error),
        }
    }
    ```

- panic!() : panic! is a macro that signals an unrecoverable error in a program. 
    When panic! is called, the program's execution stops, and it typically unwinds the stack, running 
    destructors for any local variables in scope. 
    This mechanism is used when a program enters a state that it cannot handle or recover from, and 
    continuing execution would lead to undefined behavior or logical inconsistencies.

    Depending on the configuration, panic! might unwind the stack and run destructors, or it might
    immediately abort the process.

    panic! can be explicitly called by developers using the panic! macro. 
    It is also implicitly called by other functions and macros, such as unwrap() and expect() on Option or 
    Result types when they encounter a None or Err value, respectively.

- **Error Propagation**
---

    Error propagation is the process of passing errors up the call stack to be handled by the caller. 
    In Rust, error propagation is typically done using the `?` operator, which returns early from a function 
    with an error if the expression it's used with returns an error.

    Here's an example of error propagation:
    ```rust
    fn read_file(path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }
    fn main() -> Result<(), std::io::Error> {
        let contents = read_file("example.txt")?; // the ? operator to propagate the error up the stack
        println!("File contents: {}", contents);
        Ok(())
    }
    ```

- **The `?` Operator**
---

    The `?` operator is a shorthand for error propagation. 
    It's used to return early from a func with an error if the expression it's used with returns an error.

    The ? operator is generally used with `Result` and `Option` types. 
    It provides a concise way to handle potential errors or `None` values by either extracting the success 
    value or returning early with the `error`/`None`.

    Early Return on Error:
        If the expression on which ? is applied evaluates to an Err variant (for Result) or None variant 
        (for Option), the ? operator will immediately return that Err or None from the current function. 
        This does affect the program logic flow by preventing subsequent code within that function from 
        executing.

    Unwrapping on Success:
        If the expression evaluates to an Ok variant (for Result) or Some variant (for Option), 
        the ? operator unwraps the value inside, and the program continues execution as normal.


    Example: comparing with and without the error ? operation:

    ```rust  
    // with out the ? error opretaion to read and parse a file contents 
    // Handling error manually requires a verbose match statement to check for an error after each step.

    use std::fs::File;
    use std::io::{self, Read};
    
    fn read_and_process() -> Result<i32, io::Error> {
        let mut file = match File::open("data.txt") {
            Ok(f) => f,
            Err(e) => return Err(e),
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => {},
            Err(e) => return Err(e),
        };

        let number: i32 = match contents.trim().parse() {
            Ok(n) => n,
            Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid number format")),
        };
        Ok(number * 2)
    }
    ```

    With ? error operator the above code can be written as :

    ```rust 
    use std::fs;
    use std::io;
    
    fn read_and_process() -> Result<i32, Box<dyn std::error::Error>> {
        let contents = fs::read_to_string("data.txt")?; // <- Early return on File::open or read_to_string error
        let number = contents.trim().parse::<i32>()?; // <-- Early return on parse error

        Ok(number * 2)
    }

    ```

    Example2: of using the `?` operator: ( example same as above )

    ```rust 
    fn read_file(path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }
    fn main() -> Result<(), std::io::Error> {
        let contents = read_file("example.txt")?;
        println!("File contents: {}", contents);
        Ok(())
    }
    ```

- **Custom Error Types**
---

    Rust allows you to define custom error types using the `enum` keyword. 
    Custom error types can be used to represent domain-specific errors that aren't covered by the std lib.
    
    Here's an example of defining a custom error type:
    ```rust 
    enum CustomError {
        InvalidInput,
        NetworkError,
    }
    fn read_file(path: &str) -> Result<String, CustomError> {
        if path.is_empty() {
            Err(CustomError::InvalidInput)
        } else {
            // Simulate a network error 
            Err(CustomError::NetworkError)
        }
    }
    fn main() {
        match read_file("example.txt") {
            Ok(contents) => println!("File contents: {}", contents),
            Err(error) => match error {
                            CustomError::InvalidInput => println!("Invalid input"),
                            CustomError::NetworkError => println!("Network error"),
                          },
        }
    }
    ```


- **Error Handling in Test Programs**
---

    It's essential to handle errors properly to ensure that your tests are reliable and accurate. 
    Here are some best practices for error handling in test programs:

    1. **Use `assert!` and `assert_eq!`**: 
        macros to assert that a condition is true or that two values are equal. 
        If the assertion fails, the test will panic and report an error.

    2. **Use `Result` and `?`**: 
        When writing test functions that return a `Result`, use the `?` operator to propagate errors up 
        the call stack.

    3. **Use `unwrap` and `expect` carefully**: 
        These methods can be used to unwrap a `Result` or `Option` value, but they will panic if the value 
        is an error. Use them carefully and only when you're sure that the value will not be an error.

    example of error handling in a test program:

    ```rust 
    #[cfg(test)]
    mod tests { 
        use super::*;
        #[test]
        fn test_divide() {
            assert_eq!(divide(10, 2).unwrap(), 5);
            assert_eq!(divide(10, 0).unwrap_err(), "Division by zero!");
        }
    }
    ```

- **Error Handling in Production Code**
---

    Essential to handle errors properly to ensure that your program is reliable and robust. 
    Here are some best practices for error handling in production code:

    1. **Use `Result` and `?`**: 
        When writing functions that return a `Result`, use the `?` operator to propagate errors up the 
        call stack.

    2. **Use `match` and `if let`**: 
        These statements can be used to handle errors explicitly and provide a more robust error handling 
        mechanism.

    3. **Log errors**: 
        Use a logging framework to log errors and provide a way to diagnose and debug issues.

    4. **Provide error messages**: 
        Provide meaningful error messages to help users understand what went wrong and how to fix it.

    example of error handling in production code:
    
    ```rust 
    fn main() -> Result<(), std::io::Error> { 
        
            let contents = read_file("example.txt")?; 
            println!("File contents: {}", contents);
            Ok(())
    }

    fn read_file(path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }
    ```
## Best Practices
---

1. **Use `Result` instead of `Option`**: 
    `Result` is a more expressive type that can represent both success and failure, whereas `Option` can 
    only represent the absence or presence of a value.

2. **Use `?` for error propagation**: 
    `?` operator is a concise way to propagate errors up the call stack.

3. **Define custom error types**: 
    Custom error types can help you represent domain-specific errors that aren't covered by the std lib.

4. **Handle errors explicitly**: 
    Avoid ignoring errors or using `unwrap` or `expect` without a good reason. 
    Instead, handle errors explicitly using `match` or `if let`.

The above points are required to understand when and how to use the error handling for writing more robust
and reliable code.


---

## Additional material and examples:

**Why Error Handling Matters**


- **Error Types in Rust**
---

1.  Recoverable Errors: These are errors that can be recovered from, these errors can be represented using 
    the `Result` type.

2.  Unrecoverable Errors: Errors that cannot be recovered from, (out-of-memory invalid memory accesses...)
    These errors are typically represented using the `panic!` macro.

- **The `Result` Type**
---

    `Result` type is a built-in enum in Rust that represents a value that may or may not be present. 
    It has two variants:
    *   `Ok(value)`: Represents a successful outcome with a value.
    *   `Err(error)`: Represents a failed outcome with an error.


    Example of using the `Result` type:

    ```rust
    fn divide(x: i32, y: i32) -> Result<i32, &'static str> {
        if y == 0 {
            Err("Division by zero!")
        } else {
            Ok(x / y)
        }
    }
    fn main() {
        match divide(10, 2) {
            Ok(result) => println!("Result: {}", result),
            Err(error) => println!("Error: {}", error),
        }
    }
    ```
- **Error Propagation**
---

    Process of passing errors up the call stack to be handled by the caller. 

    Error propagation is typically done using the `?` operator, which returns early from a 
    function with an error if the expression it's used with returns an error.

    example of error propagation:
    ```rust
    fn read_file(path: &str) -> Result<String, std::io::Error> {
        std::fs::read_to_string(path)
    }
    fn main() -> Result<(), std::io::Error> {
        let contents = read_file("example.txt")?;
        println!("File contents: {}", contents);
        Ok(())
    }
    ```

- **The `?` Operator**
---

    `?` operator represents error propagation. 
    used to return early from a function with an error if the expression it's used with returns an error.

    example of using the `?` operator:
    ```rust 
    fn read_file(path: &str) -> Result<String, std::io::Error> { 
        std::fs::read_to_string(path)
    }

    fn main() -> Result<(), std::io::Error> {
        let contents = read_file("example.txt")?;
        println!("File contents: {}", contents);
        Ok(())
    }
    ```

- **Custom Error Types**
----------------------

To define custom error types using the `enum` keyword. 
Custom error types can be used to represent domain-specific errors that aren't covered by the std lib.

    Example of defining a custom error type:
    ```rust 
    enum CustomError { 
        InvalidInput,
        NetworkError,
    }

    fn read_file(path: &str) -> Result<String, CustomError> {
        if path.is_empty() {
            Err(CustomError::InvalidInput)
        } else {
            // Simulate a network error
            Err(CustomError::NetworkError)
        }
    }

    fn main() {
        match read_file("example.txt") {
            Ok(contents) => println!("File contents: {}", contents),
            Err(error) => match error {
                CustomError::InvalidInput => println!("Invalid input"),
                CustomError::NetworkError => println!("Network error"),
            },
        }
    }
    ```

# Additional 
Ref : https://dev.to/nathan20/how-to-handle-errors-in-rust-a-comprehensive-guide-1cco

Purpose: Help identify, debug, and resolve errors that occur during the execution of a program.

Helps to ensure smooth functioning of the program by preventing errors from occuring and allowing program to
continue running in an optimal state.

Error handling also allows users to be informed of any problems that may arise and take corrective action to
prevent the errors from happening again in the future.

## Result type:
    Its a built-in enum in Rust Standard library:
    It has 2 states or variants Ok(T) and Err(E)

    enum Result<T,E> {
        Ok(T),
        Err(E),
    }

    This type is used for any function that can potentially encounter error situations.
    Ok value is return in case of success or Err in case of error.

    ex:
    fn picture_found() -> Result<i64, bool> {
        if i {  // if true 
            Ok(400) // return Ok(400)
        } else {
            Err(false) // return Err (false )
        }
    }
## Summary:

What is error handling?

Sometime while using functions that can fail can encounter errors and in Rust, error handling is the process
of managing failures in a safe and predictable way.

Rust uses two main types for this:
1. `Result<T,E>` for recoverable errors ( ex: file not found or connection failure )
2. `panic!()` macro for unrecoverable error (ex:  out-of-bound access)

`Result<T,E>` allows for gracefull handling of errors using pattern matching, ? error operator or methods
like `.unwrap_or()` and `.expect()`


`panic!` when called, the program's execution stops, and it typically unwinds the stack, running destructors
for any local variables in scope. 

This mechanism is used when a program enters a state that it cannot handle or recover from, and continuing 
execution would lead to undefined behavior or logical inconsistencies.

Depending on the configuration, panic! might unwind the stack and run destructors, or it might immediately 
abort the process.

panic! can be explicitly called by developers using the panic! macro. 
It is also implicitly called by other functions and macros, such as unwrap() and expect() on Option or 
Result types when they encounter a None or Err value, respectively.
