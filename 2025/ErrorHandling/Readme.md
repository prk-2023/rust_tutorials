# Error Handling:


## Introduction:

Rust has number of features for handling situations in which something goes wrong.

Rust requires programmers to acknowledge the possibility of an error and take some action before your code
compiles. This helps your programs to be robust by ensuring that you'll discover errors and handle them
appropriately before deploying your code to production!

Rust groups errors into two major categories: 

    * Recoverable
    * UnRecoverable 


- Recoverable: File not found error, Permission errors..
-UnRecoverable: Symptoms of bug, such as trying to access a location beyond the end of an array, so we want
to immediately stop the program.


## Error handling in "C":

( Most of the languages there is no such distinction and bundle both types of errors and handle using
mechanisms such as exceptions. )

In Systems programming language like "C" it lacks a built-in mechanism of error propagation or rich error
types. Instead it relies on:

- Return Code ( ex: `int` return value, with "0" meaning success and "-1" or other codes for errors )

- `error` global variable: set by many standard library functions to indicate what kind of error occurred. 

- Manual error checking: programmers must explicitly check return values and take action.

ex: 

```c 
    #include <stdio.h>
    #include <errno.h>

    int main() {
        FILE *file = fopen("nonexistent.txt", "r");
        if (file == NULL) {
            perror("Error opening file");
            return 1; // Recoverable error, handled manually
        }
        fclose(file);
        return 0;
    }
```
This is Recoverable error and program does not crash and we decide what to do (print error and retry..etc). 

- UnRecoverable error handling : Handling UnRecoverable errors in "C", often results in :
    - *Calling* `exit()` to terminate the program.
    - `assert()` failures ( In debug builds )
    - **Undefined behaviour** If the programmer does not handle the error and continues.

Ex:
```C 
    #include <assert.h>

    int divide(int a, int b) {
        assert(b != 0);  // Program aborts if b == 0
        return a / b;
    }
```
This is unrecoverable and program crashed immediately.


## Error handling in Rust: 

Rust Error handling is **Explicit** and Encourages **Safety and Robustness**.

1. Recoverable Errors: `Result<T, E>`

   Rust has powerful enum type:
   ```rust 
       enum Result<T, E> {
           Ok(T),
           Err(E),
       }
   ```
Used for recoverable errors ( file not found , parse errors, permissions....)
Ex:
```rust 
    use std::fs::File;
    fn main() {
        let file = File::open("nonexistent.txt");
        match file {
            Ok(f) => println!("File opened successfully"),
            Err(e) => println!("Error opening file: {}", e),
        }
    }
```

2. UnRecoverable Errors: `panic!()`

Rust uses macro `panic!()` for bugs and serious issues where the program must abort:

Ex:
```rust 
    fn divide(a: i32, b: i32) -> i32 {
        if b == 0 {
            panic!("Division by zero!"); // Unrecoverable error
        }
        a / b
    }
```
=> Unrecoverable Error: Stops the program with a panic message.

"C" gives full control but little Safety- demanding the programmer to handle everything manually.

=> Rust forces you to *distinguish* between recoverable and unrecoverable errors through its type system and
compiler checks. 

Rust avoids exceptions entirely, instead offering structured, explicit error handling with `Result` and
`panic!`.

This design makes Rust safer but can sometime be more verbose when compared to "C" permissive model. ( but
which can be dangerous ...)


### UnRecoverable Errors with `panic!()`:

There are two ways to cause panic in practice:
    - By taking action that causes our code to panic as in the above examples. 
    - Second by explicitly calling `panic!()` macro. 

In both cases we cause a panic in our program. 

By default these panics will print failure message, unwind, clean up the stack, and quit. 

Via an environment variable(RUST_BACKTRACE=1), you can also have Rust display the **call stack** when a 
panic occurs to make it easier to track down the source of the panic.

`$RUST_BACKTRACE=0 ./target/debug/test_program`  <= Disable backtrace 
`$RUST_BACKTRACE=1 ./target/debug/test_program`  <= Enable backtrace 
`$RUST_BACKTRACE=full ./target/debug/test_program`  <= give full detailed backtrace with more frames and
symbols. 

NOTE:
---
 A backtrace is a list of all the functions that have been called to get to this point. 
 Backtraces in Rust work as they do in other languages: the key to reading the backtrace is to start from 
 the top and read until you see files you wrote. 
 That’s the spot where the problem originated. 
 The lines above that spot are code that your code has called; 
 the lines below are code that called your code. These before-and-after lines might include core Rust code, standard library code, or crates that you’re using. 
