# Rust modules ( Pattern matching )

Rust provides several module patterns to organize and structure code. 

An overview of the 
    different module patterns available, 
    module pattern matching, and 
    module pattern aliases.

## **Module Patterns**

Rust provides the following module patterns:

1. **Simple Module Pattern**: The most basic module pattern, where a module is defined using the 
`mod` keyword followed by the module name.

    ```rust
        mod my_module {
            // module contents
        } 
    ```

2. **Nested Module Pattern**: This pattern allows you to define modules inside other modules.

    ```rust 
        mod my_module {
            mod my_submodule {
                // submodule contents
            }
        }
    ```

3. **Module File Pattern**: In this pattern, a module is defined in a separate file with the same name as 
the module.

    ```rust
        // my_module.rs
        // module contents
    ```

    ```rust
        // main.rs
        mod my_module;
    ```

4. **Module Directory Pattern**: This pattern allows you to define a module in a directory with the same 
name as the module. The directory should contain a `mod.rs` file that defines the module.

    ```rust
    // my_module/mod.rs
    // module contents
    ```

    ```rust 
        // main.rs
        mod my_module;
    ```

**Module Pattern Matching**

Module pattern matching is used to import modules from other modules. 

Rust provides the following module pattern matching syntax:

* `mod module_name`: Matches a module with the given name.
* `mod module_name { ... }`: Matches a module with the given name and imports its contents.
* `pub mod module_name`: Matches a module with the given name and imports its contents, 
                         making them publicly visible.
* `use module_name`: Imports the module with the given name and its contents.
* `use module_name::{item1, item2, ...}`: Imports specific items from the module with the given name.

Here's an example of module pattern matching:

```rust
// my_module.rs
pub mod my_submodule {
    pub fn my_function() {
        println!("Hello from my_function!");
    }
}
```

```rust
// main.rs
mod my_module;

fn main() {
    my_module::my_submodule::my_function(); // Error: my_submodule is not publicly visible
}
```

To fix the error, you can use the `pub use` statement to re-export the `my_submodule` module:

```rust
// my_module.rs
pub mod my_submodule {
    pub fn my_function() {
        println!("Hello from my_function!");
    }
}

pub use self::my_submodule;
```

```rust
// main.rs
mod my_module;

fn main() {
    my_module::my_submodule::my_function(); // Okay
}
```

**Module Pattern Aliases**

Module pattern aliases are used to refer to modules with a different name. 

Rust provides the following module pattern alias syntax:

* `mod module_name as alias`: Creates an alias for the module with the given name.
* `use module_name as alias`: Imports the module with the given name and assigns it an alias.

Here's an example of module pattern aliases:

```rust
// my_module.rs
pub mod my_submodule {
    pub fn my_function() {
        println!("Hello from my_function!");
    }
}
```

```rust
// main.rs
mod my_module;

use my_module::my_submodule as my_alias;

fn main() {
    my_alias::my_function(); // Okay
}
```

In this example, `my_alias` is an alias for the `my_submodule` module. 
You can use `my_alias` to access the `my_function` function instead of using the 
full path `my_module::my_submodule::my_function`.

---
example of a Rust project with modules organized into sub-folders:

**Project Structure:**

```markdown
    my_project/
        Cargo.toml
        src/
            main.rs
            lib.rs
        math/
            mod.rs 
            add.rs 
            subtract.rs 
            multiply.rs 
            divide.rs 
        utils/
            mod.rs 
            logger.rs 
            config.rs
```

**Cargo.toml:**
    ```toml
        [package]
        name = "my_project"
        version = "0.1.0"
        edition = "2021"

        [lib]
        path = "src/lib.rs"

        [[bin]]
        name = "my_project"
        path = "src/main.rs"
    ```

**src/main.rs:**
    ```rust
        mod math;
        mod utils;

        fn main() {
            let result = math::add(2, 3);
            println!("Result: {}", result);

            utils::logger::log("Hello, world!");
        }
    ```

**src/lib.rs:**
    ```rust
        pub mod math;
        pub mod utils;
    ```

**src/math/mod.rs:**
    ```rust
        pub mod add;
        pub mod subtract;
        pub mod multiply;
        pub mod divide;
    ```

**src/math/add.rs:**
    ```rust
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }
    ```

**src/math/subtract.rs:**
    ```rust
        pub fn subtract(a: i32, b: i32) -> i32 {
            a - b
        }
    ```

**src/math/multiply.rs:**
    ```rust
        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }
    ```

**src/math/divide.rs:**
    ```rust
        pub fn divide(a: i32, b: i32) -> i32 {
            a / b
        }
    ```

**src/utils/mod.rs:**
    ```rust
        pub mod logger;
        pub mod config;
    ```

**src/utils/logger.rs:**
    ```rust
        pub fn log(message: &str) {
        println!("{}", message);
       }
    ```

**src/utils/config.rs:**
    ```rust
        pub struct Config {
            pub debug: bool,
        }

        impl Config {
            pub fn new(debug: bool) -> Self {
                Config { debug }
            }
        }
    ```

The example, we have a Rust project with two main modules: `math` and `utils`. 

The `math` module is further divided into four sub-modules: 
    `add`, `subtract`, `multiply`, and `divide`. 

The `utils` module is also divided into two sub-modules: `logger` and `config`.

The `main.rs` file imports the `math` and `utils` modules and uses their functions. 

The `lib.rs` file exports the `math` and `utils` modules, making them available to other crates.

Note that in Rust, the `mod` keyword is used to declare a module, and the `pub` keyword is used 
to make a module or function publicly visible. 

The `mod.rs` file is used to declare the sub-modules of a module, 
and the **`pub mod`** keyword is used to make the sub-modules publicly visible.


