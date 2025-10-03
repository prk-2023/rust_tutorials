# Traits ( Manual Implementation )


Implementing common Rust traits manually is a fantastic way to deeply understand Rust’s type system,
`ownership`, `borrowing`, and `idiomatic patterns`.

Here’s a list of **useful and commonly implemented traits** that you can try manually while learning Rust:

---

## Common Rust Traits to Implement Manually for Learning

### 1. **`Debug`**

* Purpose: Format your type using the `{:?}` formatter.
* Learning: Understand how to provide meaningful debugging output.

### 2. **`Display`**

* Purpose: Format your type for user-facing output (`{}`).
* Learning: Practice custom string formatting and `fmt::Formatter`.

### 3. **`PartialEq` & `Eq`**

* Purpose: Enable comparison of equality (`==` and `!=`).
* Learning: Understand equality semantics and when types can be fully equatable.

### 4. **`PartialOrd` & `Ord`**

* Purpose: Enable ordering (`<`, `>`, `<=`, `>=`).
* Learning: Learn how to define custom sorting and total order relations.

### 5. **`Clone`**

* Purpose: Provide explicit duplication of objects.
* Learning: Understand deep vs shallow copying and ownership.

### 6. **`Copy`**

* Purpose: Make a type copyable by simple bitwise copy.
* Learning: Learn traits bounds for simple types and the difference between `Clone` and `Copy`.

### 7. **`Iterator`**

* Purpose: Create your own iterator for a custom collection or structure.
* Learning: Deep dive into iterator protocols, lazy evaluation, and state management.

### 8. **`From` & `Into`**

* Purpose: Conversion between types.
* Learning: Understand safe and idiomatic conversions in Rust.

### 9. **`Default`**

* Purpose: Provide a default value for your type.
* Learning: Practice defining sensible defaults.

### 10. **`Drop`**

* Purpose: Cleanup when a value goes out of scope.
* Learning: Understand Rust’s ownership and RAII principles.

---

## Bonus: More Advanced or Niche Traits

* **`Deref` & `DerefMut`** — for smart pointer-like types.
* **`Hash`** — make your type usable in hash maps/sets.
* **`AsRef` & `AsMut`** — cheap references conversion.
* **`Borrow`** — customize borrowing for collections.
* **`Fn`, `FnMut`, `FnOnce`** — implement callable traits for fun (advanced).

---

## Why Implement Traits Manually?

* Deep understanding of Rust’s trait system.
* Learn idiomatic patterns in Rust code.
* Prepare for reading and writing complex crates.
* Practice ownership, borrowing, lifetimes through trait methods.

---
Note: std::fmt : 
----------------------------------------------------------------------------------------------------------
    `std::fmt` Module is the *formatting and printing machinery of the Rust std lib.
    Its main role is to control how values are converted into "string" for output, whether for `Display` (
    user-facing) and `Debug` ( programmer-facing).

    Key components of `std::fmt`:
    Module defines the traits and structures used for controlled string output:

1. Formatting Traits:
    These are public traits that you implement for your custome types:

    * `fmt::Display`: used to generate clean, user-faceing output, implemented for the basic `{}` format
      specifier.

    * `fmt::Debug`: Used for developer facing output for debugging. (ex: `{:?}` and `{:#?}` format specifier)

    * *Other formatting traits*: The module also provides traits for specific numeric formatting, such as
      Binary ({:b}), 
      Octal ({:o}), 
      LowerHex ({:x}), 
      UpperHex ({:X}), and 
      Pointer ({:p}).

    2. Formatter: The core mechanism used inside any manual `fmt` implementation:

    * `fmt::Formatter`: this is mutable struct (&mut fmt::Formatter ) passed into the `fmt` method of the
      formatting traits. It acts as the destination buffer and holds all the formatting options (like
      padding, alignment, precision...), specified within the format string.

    3. Formatting Macros: These macros are used to write data into the `Formatter`:

    * *`write! ()`*: The low-level macro used inside the `fmt` method. It takes `Formatter` and its first
      argument and write the formatted string into it. It returns a fmt::Result (success or a potential I/O
      error ).

      ```
        // Used inside impl Display for T 
        write!(f, "My value is: {}", self.data)
      ```

    * *`format! ()`* : Creates a new String based on the format string and arguments. 
      (Does not use the std::fmt traits directly but relies on the same system).

    
    * *`println! ()`* : Prints the formatted string to standard output.

    * *`eprintln! ()`* : Prints the formatted string to standard error.