---

This is backtrace feature is built-in and provides a handy panic backtrace. 
( this is very handy compare to C mainly with embedded systems which requires gdb or external tools )

Rust builds with "cargo build" you get full stack trace with readable function names and source lines.

Rust built with "cargo build --release" backtrace still works but optimizations can inline functions or
strip symbols. Stack trace is shorter, harder to interpret or missing information.

Symbols may be stripped unless you explicitly tell Cargo to keep debug info:

` RUST_BACKTRACE=1 cargo run --release`

Or:

To preserve debug info in release builds update the `Cargo.toml` with the below section:

```toml 
[profile.release]
debug = true
```
This keeps symbols and line info for stack traces with out disabling optimizations.

### Unwinding the stack or Aborting in response to a Panic:

When panic occurs the program starts `unwinding` which means Rust walks back up the stack and cleans up the
data from each function it encounters. This is a more work and Rust allows you to choose the alternative of
immediately `aborting` which ends the program without cleaning up.

Memory that the program was using will then need to be cleaned up bu the OS.
Aborting immediately also reduces the size of binary. 

This can be achieved via Cargo.toml 

```toml 
[profile.release]
debug = 'abort'
```
```rust 
fn main() {
    let v = vec![1,2,3];
    v[99]; // this is illegal and panics
}
```

```bash 
$ RUST_BACKTRACE=full cargo run

warning: unused manifest key: progile
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
     Running `target/debug/errortest`

thread 'main' panicked at src/main.rs:4:6:
index out of bounds: the len is 3 but the index is 99
stack backtrace:
   0:     0x55671f81f8d2 - std::backtrace_rs::backtrace::libunwind::trace::h9c1aa7b29a521839
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/../../backtrace/src/backtrace/libunwind.rs:117:9
   1:     0x55671f81f8d2 - std::backtrace_rs::backtrace::trace_unsynchronized::hb123c31478ec901c
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/../../backtrace/src/backtrace/mod.rs:66:14
   2:     0x55671f81f8d2 - std::sys::backtrace::_print_fmt::hdda75a118fd2034a
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/sys/backtrace.rs:66:9
   3:     0x55671f81f8d2 - <std::sys::backtrace::BacktraceLock::print::DisplayBacktrace as core::fmt::Display>::fmt::hf435e8e9347709a8
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/sys/backtrace.rs:39:26
   4:     0x55671f83a343 - core::fmt::rt::Argument::fmt::h9802ea71fd88c728
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/core/src/fmt/rt.rs:173:76
   5:     0x55671f83a343 - core::fmt::write::h0a51fad3804c5e7c
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/core/src/fmt/mod.rs:1465:25
   6:     0x55671f81d943 - std::io::default_write_fmt::h7e00b0a8732ee2a2
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/io/mod.rs:639:11
   7:     0x55671f81d943 - std::io::Write::write_fmt::h9759e4151bf4a45e
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/io/mod.rs:1954:13
   8:     0x55671f81f722 - std::sys::backtrace::BacktraceLock::print::h1ec5ce5bb8ee285e
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/sys/backtrace.rs:42:9
   9:     0x55671f820796 - std::panicking::default_hook::{{closure}}::h5ffefe997a3c75e4
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/panicking.rs:300:27
  10:     0x55671f820599 - std::panicking::default_hook::h820c77ba0601d6bb
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/panicking.rs:327:9
  11:     0x55671f821042 - std::panicking::rust_panic_with_hook::h8b29cbe181d50030
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/panicking.rs:833:13
  12:     0x55671f820eda - std::panicking::begin_panic_handler::{{closure}}::h9f5b6f6dc6fde83e
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/panicking.rs:706:13
  13:     0x55671f81fdc9 - std::sys::backtrace::__rust_end_short_backtrace::hd7b0c344383b0b61
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/sys/backtrace.rs:168:18
  14:     0x55671f820b6d - __rustc[5224e6b81cd82a8f]::rust_begin_unwind
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/panicking.rs:697:5
  15:     0x55671f802360 - core::panicking::panic_fmt::hc49fc28484033487
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/core/src/panicking.rs:75:14
  16:     0x55671f8024e1 - core::panicking::panic_bounds_check::hfa2ac8420ad021dc
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/core/src/panicking.rs:280:5
  17:     0x55671f802c04 - <usize as core::slice::index::SliceIndex<[T]>>::index::hb07043ab5377f267
                               at /home/daybreak/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:269:10
  18:     0x55671f802d35 - core::slice::index::<impl core::ops::index::Index<I> for [T]>::index::hdbfcf4af32b6fa42
                               at /home/daybreak/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/slice/index.rs:17:15
  19:     0x55671f802d35 - <alloc::vec::Vec<T,A> as core::ops::index::Index<I>>::index::h974f7293a7c9cf47
                               at /home/daybreak/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/alloc/src/vec/mod.rs:3408:9
  20:     0x55671f8031b0 - errortest::main::h2849162dcb7d806d
                               at /tmp/errortest/src/main.rs:4:6
  21:     0x55671f80364b - core::ops::function::FnOnce::call_once::h4ececd9b7cd6ac97
                               at /home/daybreak/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/core/src/ops/function.rs:250:5
  22:     0x55671f802c1e - std::sys::backtrace::__rust_begin_short_backtrace::ha7e28f3f09d7f32d
                               at /home/daybreak/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/sys/backtrace.rs:152:18
  23:     0x55671f8035c1 - std::rt::lang_start::{{closure}}::hcf0fa0770ac00e03
                               at /home/daybreak/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:206:18
  24:     0x55671f81b8b0 - core::ops::function::impls::<impl core::ops::function::FnOnce<A> for &F>::call_once::hf19f6f3c4f0cdb1c
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/core/src/ops/function.rs:284:21
  25:     0x55671f81b8b0 - std::panicking::catch_unwind::do_call::hdc689d1fa1f67ace
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/panicking.rs:589:40
  26:     0x55671f81b8b0 - std::panicking::catch_unwind::h1025d97250558c4b
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/panicking.rs:552:19
  27:     0x55671f81b8b0 - std::panic::catch_unwind::h3f76beef3f07b6dc
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/panic.rs:359:14
  28:     0x55671f81b8b0 - std::rt::lang_start_internal::{{closure}}::haf71a34e0fbc4d76
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/rt.rs:175:24
  29:     0x55671f81b8b0 - std::panicking::catch_unwind::do_call::hbd7dad3d92d409ee
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/panicking.rs:589:40
  30:     0x55671f81b8b0 - std::panicking::catch_unwind::h69749cff2ef3daa8
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/panicking.rs:552:19
  31:     0x55671f81b8b0 - std::panic::catch_unwind::ha18d8f0ab15c4858
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/panic.rs:359:14
  32:     0x55671f81b8b0 - std::rt::lang_start_internal::h31bbb7f936fd6b5d
                               at /rustc/29483883eed69d5fb4db01964cdf2af4d86e9cb2/library/std/src/rt.rs:171:5
  33:     0x55671f8035a7 - std::rt::lang_start::hd188080df7d1abc2
                               at /home/daybreak/.rustup/toolchains/stable-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/rt.rs:205:5
  34:     0x55671f80321e - main
  35:     0x7f3355529575 - __libc_start_call_main
  36:     0x7f3355529628 - __libc_start_main_alias_2
  37:     0x55671f802ae5 - _start
  38:                0x0 - <unknown>
```


