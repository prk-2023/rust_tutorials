# Panic:

## Rust Panic**
---

Rust, a panic is a way to handle unrecoverable errors that occur during the execution of a program. 
When a panic occurs, the program will stop executing and display an error message.

### How Rust Panic Works :

- When a panic occurs, the following steps happen:

    1.  The current thread is stopped.
    2.  The panic message is printed to the console.
    3.  The program unwinds the stack, which means it calls the `drop` method on all variables in the
        current scope and all parent scopes.
    4.  If the `std::panic::catch_unwind` function is used, it will catch the panic and allow the program to
        continue executing.
    5.  If not, the program will exit with a non-zero status code.

### Using Panic in Different Cases :


#### **1. Unrecoverable Errors**

Panic can be used to handle unrecoverable errors, such as out-of-memory errors or invalid input.

    ```rust
        fn divide(a: i32, b: i32) -> i32 {
            if b == 0 {
                panic!("Cannot divide by zero!");
            }
            a / b 
        }
        fn main() {
            let result = divide(10, 0);
            println!("Result: {}", result);
        }
    ```

example, if the user tries to divide by zero, the program will panic and display an error message.

#### **2. Debugging**

Panic can be used to debug a program by panicking when a certain condition is met.

    ```rust 
        fn main() { 
            let x = 5; 
            assert_eq!(x, 10); // This will panic because x is not equal to 10 
        }
    ```

In this example, the `assert_eq!` macro will panic if the condition is not met.

#### **3. Testing**

Panic can be used in tests to ensure that a certain condition is met.

    ```rust
    #[test]
    fn test_divide() {
        assert_eq!(divide(10, 2), 5);
    }
    ```

the `assert_eq!` macro will panic if the condition is not met, and the test will fail.

#### **4. Custom Panic Messages**

Panic can be used to display custom error messages.

    ```rust
        fn main() {
            panic!("This is a custom panic message!");
        }
    ```

the program will panic and display the custom error message.

#### **5. Panic in a Closure**

Panic can be used in a closure.

    ```rust
        fn main() { 
            let closure = || { 
                panic!("This is a panic in a closure!");
            };
            closure();
        }
    ```

the closure will panic when it is called.

#### **6. Panic in a Thread**

Panic can be used in a thread.

    ```rust
        use std::thread;

        fn main() {
            let handle = thread::spawn(|| {
                panic!("This is a panic in a thread!");
            });
            handle.join().unwrap();
        }
    ```

the thread will panic when it is joined.

### Best Practices**
---

*   Use panic sparingly, as it can make the program harder to debug.
*   Use custom panic messages to provide more information about the error.
*   Use `std::panic::catch_unwind` to catch panics and handle them gracefully.
*   Avoid using panic in production code, as it can cause the program to exit unexpectedly.

### panic and binary size:
---

#### Controlling Panic Code Size**


"panic" code gets compiled and parceled along with the binary when on compilation of a rust program

However, there are ways to control the size of the panic code:

    1.  **`panic = "abort"`**: abort the program immediately when a panic occurs, without unwinding the
        stack or printing an error message. 

        This can significantly reduce the size of the panic code.

        this can be achived by an entry in the Cargo.toml file:

        ```toml
        [profile.release]
        panic = "abort"
        ```

    2.  **`panic = "unwind"`**: unwinds the stack and print an error message when a panic occurs. 

        However, it does not include the full panic message, which can reduce the size of the panic code.

        ```toml 
        [profile.release] 
        panic = "unwind"
        ```

    3.  **`strip`**: strip the binary of unnecessary symbols, which can reduce the size of the binary.
        This option is not related to panic but used to reduce the size of the binary.

        ```toml
        [profile.release]
        strip = true
        ```

    4.  **`opt-level`**: controls the level of optimization performed by the compiler. 

        Higher optimization levels can reduce the size of the binary.

        ```toml
        [profile.release]
        opt-level = "z"
        ```

    5. apart from controling the size via Cargo.toml use of smaller memory allocator in place of the default
       `jemalloc` allocator,( jemalloc: is designed for performance rather than size) such as `wee_alloc` 
       allocator can reduce the size of the binary. 

    6. Remove unrequired dependencies. 

    7. Use "cargo --release" to generate stripped binary.
    
        `$ cargo build --release --target thumbv7em-none-eabi`

    Cargo builds program with optimizations enabled, targeting the `thumbv7em-none-eabi` platform.

