# Mutability

1. **Rust Mutability**
---

-  mutability is a fundamental concept that determines whether a value can be changed after it's created.
   Rust has a unique approach to mutability, which is designed to prevent bugs and ensure memory safety.

2. **Immutable by Default**
---

- All values are immutable by default. This means that once a value is created, it cannot be changed.
  This is in contrast to languages like C or C++, where values are mutable by default.

- Immutable reference (&T): When we pass a reference like `&x` to a function, you are passing  an immutable 
  reference to `x`. This means that some_function can borrow the value of `x` without taking ownership of it.

  Since You are passing `&x` the address to some function, `x` remains valid (i.e not moved or invalidated)
  after the function call, and the function can access its value without being able to modify it.

3. **Mutability with `mut`**
---

- To make a value mutable, you need to use the `mut` keyword when declaring it. Here's an example:

    ```rust
        let x = 5; // immutable
        let mut y = 5; // mutable
        y = 10; // okay
        x = 10; // error: cannot assign twice to immutable variable `x`
    ```

4. **Reference Mutability**
---

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
- While a **mutable referece** allows a function to modify the value of `x`. However, mutable reference come
  with strict rules: 
  * You can have only **One mutable reference** to a piece of data at a time ( no aliasing ).
  * You can not have a **mutable reference and an immutable reference to the same data at the same type. 

- Borrow Checker : Rust compiler performs Borrow checking which checks to ensure that you cannot access
  invalid or deallocated data, preventing use-after-free errors.

5. **Borrow Checker**
---

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
---

- Rust provides several ways to achieve interior mutability, which allows you to mutate a value even if it's
  not declared as mutable. Some examples include:

  * `Cell` and `RefCell`:
    These types provide a way to mutate a value inside a struct or enum, even if the struct or enum is not 
    declared as mutable.

[Note: Cell, RefCell: these are two types of smart pointers that allow you to mutate data in a way that's
safe and controlled. Both are part of the rust standard library "std" -> use std::cell::{Cell, RefCell};

    1. Cell: A type of smart pointer that allows to mutate its contents even if it's shared (i.e behind &
       ref ). 
       "Cell" can only be used with those types that implement "Copy" trait because it uses a technique
       called "Copy-on-Write" to update its contents. 

    2. RefCell: Also another type of smart pointer that allows to mutate its contents even if its shared.
       The difference being that RefCell can be used with any type not just those implement Copy trait.
       RefCell's will panic if you try to borrow its contents mutably while its already borrowed.
]
  * `Mutex` and `RwLock`: These types provide a way to mutate a value in a thread-safe way.

  Example using `Cell`  and `RefCell`:
  ```rust 
     use std::cell::{Cell, RefCell};
     fn main () {
         // Using Cell 
         let c = Cell::new(5);
         let c_clone = c.clone(); // Clone the Cell
         c.set(10); // Update the contents of the Cell
         println!("Cell contents: {}", c_clone.get()); // prints 10

         // Using RefCell
         let r = RefCell::new(String::from("Hello"));
         let r_borrow = r.borrow(); // Borrow the contents of the RefCell
         println!("RefCell contents: {}", r_borrow); // prints "Hello"
         // r.borrow_mut().push_str(" World"); //-> panics because we're already borrowing the contents
         drop(r_borrow); // Drop the borrow
         r.borrow_mut().push_str(" World"); // Now we can mutate the contents
         println!("RefCell contents: {}", r.borrow()); // prints "Hello World"
     }
  ```
  Ex 2:

  ```rust 
     use std::cell::{Cell,RefCell};
     struct SomeStruct {
        regular_field: u8,
        special_field: Cell<u8>,
     }
     struct Person {
         name: RefCell<String>,
     }
     fn main() {
        //Cell:
        let my_struct = SomeStruct {
            regular_field: 0,
            special_field: Cell::new(1),
        };
        let new_value = 100;
        // ERROR: `my_struct` is immutable
        // my_struct.regular_field = new_value;

        // WORKS: although `my_struct` is immutable, `special_field` is a `Cell`,
        // which can always be mutated
        my_struct.special_field.set(new_value);
        assert_eq!(my_struct.special_field.get(), new_value);

        // ReCell:
        let c = RefCell::new(5); {
            let m = c.borrow_mut();
            assert!(c.try_borrow().is_err());
        }
        {
            let m = c.borrow();
            assert!(c.try_borrow().is_ok());
        }
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
---

- To keep in mind when working with mutability in Rust:
    * Use `mut` sparingly: 
        Only use `mut` when you need to mutate a value. 
        This helps prevent bugs and makes your code easier to reason about.
    * Use references instead of mutable values: Instead of declaring a value as mutable, consider using a
      reference to the value. This can help prevent bugs and make your code more flexible.
    *   Use interior mutability carefully: Interior mutability can be useful, but it can also make your code
        harder to reason about. Use it sparingly and only when necessary.
    * In general with `Cell` and `RefCell`, 
        - should prefer `Cell` when you need to update a value that implements `Copy`, and 
        - `RefCell` when you need to update a value that doesn't implement `Copy`.

---

# Mutability:

## In Rust "mutability is about the binding, not the type"

* Types (like `struct`, `enum`, or `i32`) are not inherently mutable or immutable.
* What matters is whether the **variable (binding)** that refers to the value is declared as `mut`.

---

### 1. **Binding Mutability**

Consider:

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let mut p = Point { x: 1, y: 2 }; // <-- mutable binding
    p.x = 5; // OK: p is mutable
}
```