----------------------------------------------------------------------------------------------------------
# 1. Debug Trait:
---

### **Context & Purpose:**

* The `Debug` trait is used to format a value using the `{:?}` formatter, typically for debugging purposes.
* It helps you get a programmer-friendly, unambiguous representation of your type.
* Useful during development and troubleshooting.

### **When to implement manually?**

* When you have a custom `struct` or `enum` and want to control exactly how it appears when printed for 
  debugging.
* When `#[derive(Debug)]` doesn’t provide the format you want.

---
### Example:

Suppose we have a simple `struct`:

```rust
struct Point {
    x: i32,
    y: i32,
}
```

---

### Manual `Debug` Implementation:

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Debug for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Write output like: Point { x: 10, y: 20 }
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("{:?}", p);
}
```

---

### Explanation:

* We implement `fmt::Debug` trait for `Point`.
* The core method is `fmt(&self, f: &mut Formatter) -> fmt::Result`.
* We use `write!` macro to write to the formatter with the custom string.
* `{:?}` in `println!` calls this method.
* The output will be: `Point { x: 10, y: 20 }`.

---
# 2. `Display` Trait

### **Context & Purpose:**

* The `Display` trait is used to format a value for **end-user** presentation (with `{}` in formatting macros).
* Unlike `Debug`, which is programmer-focused, `Display` should provide a clean, readable string.
* Example: showing a date as "2025-10-03" instead of a debug dump.

### **When to implement manually?**

* When you want to control how your type is shown to users.
* When you want to support printing with `{}` in macros like `println!`.

---

### Example:

Using the same `Point` struct:

```rust
use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // User-friendly format: "(10, 20)"
        write!(f, "({}, {})", self.x, self.y)
    }
}

fn main() {
    let p = Point { x: 10, y: 20 };
    println!("{}", p);
}
```

---

### Explanation:

* We implement `fmt::Display` trait for `Point`.
* The core method is `fmt(&self, f: &mut Formatter) -> fmt::Result`.
* Use `write!` to write a clean, concise string `(x, y)`.
* `println!("{}", p)` prints `(10, 20)` instead of debug style.

---

### Note:

You can implement both `Debug` and `Display` for the same type; each serves different purposes.

---

# 3. `PartialEq` & `Eq` Traits

### **Context & Purpose:**

* **`PartialEq`** allows comparing two values for equality using `==` and `!=`.
* **`Eq`** is a marker trait indicating the equality is **total** (i.e., no partial equality or exceptions).
* Most types implement `PartialEq`; `Eq` is implemented when full equivalence makes sense.
* Used everywhere for comparing values in conditional statements, collections, etc.

### **When to implement manually?**

* When you want to customize equality logic.
* When the automatic `#[derive(PartialEq, Eq)]` is not sufficient or inappropriate.

---

### Example:

Suppose you have a struct where equality only depends on one field:

```rust
struct User {
    id: u32,
    username: String,
}
```

You want two `User` instances to be equal **only if their `id`s match**, ignoring `username`.

---

### Manual `PartialEq` & `Eq` Implementation:

```rust
#[derive(Debug)]
struct User {
    id: u32,
    username: String,
}

impl PartialEq for User {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for User {}

fn main() {
    let user1 = User { id: 1, username: "alice".to_string() };
    let user2 = User { id: 1, username: "alice_in_wonderland".to_string() };
    let user3 = User { id: 2, username: "bob".to_string() };

    println!("user1 == user2: {}", user1 == user2); // true
    println!("user1 == user3: {}", user1 == user3); // false
}
```

---

### Explanation:

* Implemented `PartialEq` manually by comparing only `id`.
* Implemented `Eq` as a marker trait (no methods).
* `user1` and `user2` are equal since their `id` is the same, despite different usernames.
* `user1` and `user3` are not equal.

---
# 4. `PartialOrd` & `Ord` Traits

### **Context & Purpose:**

* **`PartialOrd`** allows using comparison operators like `<`, `>`, `<=`, `>=`.
* **`Ord`** enables total ordering, often used in sorting (e.g., with `sort()` or `BinaryHeap`).
* **`Ord`** requires that the type also implements `PartialOrd` and `Eq`.

---

### **When to implement manually?**

* When you want **custom sorting logic** (e.g., sort users by age, score, or name length).
* When natural comparison isn’t sufficient or doesn’t exist (e.g., sorting complex structs).

---

### Example:

Let’s sort users by **username alphabetically**:

```rust
#[derive(Debug, Eq, PartialEq)]
struct User {
    id: u32,
    username: String,
}
```

