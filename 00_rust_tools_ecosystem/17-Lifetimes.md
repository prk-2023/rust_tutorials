# Rust Lifetimes:

---
Why do we we need Lifetimes in rust?

Lifetimes are a way to ensure memory safety by making sure that references do not overlive the data they point to.

They are crucial concept for the borrow checker to prevent *dangling references* ( that is references to 
data that has been deallocated or no longer valid.)

```rust 
// Example without lifetime ( this program does not compile )
fn main () {
    let r;
    {
        let x = 42;
        r = &x; // Error: `x` does not live longer enough 
    }
    println!("{}", r); // using r after x is out of scope 
}
```
In the above code after `x` gets dropped `r` would point to invalid memory address, which is a *dangling references

Rust does not allow this because it would cause undefined behaviour. 
=> The Borrow checker will prevent this from compiling.

Fix this:

To fix this Rust Introduces a concept of lifetime annotations to tell rust how long the reference is valid:

```rust 
fn main() {
    let x = 42;
    let r: &i32 = &x; // r is valid as long as x is in scope 
    println!("{}", r) // prints 42
}
```
Here reference `r` is allowed because `x` will live long enough for `r` to be used.

Explicit Lifetime Annotation in Functions:

```rust 
fn longest<'a>( s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else { s2 }
}
fn main() {
    let string1 = String::from("Hello");
    let string2 = String::from("World!");

    let result = longest(&string1, &string2);
    println!("The longest string is {}", result);
}
```
- function longest  takes 2 arguments of string slices `s1` and `s2` both with same lifetime `'a` and return
  a reference that lives for the same lifetime `'a`
- the lifetime annotation `'a` ensures that the reference returned by `longest` will not outlive the input 
  references, `s1` and `s2`.
- The compiler can infer lifetimes automatically in some cases, but you can explicitly specify them when the 
  function's signature requires it.

Lifetime: Makes sure 
1. No dangling references: A reference must never outlive the data it points to.
2. Safe Memory management: Borrow checker ensures that at any point in time, wither one part of the code has 
   a mutable reference or there are multiple immutable references, but not BOTH.

---

## Introduction
Rust’s ownership model ensures memory safety by tracking how data is borrowed. 
Lifetimes are annotations that tell the compiler how long references are valid, ensuring they don’t outlive 
the data they point to. 
Lifetimes are mostly implicit, but explicit annotations are needed in certain cases, like function 
signatures or structs.

i.e Lifetimes are compile-time annotations, used to describe `how long a references` are valid.

NOTE: 
    - They are not used in runtime
    - They do not control when memory is freed. 
    - They are used by the compilers `Borrow checker` to ensure memory safety.

### Why are lifetimes required?
Rust doesn't use a GC - It enforces `strict ownership and borrowing rules` to
    - Prevent dangling references ( pointers )
        Ensures references do not point to `deallocated` memory.
    - Enable safe-borrowing: 
        Enables multiple parts of the code to access data without ownership conflicts.
    - compile time checks: Rust Borrow checker uses lifetimes to enforce safety at compile time.
    - Avoid use-after-free bugs
    - Ensure Safe memory access.

[recap:
 1. ownership: every value has a single owner, and when owner goes out of scope the value is dropped.
 2. Borrowing: You can create references (&T for immutable and &mut T for mutable) to access data without
    taking ownership.
 3. Lifetimes: A way to describe the scope for which the reference is valid.
 ]

#### Implicit Lifetimes:

Rust often infers lifetimes automatically. 
For example:
```rust 
    fn main() {
        let x = 5;
        let r = &x; // r borrows x
        println!("r: {}", r); // r is valid here
    } // x and r go out of scope
```
Here Rust infers that the references `r` is valid as long as `x` exists.


#### When Explicit Lifetimes Are Needed:

Explicit lifetime annotations are required when:

- A function takes multiple references as parameters, and Rust can’t infer their relationship.
- A struct holds references, and the compiler needs to know their lifetimes.


## Lifetime Annotations

Lifetime annotations use the syntax 'a (or any identifier, like 'b, 'xyz), where the apostrophe denotes a 
lifetime. 

They are typically used in function signatures or structs to specify how long references live relative to 
each other.

### Syntax

* &'a T: A reference to type T with lifetime 'a.
* &'a mut T: A mutable reference to type T with lifetime 'a.

==> Example: Function with Lifetimes
Consider a function that returns the longer of two strings:

```rust 
    fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s1
        } else {
            s2
        }
    }
```
- `'a` is a lifetime parameter, declared in angle brackets `< >`.

- `s1` and `s2` are references with lifetime `'a`.

- The return value is a reference with same lifetime `'a`, ensuring it doesn’t outlive the input references.

==> Using the Function

```rust 
    fn main() {
        let string1 = String::from("short");
        let string2 = String::from("longer");

        let result = longest(&string1, &string2);
        println!("The longest string is: {}", result);
    }
```
- This works because both `string1` and `string2` live long enough for result to be valid.

==> Common Error: Dangling Reference

