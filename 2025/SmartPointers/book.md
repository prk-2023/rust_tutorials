# Smart Pointers:

## Intro:

- Pointer: Pointer is a general concept for a variable that contains the address of address in memory. The
  address refers to, or "points at", some other data. 

- Most common pointer in Rust is **reference**, they are indicated by the "&" symbol and borrow the value
  they point to. 

- Rust reference do not have any other special capabilities apart from referring to data and they also have
  no overhead. 

- Smart Pointers: In Rust they are data structures that act like a pointer but also have **meta data and
  capabilities**. 

[ Smart pointers are originated from C++ and more in other document ./CPP-SmartPointers.md and they also
exist in other languages ]

### “smart pointer” in Rust

- **Smart Pointer** is any type that **implements pointer-like semantics** (i.e., it implements `Deref` and 
  often `Drop`), and **manages ownership or lifetime** of the data it points to.

  So, it’s more about **behavior** than **where the data lives**.

  Its also to note that not all smart pointers data is held on Heap and below table gives a clear idea of
  some popular standard smart pointers from the standard library 

### Examples and whether they heap-allocate

| Smart Pointer Type           | Heap Allocation?     | Explanation                                                                                                                 |
| ---------------------------- | -------------------- | --------------------------------------------------------------------------------------------------------------------------- |
| `Box<T>`                     | ✅ Yes                | Always allocates `T` on the heap.                                                                                           |
| `Rc<T>` / `Arc<T>`           | ✅ Yes                | Heap allocates the inner value and maintains reference counts.                                                              |
| `Vec<T>`                     | ✅ Yes (for elements) | The vector’s buffer (the elements) lives on the heap.                                                                       |
| `String`                     | ✅ Yes                | The underlying UTF-8 buffer is heap-allocated.                                                                              |
| `Cow<'a, T>`                 | 🟡 Maybe             | Heap-allocates only when it needs to own a copy; otherwise borrows from another source.                                     |
| `RefCell<T>`                 | ❌ No (by itself)     | The `RefCell` itself lives wherever it’s declared — it provides *runtime-checked interior mutability*, not heap allocation. |
| `Cell<T>`                    | ❌ No                 | Similar — provides interior mutability, but no heap allocation.                                                             |
| `Pin<T>`                     | ❌ Depends on `T`     | Doesn’t imply heap allocation — just prevents moving the value.                                                             |
| `&T` / `&mut T` (references) | ❌ No                 | Not smart pointers — plain references. No allocation.                                                                       |
| `ManuallyDrop<T>`            | ❌ No                 | Just controls drop behavior; no heap allocation.                                                                            |


Implies => 

> A **smart pointer** in Rust is about **ownership and behavior**, not **storage location**.

Many smart pointers *do* use the heap because it allows flexible lifetimes or dynamic sizing, but others 
(like `RefCell`, `Cell`, `Pin`) **just wrap existing data** without changing where it lives.

---

###  Mnemonic

> “All heap allocations are smart pointers, but not all smart pointers allocate on the heap.”

(or more accurately: some smart pointers don’t allocate at all.)

- With Ownership and Borrowing in Rust, there is a difference between Smart Pointers and References: 
    - References (&) : only Borrow the data and do not take ownership.
    - Smart Pointers: in many cases _own_ the data they point to.

- Some Smart Pointers we have already encountered are `String` and `Vec<T>` 

### Implementation:

- Smart Pointers are generally implemented using structs. Unlike ordinary structures, smart pointers
  implement the `Deref` and `Drop` trait, which allow an instance of the smart pointer struct 
  to behave like a reference so you can write your code to work with either "reference" or "smart pointer".

- `Deref` Trait: allows an instance of the smart pointer struct to behave like a reference so you can write
  code to work with either "reference" or "smart pointer".

- `Drop` Trait: allows you to customize the code that’s run when an instance of the smart pointer goes out
  of scope.


## Using Box<T> to point Data on the Heap:

- Most straightforward smart pointer is a **box**, written as `Box<T>`.

- Boxes allow you to store data on the heap rather then the stack. What remains on the stack is the pointer
  to the heap data. 

- Boxes don't have performance overhead, other than storing their data on the heap instead of the stack. 

- Boxes do not have any other capabilities apart from storing data on the heap. 

- Boxes are used in below situations:
    * When you have a type whose size can't be known at compile time and you want to use a value of that
      type in a context that requires an exact size.

      Ex: Recursive Types with Boxes.

    * When You have large amount of data and you want to transfer ownership but ensure the data won't be
      copied when you do so.

      Ex: transferring ownership of a large amount of data can take a long time because the data is copied 
      around on the stack. To improve performance, we can store the large amount of data on the heap in a 
      box. Then, only the small amount of pointer data is copied around on the stack, while the data it 
      references stays in one place on the heap. 

    * When you want to own a value and you care only that it’s a type that implements a particular trait
      rather than being of a specific type. 

      Ex: Trait Object and using trait objects that allow for values of different types.