### Recoverable Errors with `Result` enum:

Enum `Result<T, E>` has two variants `Ok` and `Err`:

```rust 

    enum Result<T, E> {
        Ok(T),
        Err(E),
    }
```
T, E are generic type parameters. ( more on generics in other topic )

Here `T` is the value that is returned in success case.
and `E` is the type of error that will be returned in a failure case within the `Err` variant. 


Example:

```rust 
    use std::fs::File;
    fn main() {
        let greeting_file_result = File::open("hello.txt");

        let greeting_file = match greeting_file_result {
            Ok(file) => file,
            Err(error) => panic!("Problem opening the file: {error:?}"),
        };
    }
```
Note: Like Option enum the Result enum is a part of rust prelude and does not require to call Result::Ok and
Result::Err in the match arm.

In the above program when the file does not exit we prefer to terminate the program by calling panic!()
macro.

### matching on different errors:

If we want the code to panic! in the same way as in above program but want to handle other error cases:
we can use the `match error.kind()`

When handling errors with Result<T,E> we can use `match` and calling `.kind()` method on the error to make
it more useful.  ( mainly with I/O operations )

#### Context: `std::io::Error` and `error.kind()`:

When working with I/O functions ( like File::open, read, write...) the functions usually return:

```
    Result<T, std::io::Error> 
```
`std::io::Error` type has a method called `.kind()` that returns `ErrorKind` enum which classified the error
into broad categories ( like NotFound, PermissionDenied ....)


