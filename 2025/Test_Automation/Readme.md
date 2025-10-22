# Writing Automated Tests:

## Overview of Testing

Automated tests are essential for ensuring that your code behaves as expected, especially as it grows and 
evolves. 

Rust has built-in support for writing tests with a test framework that is part of the standard library.

This chapter covers:

* The basics of writing tests,
* Unit tests and integration tests,
* How to run tests, and
* Techniques for organizing and grouping tests.

### Writing Basic Tests

In Rust, tests are functions marked with the `#[test]` attribute. 

By default, these tests are located in the same file as your code, but you can also organize them in separate 
files.

### **Test Function Structure**

The body of the test function typically performs 3 actions:
- Set up any needed data or state.
- Runt he code you want to test.
- Assert that results are what you expect. 
Along with the above `test` attribute, a few macros, and the `should_panic` attribute.

`#[test]` attribute is added before a function `fn` ( when you run `cargo test` rust builds a test runner
binary that runs the annotated function and report on whether each test function passes or fails.)

When we make a new library project with cargo, a test module with a test function in it automatically
generated for use. 

A basic test function looks like this:

```rust
#[cfg(test)]  // Marks this module for testing
mod tests {
    #[test] // Marks this function as a test
    fn addition() {
        assert_eq!(2 + 2, 4); // Asserts that 2 + 2 is equal to 4
    }
}
```

### **Test Assertions**

There are several assertion macros available to verify conditions:

* `assert_eq!(left, right)`: Tests if `left` equals `right`.
* `assert_ne!(left, right)`: Tests if `left` is not equal to `right`.
* `assert!`: Tests if the given condition is `true`.

Here’s an example of using `assert_ne!`:

```rust
#[cfg(test)]
mod tests {
    #[test]
    fn subtraction() {
        assert_ne!(5 - 3, 1); // Asserts that 5 - 3 is not equal to 1
    }
}
```

---

## **Unit Tests**

Unit tests are the most common type of tests in Rust. 
They test small, isolated units of functionality, such as individual functions or methods.

### **Example: Testing a Function**

Let’s write a function that calculates the area of a rectangle and write a test for it.

```rust
// lib.rs or main.rs
pub fn area_of_rectangle(width: u32, height: u32) -> u32 {
    width * height
}

#[cfg(test)]
mod tests {
    use super::*; // Brings the `area_of_rectangle` function into scope

    #[test]
    fn test_area_of_rectangle() {
        assert_eq!(area_of_rectangle(3, 4), 12); // 3 * 4 = 12
        assert_eq!(area_of_rectangle(5, 6), 30); // 5 * 6 = 30
    }
}
```

### **Running Tests**

To run the tests, use the `cargo test` command in the terminal. It compiles and runs all tests in your project.

```bash
cargo test
```

### **Test Output**

When you run tests, the output will tell you how many tests passed or failed:

```
running 1 test
test tests::test_area_of_rectangle ... ok
```

---

## **Testing with `Result`**

Sometimes, your code may return a `Result` or `Option`. 

In these cases, you can use the `assert!` macro with these types to test conditions:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_some_result() {
        let result = Some(42);
        assert!(result.is_some()); // Tests that the result is `Some`
        assert_eq!(result.unwrap(), 42); // Unwraps and checks the value
    }
}
```
---

## **Integration Tests**

In addition to unit tests, Rust also supports integration tests. 
These tests verify that different parts of your program work together as expected.

### **Integration Test Structure**

Integration tests are placed in the `tests` directory in your project (outside the `src` directory). 
You don’t need to include `#[cfg(test)]` or the `mod tests` block here.

For example, let’s create a file `tests/integration_test.rs`:

```rust
// tests/integration_test.rs
extern crate my_project; // Import the crate to test

use my_project::area_of_rectangle;

#[test]
fn test_integration() {
    assert_eq!(area_of_rectangle(2, 3), 6);
    assert_eq!(area_of_rectangle(7, 8), 56);
}
```

To run all integration tests, use the same `cargo test` command. 
It will automatically discover the tests in the `tests` folder.

---

## **Test Organization**

To keep tests organized, it’s common practice to group related tests into separate modules.

### **Example: Organizing Tests**

You can split tests into different modules to better reflect the structure of your project:

```rust
#[cfg(test)]
mod tests {
    mod rectangle_tests {
        use super::*;
        
        #[test]
        fn test_area_of_rectangle() {
            assert_eq!(area_of_rectangle(4, 5), 20);
        }
    }

    mod utility_tests {
        // Other utility functions and tests
    }
}
```

---

## **Testing Private Functions**

If you need to test private functions in your module, the test module can access them because both the 
test functions and the code they are testing are within the same crate. 

In the `tests` module, simply `use super::*;` to bring them into scope.

---

## **Test Coverage and Benchmarks**

Rust also supports more advanced features like test coverage analysis and benchmarking tests.

### **Benchmarking Tests**

To enable benchmarking, you need to add a special feature to your `Cargo.toml` file:

```toml
[dependencies]
criterion = "0.3"  # Add for benchmarking
```

And in the `benches` directory, you can write performance benchmarks like so:

```rust
#[cfg(test)]
mod benches {
    use criterion::{black_box, Criterion, criterion_group, criterion_main};

    fn benchmark(c: &mut Criterion) {
        c.bench_function("area_of_rectangle", |b| {
            b.iter(|| area_of_rectangle(black_box(5), black_box(6)))
        });
    }

    criterion_group!(benches, benchmark);
    criterion_main!(benches);
}
```

---

## **Test-Driven Development (TDD)**

Rust’s testing tools encourage a **Test-Driven Development (TDD)** approach, where you write tests before 
the implementation code. This is a great way to ensure your program behaves correctly as you develop it.

Here’s an example of writing a test first:

1. **Write a failing test:**

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_perimeter() {
        assert_eq!(calculate_perimeter(3, 4), 14); // Expected perimeter
    }
}
```

2. **Implement the function:**

```rust
pub fn calculate_perimeter(width: u32, height: u32) -> u32 {
    2 * (width + height)
}
```

3. **Run the tests**: Now when you run `cargo test`, it should pass.

---

## checking for Panics with `should_panic`: ( should_panic rust nightly )

In addition to checking return values, its important to check that our code handles `error conditions` as we
expect. 

```rust
pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {value}.");
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}
```
Other code that uses Guess depends on the guarantee that Guess instances will contain only values between 1
and 100. We can write a test that ensures that attempting to create a Guess instance with a value outside 
that range panics.

We do this by adding the attribute should_panic to our test function. The test passes if the code inside the function panics; the test fails if the code inside the function doesn’t panic.



## **Conclusion**

In this chapter, we covered how to:

* Write **unit tests** using the `#[test]` attribute,
* Use various **assertion macros** to check the correctness of your code,
* Organize tests into **modules** for better structure,
* Write **integration tests** in a separate folder (`tests`),
* Test **private functions**,
* And even touched on **test benchmarks** and **TDD**.

Automated testing is a powerful tool in Rust, helping ensure that your code remains reliable and bug-free as it grows.

Would you like to dive deeper into any specific part, such as writing more advanced tests or testing with mock data?