- Syntax and how to interact with value stored within a `Box<T>`:

```rust 
fn main () {
    let b = Box::new(5);
    println!("b = {b}");
}
```
    - `b` is defined to have a value of `Box` that points to the value `5` which is allocated on the heap. 
    - `b` goes out of scope when main() closes.

- Recursive type: A value of a recursive type can have another value of the same type as part of itself. 
  Recursive types pose an issue because Rust needs to know at compile time how much space a type takes up.
  Since nesting of the value can go infinitely, in which case Rust would not know much of memory is needed.
  So In Rust its more common to use `Vec<T>` in place of infinitely recursive type.

```rust 
// enum that represents a cons list
enum List {
    Cons(i32, List),
    Nil,
}

//Using the List type to store the list 1, 2, 3
use crate::List::{Cons, Nil};

fn main() {
    let list = Cons(1, Cons(2, Cons(3, Nil)));
}
```

This fails to compile with a warning that Rust can not decide how much space is needed to store a value of
non-recursive type.
To fix this issue of determining the amount of memory that is required. you should change the data struct to
store the value indirectly by storing a pointer to the value instead. 

Because `Box<T>` is a pointer, Rust know how much of space `Box<T>` needs ( a pointer size which does not
change based on what it points to)

Changing the above program with this info:

```rust 
enum List {
    Cons(i32, Box<List>),
    Nil,
}
use crate::List::{Cons, Nil};

fn main() {
    let List = Cons (1, Box::new(2, Box::new(Cons(3, Box::new(Nil)))));
}
```

By using `Box<T>` we have solved the problem of infinitely, recursive chain, allowing the compiler to figure
out how much memory it needs to store `List` value.

- The `Box<T>` type is a smart pointer because it implements the `Deref` trait, which allows `Box<T>` values
  to be treated like references. When a `Box<T>` value goes out of scope, the heap data that the box is 
  pointing to is cleaned up as well because of the `Drop` trait implementation. 

Note: These two traits will be even more important to the functionality provided by the other smart pointer
types.

### Treating Smart Pointers Like Regular References with Deref?

- Implementing the `Deref` trait allows you to customize the behavior of the dereference operator `*`. By
  implementing `Deref` in such a way that a smart pointer can be treated like a regular reference, you can 
  write code that operates on references and use that code with smart pointers too.


- look deeper:
    * What the `Deref` trait does.
    * How it allows *smart pointers* (like `Box<T>` or your own types) to behave like regular references (`&T`).
    * How *deref coercion* makes code ergonomic and seamless.

---

#### The Motivation: Smart Pointers Should Feel Natural

In Rust, a **smart pointer** is just a type that **acts like a pointer** but adds some extra behavior — 
for ex: managing heap memory (`Box<T>`), reference counting (`Rc<T>`), or interior mutability (`RefCell<T>`).

Let’s start with a simple example:

```rust
fn main() {
    let x = 5;
    let y = Box::new(x);

    println!("x = {}", x);
    println!("y = {}", y);
}
```

Output:

```
x = 5
y = 5
```

Even though `y` is a `Box<i32>`, we can print it like an `i32`.
Why does that work? Because `Box<T>` implements the **`Deref`** trait.

#### What Is the `Deref` Trait?

The `Deref` trait lets you customize what happens when you use the **dereference operator (`*`)**.

Simplified:

```rust
trait Deref {
    type Target: ?Sized;

    fn deref(&self) -> &Self::Target;
}
```

If your type implements `Deref`, you can use `*my_value` to access the data it points to.

#### Implementing `Deref` for a Custom Smart Pointer