---

### Manual `PartialOrd` & `Ord` Implementation:

```rust
use std::cmp::Ordering; // std module cmp is a utilities for comparision and ordering values.

#[derive(Debug, Eq, PartialEq)]
struct User {
    id: u32,
    username: String,
}

impl PartialOrd for User {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.username.cmp(&other.username))
    }
}

impl Ord for User {
    fn cmp(&self, other: &Self) -> Ordering {
        self.username.cmp(&other.username)
    }
}

fn main() {
    let mut users = vec![
        User { id: 3, username: "charlie".into() },
        User { id: 1, username: "alice".into() },
        User { id: 2, username: "bob".into() },
    ];

    users.sort();

    for user in users {
        println!("{:?}", user);
    }
}
```

---

### Output:

```
User { id: 1, username: "alice" }
User { id: 2, username: "bob" }
User { id: 3, username: "charlie" }
```

---

### Explanation:

* `PartialOrd::partial_cmp` returns an `Option<Ordering>`, allowing for cases where some values may be 
   non-comparable.
* `Ord::cmp` defines a **total order** and is required for sorting.
* We use `username.cmp(...)` to sort alphabetically.
* The `sort()` function relies on `Ord`.

---

> If your type may not be totally orderable (e.g., floating point numbers), implement only `PartialOrd`.

---
# 5. `Clone` Trait

### **Context & Purpose:**

* The `Clone` trait allows for **explicit duplication** of a value.
* You call `.clone()` to create a **deep copy** of a value.
* Required when you want to duplicate data that's **owned** and not trivially copyable.

---

### **When to implement manually?**

* When your type contains data that doesn’t automatically implement `Clone`.
* When you want to control the cloning logic — maybe to log, count, or skip certain fields.

---

### Example:

Suppose we have a struct with a `String` field, which is heap-allocated:

```rust
#[derive(Debug)]
struct User {
    id: u32,
    username: String,
}
```

The compiler won't let you copy `User` values unless you implement `Clone`.

---

### Manual `Clone` Implementation:

```rust
#[derive(Debug)]
struct User {
    id: u32,
    username: String,
}

impl Clone for User {
    fn clone(&self) -> Self {
        User {
            id: self.id,
            username: self.username.clone(), // Explicit deep clone
        }
    }
}

fn main() {
    let user1 = User { id: 1, username: "alice".to_string() };
    let user2 = user1.clone();

    println!("Original: {:?}", user1);
    println!("Cloned:   {:?}", user2);
}
```

---

### Output:

```
Original: User { id: 1, username: "alice" }
Cloned:   User { id: 1, username: "alice" }
```

---

### Explanation:

* We implement `Clone` manually and specify how each field should be cloned.
* `username.clone()` performs a deep copy of the `String` (new heap allocation).
* Now `user1` and `user2` are independent.

---

> Tip: You can often use `#[derive(Clone)]`, but manual implementation gives you control 
(e.g., logging, custom behavior).

---
# 6. `Copy` Trait

### **Context & Purpose:**

* `Copy` allows for **implicit, bitwise copy** of a value.
* Types that implement `Copy` can be duplicated just by assignment (no `.clone()` needed).
* It’s for **simple, stack-only** types like integers, floats, and types composed only of `Copy` fields.

### **Difference from `Clone`:**

* `Copy` is implicit and cheap (bitwise).
* `Clone` is explicit and can be expensive (deep copy).
* `Copy` requires that your type only contains `Copy` fields.

---

### When to implement manually?

* Usually, you **don’t implement `Copy` manually** yourself — you just add the `Copy` trait **if it makes sense**.
* For your own simple structs composed of `Copy` fields, you can derive it.
* **Important:** You must implement `Clone` if you implement `Copy`.

---

### Example:

```rust
#[derive(Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p1 = Point { x: 5, y: 10 };
    let p2 = p1; // Implicit copy, no move!
    
    println!("p1: {:?}", p1); // still valid
    println!("p2: {:?}", p2);
}
```

---

### Explanation:

* `Point` is simple: two `i32`s (which are `Copy`).
* We `derive(Copy, Clone)` so Rust knows it’s safe to copy by bits.
* After `let p2 = p1;`, `p1` is still valid because the data was copied, **not moved**.
* Without `Copy`, `p1` would be moved, and accessing it after would cause a compile error.

---

### Manual `Copy` Implementation (rarely done):

You **cannot** write out the methods of `Copy` yourself because it's a marker trait with **no methods**.

You simply declare it:

```rust
impl Copy for Point {}
impl Clone for Point {
    fn clone(&self) -> Self {
        *self
    }
}
```

But this is almost always replaced by `#[derive(Copy, Clone)]`.

---
# 7. `Iterator` Trait

### **Context & Purpose:**

* The `Iterator` trait allows a type to **produce a sequence of values, one at a time**.
* It’s fundamental to Rust’s **for loops**, **collection processing**, and **lazy evaluation**.
* Implementing your own iterator teaches you state management and Rust’s **borrowing rules**.

---

### When to implement manually?

* When creating a custom collection or data structure.
* When you want to provide a custom way to iterate over your type.

---

### Key Methods:

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
    // Many other provided methods built on top of `next()`
}
```

You **must implement `next()`**, which returns:

* `Some(item)` — the next item in the sequence.
* `None` — no more items.

---

### Example: A simple counter iterator

```rust
struct Counter {
    count: u32,
    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        Counter { count: 0, max }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    let mut counter = Counter::new(5);

    while let Some(num) = counter.next() {
        println!("{}", num);
    }
}
```

---

### Output:

```
1
2
3
4
5
```

---

### Explanation:

* `Counter` keeps track of the current count and a maximum.
* `next()` increments and returns the next number until `max`.
* Once it reaches `max`, returns `None` to signal end.
* You can use your iterator with `for` loops and all iterator adapters!

---

### Bonus: Using it in a `for` loop

```rust
fn main() {
    let counter = Counter::new(3);

    for num in counter {
        println!("{}", num);
    }
}
```

This prints:

```
1
2
3
```

---
# 8. `From` & `Into` Traits

### **Context & Purpose:**

* `From` and `Into` are traits for **type conversion**.
* `From<T>` defines how to convert **from** type `T` into your type.
* `Into<T>` defines how to convert **into** type `T` from your type.
* They are closely related: if you implement `From<T> for U`, Rust automatically provides `Into<U> for T`.
* Used for **safe, idiomatic conversions** without explicit method names.

---

### When to implement manually?

* When you want to define custom conversions between your types or from primitive types.
* To enable ergonomic `.into()` and `.from()` calls in your code.

---

### Example:

Suppose we have a simple `Point` struct and want to convert from a tuple `(i32, i32)`:

```rust
struct Point {
    x: i32,
    y: i32,
}
```

---

### Manual `From` Implementation:

```rust
impl From<(i32, i32)> for Point {
    fn from(coords: (i32, i32)) -> Self {
        Point { x: coords.0, y: coords.1 }
    }
}

fn main() {
    let tuple = (10, 20);

    // Using From:
    let p1 = Point::from(tuple);

    // Using Into (auto-implemented because of From):
    let p2: Point = tuple.into();

    println!("p1: ({}, {})", p1.x, p1.y);
    println!("p2: ({}, {})", p2.x, p2.y);
}
```

---

### Explanation:

* We implemented `From<(i32, i32)>` for `Point` to convert a tuple into a `Point`.
* You can now use `Point::from(tuple)` or `tuple.into()` interchangeably.
* The `.into()` method is very convenient and idiomatic for conversions.

---
# 9. `Default` Trait

### **Context & Purpose:**

* The `Default` trait provides a **default value** for a type.
* It’s useful when you want a **"zeroed" or initial state** for your type.
* Commonly used with structs where you want sensible defaults for fields.

---

### When to implement manually?

* When your type needs a custom default value.
* When you want to use `T::default()` or `..Default::default()` syntax for convenience.

---

### Example:

```rust
#[derive(Debug)]
struct Config {
    max_connections: u32,
    timeout_seconds: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_connections: 100,
            timeout_seconds: 30,
        }
    }
}

fn main() {
    let config = Config::default();

    println!("{:?}", config);
}
```

---

### Output:

```
Config { max_connections: 100, timeout_seconds: 30 }
```

---

### Explanation:

* Implemented `Default` trait for `Config` to provide initial values.
* Now you can create a default `Config` with `Config::default()`.
* This is especially handy with struct update syntax:

```rust
let custom_config = Config {
    timeout_seconds: 60,
    ..Default::default() // fill the rest with defaults
};
```

---

Ready to proceed with the **`Drop`** trait?




# 10. `Drop` Trait

### **Context & Purpose:**

* The `Drop` trait lets you specify **custom cleanup logic** when a value goes out of scope.
* It’s Rust’s way of implementing **RAII** (Resource Acquisition Is Initialization).
* Used for freeing resources, closing files, or other teardown actions.

---

### When to implement manually?

* When your type manages resources that need explicit cleanup (files, network sockets, memory, etc).
* When you want to run custom code at destruction time.

---

### Example:

```rust
struct Logger {
    name: String,
}

