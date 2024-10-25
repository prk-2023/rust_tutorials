# Mutability

1. **Rust Mutability**
=====================

-  mutability is a fundamental concept that determines whether a value can be changed after it's created.
   Rust has a unique approach to mutability, which is designed to prevent bugs and ensure memory safety.

2. **Immutable by Default**
------------------------

- All values are immutable by default. This means that once a value is created, it cannot be changed.
  This is in contrast to languages like C or C++, where values are mutable by default.

3. **Mutability with `mut`**
-------------------------

- To make a value mutable, you need to use the `mut` keyword when declaring it. Here's an example:

    ```rust
        let x = 5; // immutable
        let mut y = 5; // mutable
        y = 10; // okay
        x = 10; // error: cannot assign twice to immutable variable `x`
    ```

4. **Reference Mutability**
-------------------------

- In Rust, references can also be mutable or immutable. 
- A mutable reference allows you to modify the value it points to, while an immutable reference only allows
  you to read the value.

    ```rust
        let mut x = 5;
        let mut y = &mut x; // mutable reference
        *y = 10; // okay
        let z = &x; // immutable reference
        *z = 10; // error: cannot assign to `*z` which is behind a `&` reference
    ```

5. **Borrow Checker**
------------------

- Rust's borrow checker is a key component of its mutability system. 
- The borrow checker ensures that:
    * You can't have multiple mutable references to the same value at the same time.
    * You can't have a mutable reference and an immutable reference to the same value at the same time.

    example:
    ```rust
        let mut x = 5;
        let y = &mut x; // mutable reference
        let z = &x; // error: cannot borrow `x` as immutable because it is also borrowed as mutable
    ```

6. **Interior Mutability**
-------------------------

- Rust provides several ways to achieve interior mutability, which allows you to mutate a value even if it's
  not declared as mutable. Some examples include:

  * `Cell` and `RefCell`:
    These types provide a way to mutate a value inside a struct or enum, even if the struct or enum is not 
    declared as mutable.

  * `Mutex` and `RwLock`: These types provide a way to mutate a value in a thread-safe way.

  Example using `RefCell`:

  ```rust 
    use std::cell::RefCell;
    struct Person {
        name: RefCell<String>,
    }
    fn main() {
        let person = Person {
            name: RefCell::new("John".to_string()),
        };
        person.name.borrow_mut().push_str(" Doe");
        println!("{}", person.name.borrow()); // prints "John Doe"
    }
    ```

7. Field-level mutability:

- Mutability is the property of either a borrow (&mut) or binding (let mut).
  i.e for ex you cannot have a `struct` with some fields as mutable and some as immutable:

  struct Point {
    x: i32,
    mut y: i32,   // Nope
  }

  => mutability of `struct` is in its binding

  struct Point {
    x: i32,
    y: i32,   
  }
  let mut a = Point {x:5, y:2};
  a.x = 10;
  let b = Point {x:5, y:2};
  b.x = 10; // Error: cannot assign to immutable field b.x

  Note: using `Cell<T>` you can emulate field-level mutability:

    #![allow(unused_variables)]
    fn main() {
        use std::cell::Cell;
        struct Point {
            x: i32,
            y: Cell<i32>,
        }
        let point = Point { x: 5, y: Cell::new(6) };
        point.y.set(7);
        println!("y: {:?}", point.y);
    }
    This will print y: Cell { Value = 7} ==> we have successfully updated y. 

8. **Best Practices**
------------------

- To keep in mind when working with mutability in Rust:
    * Use `mut` sparingly: 
        Only use `mut` when you need to mutate a value. 
        This helps prevent bugs and makes your code easier to reason about.
    * Use references instead of mutable values: Instead of declaring a value as mutable, consider using a
      reference to the value. This can help prevent bugs and make your code more flexible.
    *   Use interior mutability carefully: Interior mutability can be useful, but it can also make your code
        harder to reason about. Use it sparingly and only when necessary.