Ex:
```rust 
    use std::fs::File;
    use std::io::{self, ErrorKind};

    fn main() {
        let result = File::open("some_file.txt");

        match result {
            Ok(file) => {
                println!("File opened successfully: {:?}", file);
            }
            Err(error) => {
                match error.kind() {
                    ErrorKind::NotFound => {
                        println!("File not found. Maybe create it?");
                        // You could create the file here instead of panicking
                    }
                    ErrorKind::PermissionDenied => {
                        println!("Permission denied when opening the file.");
                    }
                    _ => {
                        println!("Some other error occurred: {}", error);
                    }
                }
            }
        }
    }
```

The above code has a lot of `match`. This method can be made more concise using than using `match` and
`Result<T,E>`  by using **closures**

Using `unwrap_or_else` and closures:

```rust 
use std::fs::File;
use std::io::{self, ErrorKind};

fn main() {
    let _file = File::open("some_file.txt").unwrap_or_else(|error| {
        match error.kind() {
            ErrorKind::NotFound => {
                println!("File not found. Consider creating it.");
                // Optionally create the file here
                std::process::exit(1);
            }
            ErrorKind::PermissionDenied => {
                println!("Permission denied.");
                std::process::exit(1);
            }
            _ => {
                println!("Unexpected error: {}", error);
                std::process::exit(1);
            }
        }
    });
}
```

this prevents the use or nested matches code gets bit cleaner and concise.
`unwrap_or_else` allows concise, inline error handling with a closure. 

This can also be more abstract by extracting to a function as below:

```rust 
fn handle_file_error(error: std::io::Error) -> ! {
    use ErrorKind::*;

    match error.kind() {
        NotFound => eprintln!("File not found."),
        PermissionDenied => eprintln!("Permission denied."),
        _ => eprintln!("Other error: {}", error),
    }
    std::process::exit(1);
}

fn main() {
    let _file = File::open("some_file.txt").unwrap_or_else(handle_file_error);
}
```



#### Shortcuts for Panic on Error: unwrap and expect:

The `Result<T, E>` type has many helper methods defined on it to do various, more specific tasks. 
The `unwrap` method is a shortcut method implemented just like the match expression.

If the `Result` value is the `Ok` variant, `unwrap` will return the value inside the `Ok`. 
If the `Result` is the `Err` variant, `unwrap` will call the `panic!` macro for us. 

Example using `unwrap`

```rust 
    use std::fs::File;

    fn main () {
        let greeting_file = File::open("hello.txt").unwrap();
    }
```
If we run the above code we see panic! call that the `unwrap` method makes:


Similarly the `expect` method lets us choose the `panic!` error message. Using `expect` instead of `unwrap`
and providing good error messages can covey your intent and make tracking down the source of panic easier:

Syntax and using `expect`:

```rust 
    use std::fs::File;
    fn main () {
        let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this progject!");
    }
```

#### Propagating Errors:

Propagating errors using `Result` gives the calling code the power to decide what to do, rather then making
that decision too early inside a helper function.

If function encounters error and *immediately panics or logs it* it removes control from the caller.
But instead if it returns `Result<T,E>` the calling function get more options:
to do  Retry, or Log the error, or convert it to another error type or ignore it or panic! 

This Propagation of error = Gives higher flexibility to higher-level logic.
example:

```rust
    use std::fs::File;
    use std::io::{self, Read};

    fn read_username_from_file() -> Result<String, io::Error> {
        let username_file_result = File::open("hello.txt");

        let mut username_file = match username_file_result {
            Ok(file) => file,
            Err(e) => return Err(e),
        };

        let mut username = String::new();

        match username_file.read_to_string(&mut username) {
            Ok(_) => Ok(username),
            Err(e) => Err(e),
        }
    }
```
- The fun returns `Result<String, io::Error>` 
- If the fun succeeds it returns `Ok` value that holds `String` 
- If the fun encounters any problems, the calling code will receive an `Err` which hold `io::Error` which
  contains more info about what the problem was. Note that both opening file and reading username from file
  can fail with Err. Leaving it to the calling code to decide by passing enough information **upwards**. 

=> This propagation is common in Rust that Rust provides the question mark operator `?` to make it easier.
The ? placed after a `Result` value is defined to work in almost the same way as the `match` expressions we 
defined to handle the Result value.

There is a difference between what the `match` expression used in above code does and what the `?` operator
does: error values that have the `?` operator called on them go through the `from` function, defined in the 
`From` trait in the std library, which is used to convert values from one type into another. 
When the `?` operator calls the from function, the error type received is converted into the error type 
defined in the return type of the current function. 

This is useful when a function returns one error type to represent all the ways a function might fail, 
even if parts might fail for many different reasons