Let’s make our own simplified `MyBox<T>` type:

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}
```

Now, without `Deref`, you can’t do:

```rust
let x = 5;
let y = MyBox::new(x);
println!("{}", *y); // ❌ Error: cannot dereference MyBox<i32>
```

So let’s implement `Deref`:

```rust
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
```

Now this works:

```rust
fn main() {
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y); // ✅ works now!
}
```

---

#### How Deref Works Under the Hood

When you write `*y`, Rust automatically calls: **\*(y.deref())**

```rust
*(y.deref())
```

So your `deref()` method returns a reference to the inner value (`&T`), and the extra `*` removes that layer
of indirection.

Think of it like unwrapping layers:

```
MyBox<T> → &T → T
```

---

#### Deref Coercion — Making Code Ergonomic

Rust has a feature called **deref coercion**, which means:

> Rust automatically converts references from one type to another when it finds a `Deref` implementation chain that fits.

This makes calling functions with smart pointers much smoother.

##### Example: String Slices and Smart Pointers

```rust
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rust"));
    hello(&m); // ✅ Works, even though hello() expects &str
}
```

What happens here:

1. `&m` → `&MyBox<String>`
2. Rust sees that `MyBox<T>` implements `Deref<Target = T>` → converts to `&String`
3. `String` also implements `Deref<Target = str>` → converts again to `&str`
4. ✅ Now the type matches what `hello` expects.

No explicit `&*m` or messy syntax needed — **deref coercion** makes it seamless.

---

#### Deref Coercion Rules

Rust applies **deref coercion** in function/method calls when:

* You have a `&T`, and the parameter expects a `&U`
* And `T: Deref<Target = U>` (or recursively follows a chain of derefs)

This works for:

* `Box<T>`
* `Rc<T>`
* `Arc<T>`
* `String` (which derefs to `str`)
* Any custom smart pointer implementing `Deref`

---

#### A Note on Mutability: `DerefMut`

If you want to deref **mutable references**, implement `DerefMut` too:

```rust
use std::ops::{Deref, DerefMut};

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
```

This lets you write:

```rust
let mut m = MyBox::new(String::from("Hi"));
m.push_str(", there!"); // ✅ works thanks to DerefMut
```

---

#### Summary

| Concept        | Description                                                             |
| -------------- | ----------------------------------------------------------------------- |
| `Deref`        | Allows smart pointers to act like references.                           |
| `*` operator   | Calls the `deref()` method automatically.                               |
| Deref Coercion | Rust automatically converts `&T` to `&U` when possible through `Deref`. |
| `DerefMut`     | Allows mutable dereferencing (`*mut_ptr`).                              |
| Goal           | Make smart pointers feel like regular references!                       |

---

#### Final Example

```rust
use std::ops::{Deref, DerefMut};

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self { MyBox(x) }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut self.0 }
}

