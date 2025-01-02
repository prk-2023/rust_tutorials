# Option and Result:

In Rust, `Option` and `Result` are two fundamental data types that help handle situations where a value may 
or may not be present, or where an operation may succeed or fail.

### Option

Ref: https://doc.rust-lang.org/nightly/core/option/index.html

The `Option` enum is used to represent a value that may or may not be present. It has two variants:

* `Some(value)`: represents a value that is present.
* `None`: represents the absence of a value.

pub enum Option<T> {
    None, 
    Some(T)
}

Options are commonly paired with pattern matching to query the presence of a value and take action, always 
accounting for the None case.

Here's an example:
    ```rust
    let some_value: Option<i32> = Some(5);
    let no_value: Option<i32> = None;
    ```
You can use `Option` to handle situations where a value may not be available, such as when reading from a 
file or network connection.

To work with `Option`, you can use various methods, such as:

* `unwrap()`: returns the value inside `Some` or panics if it's `None`.
* `expect()`: similar to `unwrap()`, but allows you to specify a custom error message.
* `is_some()` and `is_none()`: check if the value is present or absent.
* `map()`, `filter()`, and `and_then()`: apply transformations to the value inside `Some`.

Example:

    ```rust
    let some_value: Option<i32> = Some(5);
    let value = some_value.unwrap(); // returns 5
    println!("{}", value); // prints 5

    let no_value: Option<i32> = None;
    // no_value.unwrap(); // would panic!
    ```
### Result

Ref: https://doc.rust-lang.org/nightly/core/result/enum.Result.html

The `Result` enum is used to represent the outcome of an operation that may succeed or fail. 
It has two variants:

* `Ok(value)`: represents a successful outcome with a value.
* `Err(error)`: represents a failed outcome with an error.

pub enum Result<T,E> {
    Ok<T), // contains the success value
    Err(E), // contains the error value
}

Here's an example:
    ```rust
    let success: Result<i32, &str> = Ok(5);
    let failure: Result<i32, &str> = Err("Something went wrong");
    ```
You can use `Result` to handle situations where an operation may fail, such as when reading from a file or 
network connection.

To work with `Result`, you can use various methods, such as:

* `unwrap()`: returns the value inside `Ok` or panics if it's `Err`.
* `expect()`: similar to `unwrap()`, but allows you to specify a custom error message.
* `is_ok()` and `is_err()`: check if the outcome was successful or failed.
* `map()`, `filter()`, and `and_then()`: apply transformations to the value inside `Ok`.
* `map_err()`: applies a transformation to the error inside `Err`.

Example:
    ```rust
    let success: Result<i32, &str> = Ok(5);
    let value = success.unwrap(); // returns 5
    println!("{}", value); // prints 5

    let failure: Result<i32, &str> = Err("Something went wrong");
    // failure.unwrap(); // would panic!
    ```
### Combining Option and Result

You can combine `Option` and `Result` to handle more complex situations. For example:
    ```rust
    let value: Option<Result<i32, &str>> = Some(Ok(5));
    let value = value.unwrap().unwrap(); // returns 5
    println!("{}", value); // prints 5
    ```
In this example, we have an `Option` that contains a `Result`. 
We use `unwrap()` twice to extract the value inside `Some` and then inside `Ok`.

### Best Practices

When working with `Option` and `Result`, it's essential to follow best practices to avoid common pitfalls:

* Use `unwrap()` and `expect()` sparingly, as they can lead to panics and crashes.
* Instead, use `match` or `if let` statements to handle `Option` and `Result` values explicitly.
* Use `?` operator to propagate errors up the call stack.
* Document your functions and methods to indicate what types of errors they may return.

By following these guidelines and using `Option` and `Result` effectively, you can write robust and error-free code in Rust.