impl Drop for Logger {
    fn drop(&mut self) {
        println!("Logger '{}' is being dropped!", self.name);
    }
}

fn main() {
    {
        let logger = Logger { name: String::from("MyLogger") };
        println!("Logger created.");
    } // `logger` goes out of scope here, `drop` is called

    println!("End of main.");
}
```

---

### Output:

```
Logger created.
Logger 'MyLogger' is being dropped!
End of main.
```

---

### Explanation:

* Implemented `Drop` for `Logger` to print a message when dropped.
* When the variable goes out of scope, Rust calls the `drop` method automatically.
* You **cannot call `drop` explicitly**; instead, use `std::mem::drop()` if needed.
* **Important:** Do not manually call `drop` inside `drop` method to avoid recursion!

---

# Advanced or Niche Traits:

## Bonus 1: `Deref` and `DerefMut` Traits

### **Context & Purpose:**

* `Deref` allows you to customize **how a smart pointer behaves when dereferenced** (`*` operator).
* `DerefMut` is the mutable counterpart, letting you dereference to mutable data.
* Enables **smart pointer types** (like `Box<T>`, `Rc<T>`, `RefCell<T>`) to behave like references.
* Makes your custom types **transparent** to users when accessing inner data.

---

### When to implement manually?

* When you build your own smart pointer or wrapper type.
* When you want to provide ergonomic access to inner data without explicit method calls.

---

### Example: Simple wrapper around `String`

```rust
use std::ops::{Deref, DerefMut};

struct MyString {
    inner: String,
}

impl MyString {
    fn new(s: &str) -> Self {
        MyString { inner: s.to_string() }
    }
}