fn greet(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = MyBox::new(String::from("Rustacean"));
    greet(&m); // Deref coercion: &MyBox<String> → &String → &str
}
```

Output:

```
Hello, Rustacean!
```

-----------------------------------------------

## Running Code on Cleanup with the `Drop` Trait

Things to cover:
* What the `Drop` trait is.
* How and when Rust automatically calls it.
* How to use it to release resources safely.
* Why you **can’t** call `drop()` manually — and how to actually do it safely.

#### Why Do We Need `Drop`?

In many languages (like C++, Java, Python), you can write code that automatically runs when an object goes 
"out of scope” or is “garbage collected."

In Rust, that behavior is **explicitly defined** through the **`Drop` trait**.

> The `Drop` trait lets you run custom code *automatically* when a value goes out of scope.

---

#### The `Drop` Trait Definition

Here’s what it looks like in the standard library:

```rust
pub trait Drop {
    fn drop(&mut self);
}
```

* You **implement `drop()`** on your type to define what happens when it’s destroyed.
* Rust **automatically calls it** when the variable goes out of scope.

---

#### A Simple Example

Let’s build a struct and watch its cleanup happen:

```rust
struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data: `{}`", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("My Stuff"),
    };
    let d = CustomSmartPointer {
        data: String::from("Other Stuff"),
    };
    println!("CustomSmartPointers created.");
}
```
Output:
```
CustomSmartPointers created.
Dropping CustomSmartPointer with data: `Other Stuff`
Dropping CustomSmartPointer with data: `My Stuff`
```
Notice:

* `drop()` is called **automatically** when `c` and `d` go **out of scope** (at the end of `main`).
* The order is **reverse of creation** — just like stack unwinding.

#### Important: You *Can’t* Call `drop()` Manually

This won’t work:

```rust
fn main() {
    let c = CustomSmartPointer { data: String::from("Stuff") };
    c.drop(); // ❌ error: explicit destructor calls not allowed
}
```

Why?
Because Rust’s memory safety model guarantees **each value is dropped exactly once**, and if you could call 
`drop()` manually, Rust couldn’t prevent double-free errors.

#### Correct Way: Using `std::mem::drop`

If you want to drop a value early, use `std::mem::drop`, which *takes ownership* of the value and runs its 
`Drop` implementation.

```rust
use std::mem::drop;
fn main() {
    let c = CustomSmartPointer {
        data: String::from("Stuff"),
    };
    println!("CustomSmartPointer created.");

    drop(c); // ✅ Drops `c` right here

    println!("CustomSmartPointer dropped before end of main.");
}
```

Output:

```
CustomSmartPointer created.
Dropping CustomSmartPointer with data: `Stuff`
CustomSmartPointer dropped before end of main.
```

---

#### When Does Drop Happen?

The `drop()` method runs automatically:

* When a variable **goes out of scope**.
* When a smart pointer (like `Box<T>`, `Rc<T>`, `Arc<T>`, etc.) **releases ownership**.
* During **panic unwinding**, as Rust cleans up partially-constructed data safely.

---

#### Real Use Cases of `Drop`

The `Drop` trait is not just for printing messages — it’s how Rust safely releases **system resources** like:

| Resource             | Example Type                 | What `drop()` Does    |
| -------------------- | ---------------------------- | --------------------- |
| Heap memory          | `Box<T>`, `Vec<T>`, `String` | Frees heap data       |
| File handles         | `std::fs::File`              | Closes the file       |
| Locks                | `std::sync::MutexGuard`      | Unlocks automatically |
| Network sockets      | `TcpStream`                  | Closes the socket     |
| Database connections | Custom type                  | Releases DB handle    |

---

#### Drop Order: Reverse of Creation

Let’s demonstrate clearly:

```rust
struct PrintOnDrop(&'static str);

impl Drop for PrintOnDrop {
    fn drop(&mut self) {
        println!("Dropping {}", self.0);
    }
}

fn main() {
    let _a = PrintOnDrop("a");
    let _b = PrintOnDrop("b");
    let _c = PrintOnDrop("c");
    println!("Exiting main...");
}
```

### Output:

```
Exiting main...
Dropping c
Dropping b
Dropping a
```

=> Rust cleans up **in reverse order of declaration** (like stack unwinding).

---

#### What Happens During Panic?

Even during a panic, Rust still runs `drop()` on all values as the stack unwinds (unless you abort).

Example:

```rust
struct Guard;

impl Drop for Guard {
    fn drop(&mut self) {
        println!("Cleaning up Guard!");
    }
}

fn main() {
    let _g = Guard;
    panic!("Something went wrong!");
}
```

Output:

```
Cleaning up Guard!
thread 'main' panicked at 'Something went wrong!'
```

So even on panic, Rust ensures **resource cleanup is deterministic and safe**.

#### `Drop` in Smart Pointers

All Rust smart pointers rely on `Drop` internally:

| Smart Pointer      | What `Drop` Does                                                |
| ------------------ | --------------------------------------------------------------- |
| `Box<T>`           | Frees the heap memory for `T`.                                  |
| `Rc<T>` / `Arc<T>` | Decrements the reference count and drops `T` when it hits zero. |
| `RefCell<T>`       | Drops any active `Ref`/`RefMut` handles.                        |
| `MutexGuard<T>`    | Unlocks the mutex.                                              |

That’s why you rarely need to call `drop()` manually — Rust ensures resources are freed automatically.

---

### Summary

| Concept        | Description                                                     |
| -------------- | --------------------------------------------------------------- |
| `Drop` trait   | Defines custom cleanup logic when values go out of scope.       |
| Automatic call | Rust calls `drop()` automatically — you can’t call it directly. |
| Manual cleanup | Use `std::mem::drop(value)` to drop early.                      |
| Drop order     | Reverse of creation (stack unwinding).                          |
| Panic-safe     | `Drop` runs even during panics.                                 |

---

##  Final Example: Combining `Deref` + `Drop`

Here’s how you might combine both traits in a simple smart pointer:

```rust
use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> Self {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> Drop for MyBox<T> {
    fn drop(&mut self) {
        println!("Dropping MyBox, cleaning up!");
    }
}

fn main() {
    let m = MyBox::new(String::from("Smart pointer"));
    println!("MyBox contains: {}", *m);
} // Automatically prints drop message here
```

Output:

```
MyBox contains: Smart pointer
Dropping MyBox, cleaning up!
```

---

### Key Takeaway

> Rust’s `Drop` trait gives you deterministic, safe, automatic cleanup — **no garbage collector, no leaks, 
  no double frees**.

It’s the foundation of Rust’s memory and resource safety guarantees.
You write what should happen, and Rust guarantees *it happens once and only once*.

Till now we have covered:

* Smart pointers conceptually
* How `Deref` lets them act like references
* How `Drop` ensures safe automatic cleanup


## Reference Counting with `Rc<T>`  ( Next Smart Pointer )

Topics:
* What **reference counting** means
* How `Rc<T>` lets **multiple owners** share a single value
* When to use it (and when not to)
* How automatic cleanup still happens safely with `Drop`

---

### The Problem: Sharing Ownership

So far, we’ve seen that Rust enforces **one owner per value**.

Example:

```rust
let a = Box::new(5);
let b = a; // ❌ a is moved into b
println!("{}", a); // error: a is no longer valid
```

But sometimes we *want* multiple parts of a program to **share ownership** of the same value.

For example:

* In a tree structure, multiple branches may point to the same node.
* In a graph, multiple edges might reference the same vertex.

That’s where **reference counting** comes in.

---

### Introducing `Rc<T>`

`Rc<T>` (Reference Counted) is a **smart pointer** type in Rust’s standard library that **enables multiple 
ownership**.

When you clone an `Rc<T>`, it **increases a counter** tracking how many owners exist.
When all clones go out of scope, Rust automatically **drops** the value.

You can find it in:

```rust
use std::rc::Rc;
```

##### Basic Example

```rust
use std::rc::Rc;

fn main() {
    let a = Rc::new(String::from("Hello, Rust!"));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Rc::clone(&a);
    println!("count after cloning to b = {}", Rc::strong_count(&a));

    {
        let c = Rc::clone(&a);
        println!("count after cloning to c = {}", Rc::strong_count(&a));
    } // c goes out of scope here

    println!("count after c is dropped = {}", Rc::strong_count(&a));
}
```

Output:

```
count after creating a = 1
count after cloning to b = 2
count after cloning to c = 3
count after c is dropped = 2
```

The reference count increases with each `Rc::clone()` and decreases when clones go out of scope.

#### How `Rc<T>` Works Internally

You can imagine `Rc<T>` as a structure like this (simplified):

```rust
struct Rc<T> {
    strong_count: usize,
    value: T,
}
```
* Every `Rc::clone()` increments `strong_count`.
* Every `Drop` of an `Rc` decrements it.
* When it hits **0**, Rust automatically drops the inner value (`T`).

This behavior uses the `Drop` trait behind the scenes.

#### Cloning `Rc<T>` vs Cloning the Data

When you call `Rc::clone(&a)`, it does **not** copy the underlying data — only the reference count.

```rust
use std::rc::Rc;

fn main() {
    let s1 = Rc::new(String::from("hello"));
    let s2 = Rc::clone(&s1);

    println!("s1: {}, s2: {}", s1, s2); // both print the same data
}
```

> Tip: `Rc::clone()` is cheap — it only increments a counter, not a deep copy.

---

#### Example: Shared Ownership in a Data Structure

Let’s model a **list** where multiple lists share ownership of the same tail.

```rust
use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
```

### Output:

```
count after creating a = 1
count after creating b = 2
count after creating c = 3
count after c goes out of scope = 2
```

Each new list (`b`, `c`) shares the same tail (`a`) — when they’re dropped, the reference count decreases.
When it reaches **zero**, the list memory is freed automatically.

#### Reference Counting and `Drop`

When the last `Rc<T>` owner goes out of scope:

* The reference count reaches 0
* `Rc`’s `Drop` implementation automatically frees the inner value

You don’t need to (and can’t) call `drop()` manually — it’s all automatic and safe.

---

#### Limitations of `Rc<T>`

`Rc<T>` is powerful, but it comes with restrictions:

| Limitation                                 | Explanation                                                                                                   |
| ------------------------------------------ | ------------------------------------------------------------------------------------------------------------- |
| ❌ Not thread-safe                          | `Rc<T>` cannot be shared across threads (it’s `!Send` and `!Sync`).                                           |
| ✅ Use `Arc<T>` for threads                 | If you need multi-threaded reference counting, use `Arc<T>` (“atomic reference counting”).                    |
| ⚠️ Cannot mutate through shared references | You can’t mutate the data inside an `Rc<T>` directly because Rust enforces immutability for shared ownership. |
| ✅ Use `RefCell<T>` if you need mutability  | You can wrap your data in `Rc<RefCell<T>>` to allow interior mutability.                                      |

---

#### Common Pattern: `Rc<RefCell<T>>`

You’ll often see these together:

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::clone(&value);
    let b = Rc::clone(&value);

    *a.borrow_mut() += 10;
    *b.borrow_mut() += 20;

    println!("Final value = {}", value.borrow());
}
```

Output:

```
Final value = 35
```

This pattern allows:

* Multiple owners (`Rc`)
* Mutability inside (`RefCell`)

We’ll explore this combination more deeply in the next subtopic

---

###  Summary

| Concept       | Description                                                             |
| ------------- | ----------------------------------------------------------------------- |
| `Rc<T>`       | Smart pointer that enables multiple ownership using reference counting. |
| `Rc::clone()` | Increments count; does **not** copy data.                               |
| Drop behavior | When count hits 0, data is dropped automatically.                       |
| Thread safety | `Rc<T>` is single-threaded. Use `Arc<T>` for multi-threaded code.       |
| Mutability    | Use `Rc<RefCell<T>>` to allow interior mutability.                      |

---

### Final Example: Shared Tree Nodes

```rust
use std::rc::Rc;

#[derive(Debug)]
enum Node {
    Leaf(i32),
    Branch(Rc<Node>, Rc<Node>),
}

use Node::{Leaf, Branch};

fn main() {
    let left = Rc::new(Leaf(1));
    let right = Rc::new(Leaf(2));
    let tree = Branch(Rc::clone(&left), Rc::clone(&right));

    println!("tree: {:?}\nleft count: {}, right count: {}",
        tree,
        Rc::strong_count(&left),
        Rc::strong_count(&right)
    );
}
```

Output:

```
tree: Branch(Leaf(1), Leaf(2))
left count: 2, right count: 2
```

Each node is shared safely — `Rc` ensures cleanup happens once both the tree and any shared owners go out 
of scope.

### Key Takeaway

> `Rc<T>` lets you share immutable ownership of data in a single-threaded context.
> It’s automatic, reference-counted, and integrates safely with Rust’s ownership and `Drop` system.

What is covered till now:

* `Box<T>` → simple ownership + heap allocation
* `Rc<T>` → shared ownership (multiple readers)
* But: **neither allows mutation** through shared references.

So what if you **want shared, mutable data** — safely, without `unsafe`?

That’s where `RefCell<T>` steps in.

## Interior Mutability and `RefCell<T>`

To Cover
* What **interior mutability** means
* How `RefCell<T>` allows mutation through an immutable reference
* The difference between **compile-time** and **runtime** borrow checking
* How to combine `Rc<T>` + `RefCell<T>` for shared, mutable data

---

### The Problem: Mutability vs Ownership Rules

Rust’s basic rule:

> You can have *either one mutable reference* OR *any number of immutable references* — but not both at the
  same time.

Example:

```rust
let x = 5;
let y = &mut x; // ❌ cannot borrow `x` as mutable, as it is not declared mutable
```

And even when declared `mut`, you can’t borrow mutably through an immutable owner:

```rust
let x = 5;
let r = &x;
*r = 10; // ❌ cannot assign to data through an immutable reference
```

But what if we need to **mutate data that’s owned by a shared smart pointer**, like `Rc<T>`?

---

### Introducing `RefCell<T>`

`RefCell<T>` provides **interior mutability**:

> It lets you *mutate the inside of an immutable object* — but checks borrow rules at **runtime**, not compile-time.

```rust
use std::cell::RefCell;

fn main() {
    let x = RefCell::new(5);
    *x.borrow_mut() += 1;
    println!("x = {}", x.borrow());
}
```

Output:

```
x = 6
```

✅ `borrow_mut()` gives you a mutable reference to the data inside.
✅ `borrow()` gives you an immutable reference.
⚠️ If you violate borrow rules (e.g. two mutable borrows), you’ll get a **runtime panic**.

---

### Key Idea: Compile-Time vs Runtime Checking

| Type            | Borrow Checking | Mutable Sharing Allowed? |
| --------------- | --------------- | ------------------------ |
| `&T` / `&mut T` | Compile-time    | ❌                        |
| `RefCell<T>`    | Runtime         | ✅                        |

So `RefCell<T>` trades **compile-time safety** for **runtime flexibility**.
If you misuse it, Rust will panic at runtime instead of refusing to compile.

---

####  Example: Violating Borrow Rules at Runtime

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(42);

    let m1 = data.borrow_mut();
    let m2 = data.borrow_mut(); // ❌ PANIC!

    println!("{:?}, {:?}", m1, m2);
}
```

Output:

```
thread 'main' panicked at 'already borrowed: BorrowMutError'
```

Rust’s borrow checker enforces:

* Only one active `borrow_mut()`
* Any number of `borrow()`s (immutable borrows)
* But never both at the same time

---

#### Common Methods on `RefCell<T>`

| Method         | Type Returned | Description                                          |
| -------------- | ------------- | ---------------------------------------------------- |
| `borrow()`     | `Ref<T>`      | Immutable borrow (panics if a mutable borrow exists) |
| `borrow_mut()` | `RefMut<T>`   | Mutable borrow (panics if any borrow exists)         |
| `into_inner()` | `T`           | Consumes the `RefCell`, returning the inner value    |

These `Ref<T>` and `RefMut<T>` types act like smart pointers and automatically release their borrow when 
they go out of scope.

---

#### Example: Combining `Rc<T>` and `RefCell<T>`

You often see this combination:

> `Rc<RefCell<T>>` — multiple owners, each able to mutate the shared data.

Let’s see it in action:

```rust
use std::rc::Rc;
use std::cell::RefCell;

fn main() {
    let shared_value = Rc::new(RefCell::new(5));

    let a = Rc::clone(&shared_value);
    let b = Rc::clone(&shared_value);

    *a.borrow_mut() += 10;
    *b.borrow_mut() += 5;

    println!("Final value: {}", shared_value.borrow());
}
```

Output:

```
Final value: 20
```

✅ Multiple owners via `Rc`
✅ Mutability inside via `RefCell`
✅ Automatic cleanup via `Drop`
✅ Safe and simple (panics only if rules violated at runtime)

#### Example: Shared Mutable List

Let’s revisit our linked list example, now allowing **mutation** of a shared tail.

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let value = Rc::new(RefCell::new(5));
    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10; // Mutate the shared value

    println!("a = {:?}", a);
    println!("b = {:?}", b);
    println!("c = {:?}", c);
}
```

Output:

```
a = Cons(RefCell { value: 15 }, Nil)
b = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
c = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
```

Notice how updating `value` affected all lists — they all shared the same inner cell.

#### When to Use `RefCell<T>`

| Situation                          | Recommended Type                    |
| ---------------------------------- | ----------------------------------- |
| Single ownership, need mutability  | `Box<T>` or `&mut T`                |
| Multiple ownership, immutable data | `Rc<T>`                             |
| Multiple ownership + mutability    | `Rc<RefCell<T>>`                    |
| Multi-threaded + mutability        | `Arc<Mutex<T>>` or `Arc<RwLock<T>>` |

In short:

> `RefCell<T>` enables *interior mutability* — `Rc<RefCell<T>>` enables *shared, interiorly mutable ownership*.

---

#### Interior Mutability in the Standard Library

The interior mutability pattern isn’t just for `RefCell<T>` — it’s a **core design technique** in Rust.

Here are other types that use it internally:

| Type         | Interior Mutability Mechanism  | Use Case                    |
| ------------ | ------------------------------ | --------------------------- |
| `Cell<T>`    | Copy-based interior mutability | For small, Copy types       |
| `RefCell<T>` | Borrow-tracking at runtime     | For dynamic mutable borrows |
| `Mutex<T>`   | OS-based locking               | For threads                 |
| `RwLock<T>`  | Read-write locking             | For concurrent reads        |

---

### Summary

| Concept                 | Description                                  |
| ----------------------- | -------------------------------------------- |
| **Interior mutability** | Allows mutation through immutable references |
| **`RefCell<T>`**        | Checks borrowing rules at runtime            |
| **`Rc<RefCell<T>>`**    | Shared ownership + interior mutability       |
| **Safety tradeoff**     | Compile-time → runtime borrow checks         |
| **Common uses**         | Graphs, trees, caches, observers             |


### Final Example: Shared Counter

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let counter = Rc::new(RefCell::new(0));

    let c1 = Rc::clone(&counter);
    let c2 = Rc::clone(&counter);

    *c1.borrow_mut() += 1;
    *c2.borrow_mut() += 2;

    println!("Final count = {}", counter.borrow());
}
```

Output:

```
Final count = 3
```

Here:

* `Rc` lets us share ownership of the counter
* `RefCell` lets us mutate the value inside, safely

---

### Key Takeaway

> **`RefCell<T>` enables controlled interior mutability in single-threaded code.**
> When combined with `Rc<T>`, you can build shared, mutable data structures safely — with runtime checks 
  replacing compile-time restrictions.

Youe have covered
* ✅ `Rc<T>` — shared ownership with reference counting
* ✅ `RefCell<T>` — interior mutability with runtime borrow checks
* ✅ `Rc<RefCell<T>>` — shared and mutable data

But… these tools are **so powerful** that if you’re not careful, they can create **reference cycles** — 
situations where memory can never be freed.

Let’s explore how that happens and how to fix it.

---

## Tutorial: Reference Cycles and Memory Leaks

To cover:

* How reference cycles occur with `Rc<T>`
* Why Rust doesn’t prevent them at compile time
* How to detect them conceptually
* How to break them using **`Weak<T>`**


### The Problem: Reference Counting Without End

`Rc<T>` keeps a **strong reference count** (how many `Rc` clones exist).
When that count hits **zero**, the value is dropped.

But if two (or more) values **own each other**, their reference counts never hit zero.
That’s a **reference cycle**, and it leaks memory.


###  Building a Cycle by Accident

Let’s look at an example with a linked list.

```rust
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

