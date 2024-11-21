# Error Handling: 


- Ref:  https://www.youtube.com/watch?v=j-VQCYP7wyw

**Introduction**
---

Rust prioritizes safety and reliability. Error handling is one of the key features that enables this is its 
robust.

Topics covered Rust error handling, exploring the concepts, mechanisms, and best practices that make it so 
effective.


- Why Error Handling Matters
    
    1. Error handling is crucial in any prog lang, it allows developers to anticipate and respond to
       unexpected events events or conditions that may arise during program execution.
 
    2. Rust, error handling is particularly important due to its focus on memory safety and preventing
       common errors like null pointer dereferences or buffer overflows.

- **Error Types in Rust**
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

    example of using the `?` operator: ( example same as above )

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
- **Best Practices**
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

Additional material and examples:

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