If a reference outlives the data it points to, Rust will reject the code:
```rust 
    fn main() {
        let result;
        {
            let string1 = String::from("short");
            result = longest(&string1, "longer");
        } // string1 is dropped here
        println!("The longest string is: {}", result); // Error: result points to dropped data
    }
```
- Compiler will catch this, as result would reference `string1`, which is dropped when the inner scope ends.

==> Lifetimes in Structs

When a struct contains references, you must specify their lifetimes:

    ```rust 
        struct Book<'a> {
            title: &'a str,
            author: &'a str,
        }

        fn main() {
            let author = String::from("Jane Doe");
            let book = Book {
                title: "Rust Guide",
                author: &author,
            };
            println!("Book: {} by {}", book.title, book.author);
        }
    ```

Here, the `'a` lifetime ensures that `title` and `author` references live at least as long as the `Book` 
instance.

### Lifetime Elision Rules

Rust has rules to reduce the need for explicit lifetime annotations in simple cases. 
These are called lifetime elision rules:

1. Each input reference gets its own lifetime:

    `fn foo(x: &i32) // Inferred as fn foo<'a>(x: &'a i32)` 

2. If there’s one input lifetime, the output gets the same lifetime:
    
    `fn foo(x: &i32) -> &i32 // Inferred as fn foo<'a>(x: &'a i32) -> &'a i32`

3. If a method has `&self` or `&mut self`, the output lifetime matches `self`:

    `
    struct Example;
    impl Example {
        fn get_value(&self, x: &i32) -> &i32 { x } // self’s lifetime applies
    }
    `
These rules cover many cases, so you often don’t need explicit annotations.

### Multiple Lifetimes

When dealing with multiple references, you may need multiple lifetime annotations:

```rust 
    fn compare<'a, 'b>(s1: &'a str, s2: &'b str) -> &'a str {
        s1 // Return s1 with lifetime 'a
    }
```
Here, `'a` and `'b` allow `s1` and `s2` to have different lifetimes, but the return value is tied to `'a`.


### The Static Lifetime

The `'static` lifetime means a reference lives for the entire program duration. 
String literals, for example, are `'static` :

    `let s: &'static str = "I live forever!";`

Use `'static` only when necessary, as it implies the reference never goes out of scope.

### Practical Example: Parsing Data

Let’s create a function that extracts the first word from a string, using lifetimes:

```rust 
    fn first_word<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        s
    }

    fn main() {
        let text = String::from("hello world");
        let word = first_word(&text);
        println!("First word: {}", word);
    }
```

- The function returns a slice of `s`, so the return value’s lifetime is tied to `s’s` lifetime ('a).
- This ensures the returned slice is valid as long as text exists.

### Common Pitfalls and Solutions

1. Overly Restrictive Lifetimes

If you use the same lifetime for unrelated references, you may restrict their usage unnecessarily:

```rust 
    fn bad_longest<'a>(s1: &'a str, s2: &str) -> &'a str {
        if s1.len() > s2.len() { s1 } else { s2 } // Error: s2’s lifetime isn’t 'a
    }
```
Solution: Use separate lifetimes or rely on elision if possible.

2. Missing Lifetime in Structs

Forgetting to annotate lifetimes in structs causes errors:

```rust 
    struct BadBook {
        title: &str, // Error: missing lifetime specifier
    }
```

Solution: Always specify lifetimes for references in structs:

    ```
    struct GoodBook<'a> {
        title: &'a str,
    }
    ```

Exercises

1. Write a function that takes two string slices and returns the shorter one, using explicit lifetimes.
2. Create a `struct Person` with a `name` field as a string reference, and write a function to create a 
   `Person` from a `String`.
3. Modify the first_word function to return an empty string slice ("") if the input is empty.
----------------------------------------------
1. 

```rust 
    fn shortest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
        if s1.len() > s2.len() {
            s2
        } else {
            s1
        }
    }
```
- `'a` is a lifetime parameter, declared in angle brackets `< >`.

- `s1` and `s2` are references with lifetime `'a`.

- The return value is a reference with same lifetime `'a`, ensuring it doesn’t outlive the input references.

----------------------------------------------
2. 
```rust 
    struct Person<'a> {
        name: &'a str,  //reference to a string slice
    }
    fn create_person_from_string <'a> (input: &'a String) -> Person<'a> {
        Person{ name: input.as_str()}
    }

    fn main() {
        let name_string = String::from("pickachoo");
        //create person by borrowing from name_string;
        let person = create_person_from_string(&name_string)

        println!("Person;s name is: {}", person.name);
    }
```
----------------------------------------------
3. 
Modify the first_word function to return an empty string slice ("") if the input is empty.

```rust 
    fn first_word<'a>(s: &'a str) -> &'a str {

        if s.is_empty() {
            return ""; // this will return "" string slice.
        }

        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        s
    }

    fn main() {
        let text = String::from("hello world");
        let word = first_word(&text);
        println!("First word: {}", word);
    }
```

- The function returns a slice of `s`, so the return value’s lifetime is tied to `s’s` lifetime ('a).
- This ensures the returned slice is valid as long as text exists.

Further Reading

Rust Book: Validating References with Lifetimes
Rust Reference: Lifetime Elision

----------------------------------------------