use List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, RefCell::new(Rc::new(Nil))));
    let b = Rc::new(Cons(10, RefCell::new(Rc::clone(&a))));

    if let Cons(_, ref link) = *a {
        *link.borrow_mut() = Rc::clone(&b); // a points to b — cycle created!
    }

    println!("a strong = {}, weak = {}", Rc::strong_count(&a), Rc::weak_count(&a));
    println!("b strong = {}, weak = {}", Rc::strong_count(&b), Rc::weak_count(&b));
}
```

###  What happens

```
a → b → a
```

Both `a` and `b` point to each other using `Rc`.
Their reference counts never reach zero — so they’re **never dropped**.

Output:

```
a strong = 2, weak = 0
b strong = 2, weak = 0
```

The program ends, but their heap memory is **leaked** (not freed).
=> Rust guarantees *memory safety*, but not *leak prevention*.

---

### Why This Happens

Each `Rc<T>` increases the **strong reference count** of the value it points to.
When the count is nonzero, Rust won’t free the memory.

In a cycle, each value holds another `Rc`, keeping each other alive forever — even though they’re both unreachable to the program.

---

### Detecting Leaks

Rust doesn’t prevent or automatically detect cycles, because it would require runtime garbage collection.
But you can:

* Watch reference counts (`Rc::strong_count`)
* Use external crates like [`leak`](https://docs.rs/leak) or [`valgrind`](https://valgrind.org/)
* Be mindful when designing data structures that can form graphs or trees.

---

### The Solution: `Weak<T>`

Rust’s answer: **weak references**.

`Rc<T>` actually maintains **two counters**:

* **Strong count:** how many owners exist (controls when to drop the data)
* **Weak count:** how many *non-owning* references exist (for observation only)

A `Weak<T>` does **not** affect the strong count.
If only weak references remain, the data is dropped.

You can create one using `Rc::downgrade()`.

---

### Converting from `Rc<T>` to `Weak<T>`

```rust
use std::rc::Rc;