**Other Programs Packed with Rust**
------------------------------------

When a Rust program is compiled, several other programs are included in the binary:

1.  **`std`**: The Rust standard library is included in the binary. However, the `std` library is designed to be highly modular, so only the parts of the library that are actually used by the program are included in the binary.
2.  **`libc`**: The C standard library is included in the binary, as Rust uses it for certain operations such as file I/O and networking.
3.  **`libm`**: The math library is included in the binary, as Rust uses it for certain mathematical operations.
4.  **`libgcc`**: The GCC runtime library is included in the binary, as Rust uses it for certain operations such as exception handling.
5.  **`libpthread`**: The POSIX threads library is included in the binary, as Rust uses it for certain operations such as threading.
6.  **`libdl`**: The dynamic linker library is included in the binary, as Rust uses it for certain operations such as dynamic linking.
7.  **`librt`**: The real-time library is included in the binary, as Rust uses it for certain operations such as timing and scheduling.
Here's an example of how to reduce the size of a binary using the `cargo build` command:


# Divergent Functions that never return **!** 

In Rust a function that has a return type of `!` if known as **Divergent Function**. This means that the
function will never return a value, either because it causes the program to panic, or enters an infinite
loop, Or exits the program in such a way that no normal return occurs. 

## Key points about `!` (the never type):

* **`!` (Never Type)**: This type represents a value that cannot exist. It is used for funcs that either:

  * **Panics** (using `panic!()`),
  * **Exits** the program (via `std::process::exit()`),
  * **Runs forever** (with an infinite loop like `loop {}`).

## Common scenarios where `!` is used:

### 1. **Infinite Loops**:
   If you have a function that runs forever (without returning), its return type can be `!`.

   Example:

   ```rust
   fn infinite_loop() -> ! {
       loop {
           // This will run forever
       }
   }
   ```

### 2. **Panic**:
   If a function can panic, it can also have a return type of `!`. A panic stops the program's normal 
   execution, so the function will never return.

   Example:

   ```rust
   fn cause_panic() -> ! {
       panic!("This is a panic!");
   }
   ```

### 3. **Exiting the Program**:
   A function can use `std::process::exit` to stop the program and never return from that function.

   Example:

   ```rust
   fn exit_program() -> ! {
       std::process::exit(1); // This exits the program with status code 1
   }
   ```

### 4. **Unreachable Code**:
   If you have a function that doesn't have any return, or has a condition where no return is possible, you 
   can use `!`. For example, after a `return` or a `panic!()` call, you can indicate that the code will 
   never get past that point.

   Example:

   ```rust
   fn unreachable_code() -> ! {
       return; // or panic!("Never reach here!");
   }
   ```

## The Behavior of Functions with `!`:

* **The type `!`** is often used in cases where you would normally expect a function to return a value, but
  instead, it never finishes execution.

* **Control Flow**: When the compiler sees that a function has the return type `!`, it understands that any 
  code after that function call is unreachable. 
  This can help the compiler with optimizations, as it knows that a path in the program will never be taken.

### Example: A function with a panic:

```rust
fn fail() -> ! {
    panic!("This function will never return!");
}

fn main() {
    fail(); // This will panic and terminate the program
    println!("This line will never be reached!");
}
```

Output:

```
thread 'main' panicked at 'This function will never return!', src/main.rs:2:5
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

In this example, the `fail` function causes a panic, and the program terminates. The line `println!("This line will never be reached!");` will never be executed, because the program has already terminated due to the panic.

### Why Use `!`?

Using `!` in Rust is a way of expressing that a function doesn't return normally. It's useful in situations like:

* Functions that stop the program or cause an unrecoverable error (such as panicking or exiting).
* Expressing functions that run forever, like in embedded systems or servers that wait for input indefinitely.

### In Summary:

* A function with a return type of `!` indicates that the function **never returns**.
* It could be due to:

  * An infinite loop (`loop {}`),
  * A panic (`panic!()`),
  * Or program exit (`std::process::exit()`).

This feature makes Rust's type system expressive and helps with control flow analysis and optimizations.