* `Point` is just a type. It doesn’t "know" or "care" whether it's mutable.
* `p` is a **binding to a value of type `Point`**.
* Because the binding is declared with `mut`, you can mutate the fields.

---

### 2. **Immutable Binding Example**

Now compare this:

```rust
fn main() {
    let p = Point { x: 1, y: 2 }; // <-- immutable binding
    p.x = 5; // ❌ Error: cannot assign to `p.x`, as `p` is not declared as mutable
}
```
* Same exact `Point` type and value.
* The only difference is that `p` is not declared as `mut`.
* Result: Rust will prevent you from modifying it.

Again: **the type didn't change**, just the **binding**.

---

### 3. **Same Concept with `enum`**

Here's an `enum` example:

```rust
enum State {
    Running(i32),
    Paused,
}

fn main() {
    let mut state = State::Running(10); // mutable binding

    // Reassign the whole enum:
    state = State::Paused; // ✅ OK
}
```

You can change the whole variant because `state` is `mut`.

But if you don’t do `let mut state`, you'll get a compiler error:

```rust
fn main() {
    let state = State::Running(10); // immutable binding
    state = State::Paused; // ❌ Error: cannot assign, state is not mutable
}
```

---

## Summary

* In Rust, **mutability is not a property of the type**, but of the **binding** to the value.
* `let mut x = ...` means “the variable `x` can be changed”.
* Even if a type has mutable fields or complex data, **you can't mutate them unless the binding is mutable**.

---

## Bonus: Field-Level Mutability

Rust does **not** allow partial field mutability like some languages.

```rust
struct User {
    name: String,
    age: u32,
}

fn main() {
    let mut user = User {
        name: "Alice".to_string(),
        age: 30,
    };

    user.age = 31; // ✅ OK

    // But if 'user' is not declared as 'mut', this would not compile
}
```

---

# Interior mutability

---

## What is *Interior Mutability*?

In Rust, **interior mutability** allows you to **mutate data even when the outer binding is immutable**.

This might sound like it breaks Rust’s rules, but it's safe because it uses **runtime checks** instead of compile-time guarantees. You typically use this when:

* You need to mutate data from multiple places.
* You can't (or don’t want to) make the outer binding `mut`.
* You need shared ownership (e.g., inside `Rc`).

---

## Tools for Interior Mutability

The two most commonly used types are:

### 1. `Cell<T>` — for `Copy` types (like `i32`, `bool`, etc.)