fn main() {
    let strong = Rc::new(5);
    let weak = Rc::downgrade(&strong);

    println!("strong = {}, weak = {}", Rc::strong_count(&strong), Rc::weak_count(&strong));

    {
        if let Some(value) = weak.upgrade() {
            println!("Weak upgraded: {}", value);
        } else {
            println!("Value already dropped");
        }
    }

    drop(strong);

    if let Some(_) = weak.upgrade() {
        println!("Still alive!");
    } else {
        println!("Now it's gone.");
    }
}
```

Output:

```
strong = 1, weak = 1
Weak upgraded: 5
Now it's gone.
```

The `Weak` pointer lets you *peek* at the data (using `upgrade()`), but not keep it alive.

---

###  Using `Weak<T>` to Prevent Cycles

Let’s revisit the **tree** example from the Rust book.
A parent should not “own” its children, and children shouldn’t “own” their parent — that’s a perfect use for `Weak<T>`.

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    // Set leaf’s parent to branch using Weak
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
```

Output (no cycle!)

```
leaf parent = Some(Node { value: 5, ... })
branch strong = 1, weak = 1
leaf strong = 2, weak = 0
```

When both go out of scope:

* The `Weak` reference in the leaf doesn’t prevent the branch from dropping.
* Memory is freed correctly.