Ex: 
Step 1: Helper function that propagates error:
```rust 
    use std::fs::File;
    use std::io::{self, Read};

    fn read_file_contents(path: &str) -> Result<String, io::Error> {
        let mut file = File::open(path)?; // propagates error up
        let mut contents = String::new();
        file.read_to_string(&mut contents)?; // propagates again
        Ok(contents)
    }
```
- the `?` operator removes a lot of boilerplate code and makes this function implementation simpler.

- The ? operator can only be used in functions whose return type is compatible with the value the ? is used 
  on. This is because the ? operator is defined to perform an early return of a value out of the function, 
  in the same manner as the match expression 

- Uses `?` operator to *propagate* any `io::Error`

- Function does not decide what to do if the file does not exist or is unreadable.

Step 2: Let the calling code decide:
```rust 
fn main() {
    match read_file_contents("data.txt") {
        Ok(contents) => {
            println!("File contents:\n{}", contents);
        }
        Err(e) => {
            if e.kind() == std::io::ErrorKind::NotFound {
                println!("The file was not found. Do you want to create it?");
                // maybe create the file here
            } else {
                eprintln!("Failed to read file: {}", e);
            }
        }
    }
}
```
- Separation of concerns: The `read_file_contents` fun only handles reading — no assumptions about error
  policy.

- Reusability: You can reuse the function in different contexts with different error-handling needs.

- Composability: You can chain this with other Result-returning functions easily.



#### More Idiomatic Example with ?

You can propagate the error all the way up to main:
Example

```rust 
    fn read_file_contents(path: &str) -> Result<String, std::io::Error> {
        let mut contents = String::new();
        File::open(path)?.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn main() -> Result<(), std::io::Error> {
        let contents = read_file_contents("data.txt")?;
        println!("File:\n{}", contents);
        Ok(())
    }
```
- Uses `?` throughout.
- Even main returns a `Result` so we can avoid explicit `match` unless needed.
- In production you'd typically still handle errors in `main` but this is elegant for smaller apps or
  tools.

Example:
```rust 
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
```

#### Where the `?` Operator can be used:

`?` can only be used in function whose return type is compatible with the value the ? is used on. This is
because `?` operator is defined to perform an early return of a value out of the function, in the same
manner as the `match` expression we defined in the above examples. The `match` was using a `Result` value,
and the early return arm returned an Err(e). The return type of the function has to be a `Result` so that
it's compatible with this `return`.

Say we want to use `?` as in the below example:

```rust 
use std::file::File;
fn main () {
    let greeting_file = File::open("hello.txt")?;
}
```

This will fail to compile as the function main returns nothing and `?` operator is used to propagate the
error that returns a `Result<T,E>` (or `Option<T>`) but your main function returns nothing.

==> `?` operator can only be used in functions that return `Result` or (`Option`) because it implicitly
returns an error if one occurs. 

To Fix these there are 2 choices:
1. Choice the return tyoe of your function compatible to the value you're using the `?` operator on as long
   as you have no restrictions preventing that. The other choice is to use a `match` of one the
   `Result<T,E>` method to handle  the `Result<T,E>`.

   If the `?` returns `Option` then the function should return Option and `?` should not be mis matched with
   Result and Options ....

2. we can change the return type of `main` to be `Result<(), Box<dyn Error>>` and add return value `Ok(())`
   to the end. 

```rust 
    use std::error::Error;
    use std::fs::File;

    fn main() -> Result<(), Box<dyn Error>> {
        let greeting_file = File::open("hello.txt")?;

        Ok(())
    }
```

- The Box<dyn Error> type is a trait object. ( another article on this )
- you can read Box<dyn Error> to mean “any kind of error.” 
- Using `?` on a Result value in a `main` function with the error type `Box<dyn Error>` is allowed because
  it allows any `Err` value to be returned early. Even though the body of this main function will only 
  ever return errors of type `std::io::Error`, by specifying `Box<dyn Error>`, this signature will continue 
  to be correct even if more code that returns other errors is added to the body of main.

### To panic! or not to panic!

- `panic!` is for *unrecoverable errors* ( where there is no reasonable way for program to continue)

- Returning `Return<T,E>` is for recoverable error. Places where failure might occur and the caller may want
  to handle it. 

- Deciding between `panic!` vs returning `Result` is about making judement explicitly in your API.

- Prefer `Result` by default, use panic! only when the error is truly unrecoverable or indicates a bug. 

- `panic!` when continuing would cause worse problems or internal consistency is violated.