```rust
use std::cell::Cell;

struct Counter {
    value: Cell<i32>,
}

fn main() {
    let counter = Counter {
        value: Cell::new(0),
    };

    // Even though 'counter' is not mutable, we can still mutate the value inside
    counter.value.set(counter.value.get() + 1);

    println!("Counter = {}", counter.value.get()); // Output: 1
}
```

✅ `Cell` is good when:

* The data is small and `Copy`
* You don’t need references to the inner data

---

### 2. `RefCell<T>` — for any type (including non-`Copy`)

```rust
use std::cell::RefCell;

struct Logger {
    messages: RefCell<Vec<String>>,
}

fn main() {
    let logger = Logger {
        messages: RefCell::new(Vec::new()),
    };

    // Mutate the vector through a shared reference
    logger.messages.borrow_mut().push("App started".to_string());

    println!("Logs: {:?}", logger.messages.borrow());
}
```

✅ `RefCell` is more powerful:

* It allows borrowing mutable or immutable references at runtime.
* But it enforces **borrowing rules at runtime** — not at compile time.

---

## Important: Runtime Borrowing Rules

* `RefCell` checks the **borrow rules** at **runtime**, and will `panic!` if violated.

```rust
let r = RefCell::new(vec![1, 2, 3]);

let borrow1 = r.borrow();
let borrow2 = r.borrow_mut(); // ❌ Runtime panic: already borrowed immutably
```

So, be careful and avoid overlapping borrows.

---

## Real-World Example: Interior Mutability + Shared Ownership

If you're using `Rc<T>` (reference-counted shared ownership), you **must** use interior mutability, since you can’t have multiple `&mut` references.

```rust
use std::rc::Rc;
use std::cell::RefCell;

struct SharedCounter {
    count: Rc<RefCell<i32>>,
}

fn main() {
    let counter = Rc::new(RefCell::new(0));

    let a = SharedCounter { count: Rc::clone(&counter) };
    let b = SharedCounter { count: Rc::clone(&counter) };

    a.count.borrow_mut().add_assign(1);
    b.count.borrow_mut().add_assign(2);

    println!("Count: {}", counter.borrow()); // Output: 3
}
```

---

## Summary Table

| Tool         | Use Case                             | Compile-Time Checked? | Copy Types Only? |
| ------------ | ------------------------------------ | --------------------- | ---------------- |
| `mut`        | Regular mutable access               | ✅ Yes                 | ❌ No             |
| `Cell<T>`    | Interior mutability for `Copy` types | ❌ No (runtime only)   | ✅ Yes            |
| `RefCell<T>` | Interior mutability for any type     | ❌ No (runtime only)   | ❌ No             |

---

##  When to Use Interior Mutability

Use **interior mutability** when:

* You can't use `mut` (e.g., due to ownership constraints).
* You’re using `Rc<T>` or building shared data structures.
* You need to mutate from inside an immutable context (like caching, lazy initialization, etc.).

---

Let me know if you'd like to go through a real example like:

* building a shared cache,
* creating an event logger with `RefCell`,
* or how this works in multi-threaded contexts (`Mutex`, `RwLock`).


----


**first example**: **building a shared cache** using interior mutability.

---

## Example 1: Shared Cache with `RefCell` and `Rc`

### Goal:

We'll build a **simple in-memory cache** where multiple parts of your code can read and update shared cached data.

This is a classic use case for:

* `Rc<T>` → for **shared ownership**
* `RefCell<T>` → for **interior mutability**

---

### Code: Shared Cache Example

```rust
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

type SharedCache = Rc<RefCell<HashMap<String, String>>>;

struct CacheReader {
    cache: SharedCache,
}

struct CacheWriter {
    cache: SharedCache,
}

fn main() {
    let shared_cache: SharedCache = Rc::new(RefCell::new(HashMap::new()));

    let reader = CacheReader {
        cache: Rc::clone(&shared_cache),
    };

    let writer = CacheWriter {
        cache: Rc::clone(&shared_cache),
    };

    // Writer updates the cache
    writer.cache.borrow_mut().insert("user_1".to_string(), "Alice".to_string());

    // Reader accesses the cache
    if let Some(name) = reader.cache.borrow().get("user_1") {
        println!("Reader found user_1: {}", name); // Output: Alice
    }
}
```