// Implement Deref to allow &MyString to behave like &String
impl Deref for MyString {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

// Implement DerefMut for mutable dereferencing
impl DerefMut for MyString {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

fn main() {
    let mut s = MyString::new("Hello");

    // Because of Deref, we can call String methods directly
    println!("Length: {}", s.len());

    // DerefMut lets us mutate inner String transparently
    s.push_str(", world!");
    println!("{}", s);
}
```

---

### Explanation:

* `Deref` lets us treat `MyString` like a `&String` when accessing methods like `.len()`.
* `DerefMut` enables mutation through `&mut MyString` without exposing `inner` explicitly.
* This pattern is the core of Rust’s smart pointers and wrapper types.

---
## Bonus 2: `Hash` Trait

### **Context & Purpose:**

* The `Hash` trait allows your type to be **hashed**, which is necessary to use it as a key in hash maps (`HashMap`, `HashSet`).
* Hashing transforms your data into a fixed-size value (a hash) used for quick lookup.
* Usually implemented alongside `Eq` because hash-based collections rely on both equality and hashing.

---

### When to implement manually?

* When you have a custom struct and want to control how it’s hashed.
* When the default derived hash doesn’t fit your needs or you want to hash only part of the data.

---

### Example:

```rust
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

#[derive(Debug, Eq, PartialEq)]
struct User {
    id: u32,
    username: String,
}

impl Hash for User {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Only hash the id, ignoring username
        self.id.hash(state);
    }
}

fn main() {
    let user1 = User { id: 1, username: "alice".to_string() };
    let user2 = User { id: 1, username: "alice_in_wonderland".to_string() };

    let mut hasher1 = DefaultHasher::new();
    user1.hash(&mut hasher1);
    let hash1 = hasher1.finish();

    let mut hasher2 = DefaultHasher::new();
    user2.hash(&mut hasher2);
    let hash2 = hasher2.finish();

    println!("Hash of user1: {}", hash1);
    println!("Hash of user2: {}", hash2);
    println!("user1 == user2: {}", user1 == user2);
}
```

---

### Explanation:

* Implemented `Hash` manually to hash only `id` field.
* `DefaultHasher` is a common hasher implementation provided by Rust.
* Since `user1` and `user2` have the same `id`, their hashes are equal.
* Hashing aligns with equality: equal objects must produce equal hashes.

---
## Bonus 3: `AsRef` and `AsMut` Traits

### **Context & Purpose:**

* `AsRef<T>` is for **cheap reference-to-reference conversions**.
* It allows you to convert a value to a reference of another type (`&T`).
* `AsMut<T>` is the mutable counterpart for getting mutable references (`&mut T`).
* Often used to make APIs more flexible by accepting types that can be cheaply converted into the expected reference type.

---

### When to implement manually?

* When you want your type to be easily converted to a reference of another type.
* To support generic code that accepts anything convertible to a reference type.

---

### Example:

Imagine a wrapper struct around `String` and you want it to act like a `&str` when needed.

```rust
struct MyString {
    inner: String,
}

impl AsRef<str> for MyString {
    fn as_ref(&self) -> &str {
        &self.inner
    }
}

impl AsMut<str> for MyString {
    fn as_mut(&mut self) -> &mut str {
        &mut self.inner
    }
}

fn print_str(s: &str) {
    println!("{}", s);
}

fn main() {
    let mut my_str = MyString { inner: "hello".to_string() };

    // Thanks to AsRef, we can pass &my_str where &str is expected
    print_str(my_str.as_ref());

    // AsMut lets us mutate inner string via &mut my_str
    let inner_mut: &mut str = my_str.as_mut();
    inner_mut.make_ascii_uppercase();

    print_str(&my_str.inner);
}
```

---

### Explanation:

* `AsRef<str>` lets you get a `&str` from your type.
* `AsMut<str>` lets you get a mutable `&mut str`.
* Useful for generic programming where you want to accept multiple types convertible to a certain reference.

---
## Bonus 4: `Borrow` Trait

### **Context & Purpose:**

* The `Borrow` trait is used to allow types to be **borrowed as another type**.
* It’s mostly used by collections (like `HashMap` or `BTreeMap`) to look up keys by reference without requiring ownership.
* Helps avoid unnecessary cloning or allocations when performing lookups.
* Similar to `AsRef` but with stronger guarantees about equality and hashing compatibility.

---

### When to implement manually?

* When you want your type to be usable as a borrowed key type in maps or sets.
* To enable lookups by references to a type different than the owned key.

---

### Example:

Suppose you have a `User` struct and want to use it as a key in a `HashMap`, but you want to allow lookup by `&str` (username) without creating a full `User`.

```rust
use std::collections::HashMap;
use std::borrow::Borrow;

#[derive(Debug, Eq, PartialEq, Hash)]
struct User {
    username: String,
    id: u32,
}

impl Borrow<str> for User {
    fn borrow(&self) -> &str {
        &self.username
    }
}

fn main() {
    let mut users = HashMap::new();

    users.insert(
        User {
            username: "alice".to_string(),
            id: 1,
        },
        "Admin",
    );

    // Lookup using &str without constructing User
    let role = users.get("alice");

    println!("Role for alice: {:?}", role);
}
```

---

### Explanation:

* Implementing `Borrow<str>` allows `HashMap<User, _>` to use `&str` for lookups.
* Enables efficient, allocation-free lookups without creating a new `User`.
* `Borrow` requires consistency with `Eq` and `Hash` implementations.

---
## Bonus 5: `Fn`, `FnMut`, and `FnOnce` Traits

### **Context & Purpose:**

* These traits represent **callable things** in Rust—functions, closures, or anything you can “call.”
* They define how a value behaves when used like a function.
* Differences:

  * `Fn`: can be called multiple times without mutating state.
  * `FnMut`: can mutate state, so it requires a mutable reference to call.
  * `FnOnce`: can only be called once (takes ownership), consuming the value.

---

### When to implement manually?

* Rarely done manually, but useful to understand closures and custom callable types.
* If you want to create a type that behaves like a function or closure.

---

### Example: Implementing `Fn` for a struct

```rust
use std::ops::Fn;

struct Adder {
    amount: i32,
}

// For implementing Fn traits, Rust requires nightly features or more complex setups,
// but you can implement FnOnce manually as a simpler example.

use std::ops::FnOnce;

impl FnOnce<(i32,)> for Adder {
    type Output = i32;

    extern "rust-call" fn call_once(self, args: (i32,)) -> Self::Output {
        let x = args.0;
        x + self.amount
    }
}

fn main() {
    let add_five = Adder { amount: 5 };

    // Using FnOnce trait requires nightly or unstable features for seamless use,
    // so here we can call directly via the trait method:
    let result = FnOnce::call_once(add_five, (10,));

    println!("Result: {}", result); // prints 15
}
```

---

### Explanation:

* `FnOnce` must define `call_once` which consumes `self` and takes a tuple of arguments.
* `FnMut` and `Fn` extend on this with mutable or shared references.
* Implementing these manually is advanced and rarely needed, but understanding them helps with closures and traits.

---