---

### Diagram: Strong vs Weak

```
[Parent] Rc<Node>  (strong count = 1)
     ↑
     │ Weak<Node>
[Child] Rc<Node>   (strong count = 2)
```

* Strong arrows → own the data (keep it alive)
* Weak arrows → observe the data (don’t keep it alive)

---

### Summary

| Concept                  | Description                                                |
| ------------------------ | ---------------------------------------------------------- |
| **Reference cycle**      | When `Rc<T>`s point to each other, preventing deallocation |
| **Memory safety**        | Still guaranteed — but memory can leak                     |
| **`Weak<T>`**            | Non-owning reference that breaks cycles                    |
| **`Rc::downgrade(&rc)`** | Creates a `Weak<T>`                                        |
| **`Weak::upgrade()`**    | Returns `Option<Rc<T>>` if value still exists              |

---

#### Final Example: Safe Tree Structure

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

    println!(
        "branch strong={}, weak={}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );
    println!(
        "leaf strong={}, weak={}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}
```

Output:

```
branch strong=1, weak=1
leaf strong=2, weak=0
```

No cycle — safe cleanup!

---

### Key Takeaway

> `Rc<T>` gives shared ownership, but you must be careful to avoid cycles.
> Use `Weak<T>` for **non-owning references** (like “parent” pointers) to prevent leaks.
> This pattern — `Rc<RefCell<T>>` with `Weak<T>` for back-references — is the foundation of safe, graph-like data structures in Rust.