---

### What This Shows:

* `RefCell` allows **mutable access to the cache** even though the struct holding it (`CacheReader`, `CacheWriter`) doesn't require `mut`.
* `Rc` allows multiple components to **share ownership** of the same cache.
* No compile-time mutability needed — all mutation happens **internally**.

---

### Reminder:

If you accidentally try to call `borrow()` and `borrow_mut()` at the same time, the program **will panic** at runtime. So handle carefully.

---

Would you like to:

1. Modify this cache example? (e.g., add a `CacheManager`, or add logging?)
2. Move on to the second example: **event logger with `RefCell`**?
3. Ask questions about this example?

Let me know!

---

let’s **enhance the cache example** by adding a `CacheManager` and some basic logging. This will make the architecture cleaner and show how `RefCell` and `Rc` help structure shared, mutable state.

---

## What We're Adding:

* A `CacheManager` struct that owns the shared cache.
* Methods on `CacheManager` to **read**, **write**, and **log** operations.
* Logging output printed to show cache usage.

---

## Modified Code: CacheManager with Logging

```rust
use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;

type SharedCache = Rc<RefCell<HashMap<String, String>>>;

struct CacheManager {
    cache: SharedCache,
}

impl CacheManager {
    fn new() -> Self {
        CacheManager {
            cache: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    fn get_reader(&self) -> CacheReader {
        CacheReader {
            cache: Rc::clone(&self.cache),
        }
    }

    fn get_writer(&self) -> CacheWriter {
        CacheWriter {
            cache: Rc::clone(&self.cache),
        }
    }

    fn log_cache_state(&self) {
        let cache = self.cache.borrow();
        println!("📦 Current Cache State:");
        for (key, value) in cache.iter() {
            println!(" - {}: {}", key, value);
        }
        if cache.is_empty() {
            println!(" (Cache is empty)");
        }
    }
}

struct CacheReader {
    cache: SharedCache,
}

impl CacheReader {
    fn get(&self, key: &str) -> Option<String> {
        let cache = self.cache.borrow();
        cache.get(key).cloned()
    }
}

struct CacheWriter {
    cache: SharedCache,
}

impl CacheWriter {
    fn insert(&self, key: &str, value: &str) {
        let mut cache = self.cache.borrow_mut();
        println!("✏️ Writing to cache: {} → {}", key, value);
        cache.insert(key.to_string(), value.to_string());
    }

    fn remove(&self, key: &str) {
        let mut cache = self.cache.borrow_mut();
        println!("🗑️ Removing from cache: {}", key);
        cache.remove(key);
    }
}

fn main() {
    let manager = CacheManager::new();

    let reader = manager.get_reader();
    let writer = manager.get_writer();

    manager.log_cache_state(); // Cache is empty

    writer.insert("user_1", "Alice");
    writer.insert("user_2", "Bob");

    manager.log_cache_state(); // Shows two entries

    if let Some(name) = reader.get("user_1") {
        println!("🔍 Reader found user_1: {}", name);
    } else {
        println!("❌ user_1 not found");
    }

    writer.remove("user_1");

    manager.log_cache_state(); // Shows user_2 only
}
```

---

## What's Improved?

| Feature                  | Benefit                                        |
| ------------------------ | ---------------------------------------------- |
| `CacheManager`           | Centralizes cache ownership and logic          |
| Logging                  | Helps visualize internal changes               |
| Reader/Writer separation | Mimics real-world read/write access in systems |
| `Rc<RefCell<_>>`         | Still enables shared, mutable access safely    |

---

Would you like to:

1. **Add persistence** (e.g., simulate saving the cache to disk)?
2. Convert this to be **thread-safe** using `Arc<Mutex<>>`?
3. Move to **Example 2: Event Logger with `RefCell`**?
4. Ask something else?

Let me know!
