# References and Lifetimes:


Lifetimes are another kind of **generic**, rather then ensuring that a type has a behaviour we want,
lifetimes ensure that references are valid as long as we need them to be. 

Most of the times lifetimes are implicit and inferred, just like types are inferred. We are only required to
annotate types when multiple types are possible. Similarly we have to annotate lifetimes when the lifetimes
of references could be related in a few different ways. 

==> Rust requires us to **annotate the relationship using generic lifetime parameter** this is to ensure the
actual references used at runtime will definitely be valid.

## Borrow Checker: 

Rust compiler has a *borrow Checker* which compares scope to determine whether all borrows are valid. 
```rust
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {r}");   //          |
}                         // ---------+

```
- the code annotated the lifetime of `r` with `'a` and the lifetime of `x` with `'b`. 
  The inner `'b` block is much smaller than the outer `'a` lifetime block. 
  At compile time, Rust compares the size of the two lifetimes and sees that `r` has a lifetime of `'a` but
  that it refers to memory with a lifetime of `'b`. 
  The program is rejected because `'b` is shorter than `'a`: the subject of the reference doesn’t live as long 
  as the reference.

Fixed code:
```rust 
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {r}");   //   |       |
                          // --+       |
}                         // ----------+
```
- `x` has lifetime `'b` which in this case is larger then `'a` ==> `r` can reference `x` because rust knows
  that the reference in `r` will always be valid while `x` is valid.

## Validating References with Lifetimes — A Tutorial

---

## Why Lifetimes Matter

In Rust, **references** (`&T`) must always be valid while you use them to avoid **dangling references** 
(pointers to invalid memory).

Rust uses **lifetimes** to track how long references are valid. 
Compiler enforces these lifetimes at compile time to guarantee memory safety **without a garbage collector**.

---

## The Problem Without Lifetimes

Imagine you have this function:

```rust
fn longest_string(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

Rust will **reject** this code:

```
error[E0106]: missing lifetime specifier
```

Why? Because Rust can't tell if the returned reference is valid after the function ends—it's unclear whether 
it relates to `x` or `y`.

---

## Introducing Lifetime Annotations

To fix this, we add **lifetime parameters** to tell Rust how the lifetimes of inputs and outputs relate:

```rust
fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```
* `'a` is a **generic lifetime parameter**
  The term generic here in the context of lifetime parameter like `'a` means that the function 
  `longest_string` can operate correctly with **any set of input reference** as long as they **all live for
  at least the duration** of the lifetime represented by `'a`.

* `&'a str` means "a reference valid at least as long as lifetime `'a`"

* The return type `&'a str` means the returned reference is valid for the same lifetime `'a`

This tells Rust: *"The returned reference will live at least as long as both `x` and `y` references."*

NOTE: You only need to explicitly specify lifetimes in Rust when you are dealing with References 
(&var or &mut var).

Why Lifetimes Are Only Needed for References:
1. Ownership and Memory Safety
Rust's core memory safety model is based on Ownership. 
Lifetimes are an auxiliary system that works alongside ownership, but they only govern the validity of 
borrowed data:
- Owned Data (Non-References): 
    Types that own their data (like String, Vec<T>, i32, or standard structs) manage their own allocation 
    and deallocation. 
    When a variable owning data goes out of scope, the data is dropped. 
    The compiler knows exactly when the data is valid, so no lifetime annotations are necessary.

- Borrowed Data (References): 
    A reference (&'a T) is a non-owning pointer to data that is owned by someone else. 
    The compiler needs a way to ensure that this reference never outlives the data it points to 
    (i.e., that you can't access memory after it's been freed). 
    This is the exact job of the lifetime parameter.

2. The Lifetime Elision Rules

In many common scenarios, the Rust compiler can infer the correct lifetimes using a set of rules called 
the Lifetime Elision Rules. Because of these rules, you don't even need to specify lifetimes for references 
in most function signatures, such as:

    - Methods (the self reference often determines the output lifetime).

    - Functions with one input reference (the output lifetime is assumed to match the input lifetime).

You must specify lifetimes only in cases where the compiler cannot determine how the input lifetimes relate 
to the output lifetime. The longest_string function is a classic example:
```rust 
fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    // ...
}
```
In this case, the compiler cannot know whether the returned reference comes from x or y. 
By applying the generic lifetime 'a to all three, you tell the compiler: 
"The returned reference must live at least as long as the shorter of the two input references."

---

## Lifetime Syntax Refresher

* `'a` is the name of a lifetime parameter.
* Lifetime parameters start with a `'` (single quote) followed by a lowercase name.
* They work like generics, but for lifetimes.

---

## Example: Longest String Function

```rust
fn longest_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let s1 = String::from("short");
    let s2 = String::from("much longer string");

    let result = longest_string(&s1, &s2);
    println!("The longest string is {}", result);
}
```

This works because `s1` and `s2` live long enough for `result` to safely borrow from them.

---

## Lifetime Errors You Might Encounter

```rust
fn main() {
    let result;
    {
        let s1 = String::from("hello");
        let s2 = String::from("world");

        result = longest_string(&s1, &s2);
    } // s1 and s2 dropped here

    println!("Longest string is {}", result); // ERROR: result references dropped data!
}
```

The compiler **won’t allow this** because `result` would be referencing data that’s already been dropped.

---

## Structs with References and Lifetimes

When a struct holds references, you must annotate lifetimes on the struct:

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce(&self, announcement: &str) {
        println!("Attention please: {}", announcement);
    }

    fn get_part(&self) -> &'a str {
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().unwrap();

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("Excerpt: {}", excerpt.get_part());
}
```

Here `'a` ties the lifetime of the reference stored in `ImportantExcerpt` to the lifetime of the data it 
points to.

---

## Lifetime Elision Rules (Simplify Your Code)

Rust often **infers lifetimes** in function signatures using these three rules:

1. Each parameter that is a reference gets its own lifetime parameter.
2. If there’s exactly one input lifetime, that lifetime is assigned to all output lifetimes.
3. If there are multiple input lifetimes and one of them is `&self` or `&mut self` (methods), the output 
   lifetime is assigned to `self`.

Because of these, many functions don’t need explicit lifetime annotations.

---

## Summary

| Concept                  | Description                                        |
| ------------------------ | -------------------------------------------------- |
| **Lifetimes**            | Named scopes for how long references are valid     |
| `'a`                     | Syntax for a generic lifetime parameter            |
| **Lifetime annotations** | Link the lifetimes of parameters and return values |
| Structs with references  | Require lifetime annotations on struct             |
| Lifetime elision rules   | Compiler rules to infer lifetimes automatically    |

---

##  Quick Quiz

1. Why does Rust require lifetime annotations in functions that return references?
2. What does the lifetime `'a` represent?
3. What happens if you return a reference to a value that goes out of scope?
4. How do lifetime annotations relate to structs that hold references?

---

# Lifetime Exercises & Examples
---

## Exercise 1: Fix Lifetime Errors

Given this broken code, fix it by adding the correct lifetime annotations:

```rust
fn get_first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn main() {
    let string1 = String::from("hello world");

    let word = get_first_word(&string1);

    println!("First word: {}", word);
}
```

---

### Hint:

Rust will complain about missing lifetime specifiers on `get_first_word`. 
Try adding a lifetime parameter to the function.

---

## Exercise 2: Struct with Reference

Complete the following struct definition and implementation by adding lifetimes, so it compiles and runs:

```rust
struct CodeSnippet {
    code: &str,
}

impl CodeSnippet {
    fn new(code: &str) -> CodeSnippet {
        CodeSnippet { code }
    }

    fn print_code(&self) {
        println!("{}", self.code);
    }
}

fn main() {
    let snippet_text = String::from("fn main() { println!(\"Hello\"); }");
    let snippet = CodeSnippet::new(&snippet_text);
    snippet.print_code();
}
```

---

### Hint:

Add a lifetime parameter to `CodeSnippet` and its methods so that the compiler knows how long `code` will 
be valid.

---

## Exercise 3: Longest Programming Language Name

Implement the function `longest_name` that takes two string slices referencing programming language names, 
and returns the longest one. Add lifetimes annotations properly.

```rust
fn longest_name(x: &str, y: &str) -> &str {
    // TODO: implement
}

fn main() {
    let lang1 = "Rust";
    let lang2 = "JavaScript";

    let longest = longest_name(lang1, lang2);
    println!("Longest language name is {}", longest);
}
```

---

## Exercise 4: Reference in Struct Method

Given the following struct, implement a method `code_length` that returns the length of the code slice 
stored in the struct.

```rust
struct CodeSnippet<'a> {
    code: &'a str,
}

impl<'a> CodeSnippet<'a> {
    fn code_length(&self) -> usize {
        // TODO: implement
    }
}

fn main() {
    let code_text = String::from("fn main() {}");
    let snippet = CodeSnippet { code: &code_text };

    println!("Code length: {}", snippet.code_length());
}
```

---

## Solutions Outline

* **Exercise 1:** Add a lifetime parameter `'a` on the function signature and return type.

* **Exercise 2:** Add lifetime `'a` to struct definition and methods.

* **Exercise 3:** Add lifetime parameter `'a` linking inputs and output.

* **Exercise 4:** Return `self.code.len()` from method.

---

## Exercise 1: Fix Lifetime Errors in `get_first_word`

### Step 1: Understand the error

Rust complains:

```
error[E0106]: missing lifetime specifier
```

because the function returns a reference (`&str`), but the compiler doesn’t know how long that reference will be valid relative to the input.

---

### Step 2: Add a lifetime parameter

We need to tell Rust that the returned reference will live **at least as long as** the input reference.

```rust
fn get_first_word<'a>(s: &'a str) -> &'a str {
    // body unchanged
}
```

---

### Step 3: Complete fixed function

```rust
fn get_first_word<'a>(s: &'a str) -> &'a str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
```

---

### Step 4: Test in `main`

```rust
fn main() {
    let string1 = String::from("hello world");
    let word = get_first_word(&string1);
    println!("First word: {}", word);
}
```

This now compiles and prints:

```
First word: hello
```

---

## Exercise 2: Add Lifetimes to Struct Holding a Reference (`CodeSnippet`)

---

### Step 1: Identify the problem

Your struct holds a reference:

```rust
struct CodeSnippet {
    code: &str,
}
```

Rust **requires a lifetime annotation** here so it knows how long `code` will be valid.

---

### Step 2: Add a lifetime parameter to the struct

Add a generic lifetime `'a`:

```rust
struct CodeSnippet<'a> {
    code: &'a str,
}
```

This means: *“The reference `code` inside `CodeSnippet` is valid at least as long as lifetime `'a`.”*

---

### Step 3: Update the `impl` block to use the lifetime `'a`

```rust
impl<'a> CodeSnippet<'a> {
    fn new(code: &'a str) -> CodeSnippet<'a> {
        CodeSnippet { code }
    }

    fn print_code(&self) {
        println!("{}", self.code);
    }
}
```

* The `new` method takes a `&'a str` and returns a `CodeSnippet<'a>`.
* `print_code` borrows `self` immutably, no lifetime annotations needed here.

---

### Step 4: `main` function works as expected

```rust
fn main() {
    let snippet_text = String::from("fn main() { println!(\"Hello\"); }");
    let snippet = CodeSnippet::new(&snippet_text);
    snippet.print_code();
}
```

---

### Recap:

The key is that the struct’s lifetime parameter `'a` tells Rust the struct cannot outlive the string slice it holds a reference to.

---
## Exercise 3: Implement `longest_name` with Lifetime Annotations

---

### Step 1: Understand the function signature

You have:

```rust
fn longest_name(x: &str, y: &str) -> &str {
    // TODO
}
```

The function returns a reference to one of the input string slices. Rust needs to know **how the lifetimes of inputs and output relate**.

---

### Step 2: Add a lifetime parameter `'a`

Add `'a` to the inputs and output to tell Rust:

*“The returned reference lives at least as long as both inputs.”*

```rust
fn longest_name<'a>(x: &'a str, y: &'a str) -> &'a str {
    // TODO
}
```

---

### Step 3: Implement function logic

```rust
fn longest_name<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
```

---

### Step 4: Test in `main`

```rust
fn main() {
    let lang1 = "Rust";
    let lang2 = "JavaScript";

    let longest = longest_name(lang1, lang2);
    println!("Longest language name is {}", longest);
}
```

This compiles and prints:

```
Longest language name is JavaScript
```

---

### Summary:

* The lifetime `'a` links input lifetimes to the output lifetime.
* This ensures the returned reference is always valid.

---
## Exercise 4: Implement Method Returning Length of Referenced Code Slice

---

### Step 1: Review the struct and method signature

```rust
struct CodeSnippet<'a> {
    code: &'a str,
}

impl<'a> CodeSnippet<'a> {
    fn code_length(&self) -> usize {
        // TODO
    }
}
```

* `CodeSnippet` holds a reference to a string slice with lifetime `'a`.
* `code_length` method takes `&self` and should return the length of `code`.

---

### Step 2: Implement `code_length`

You simply return the length of the string slice referenced by `self.code`:

```rust
fn code_length(&self) -> usize {
    self.code.len()
}
```

---

### Step 3: Test in `main`

```rust
fn main() {
    let code_text = String::from("fn main() {}");
    let snippet = CodeSnippet { code: &code_text };

    println!("Code length: {}", snippet.code_length());
}
```

Output:

```
Code length: 12
```

---

### Summary

* The lifetime `'a` ensures the reference `code` inside `CodeSnippet` is valid as long as `snippet` exists.
* The method borrows `self` immutably, no additional lifetime annotations needed here.
* You access `self.code` and call `.len()` just like any other `&str`.

---

# You did it!

You’ve now worked through lifetime annotations for:

* Functions returning references tied to input lifetimes
* Structs holding references with lifetime parameters
* Methods operating on those structs

---


## Lifetime Elision Rules :

Elision Rules are set of Implicit Rules that the compiler uses to automatically infer lifetime annotations,
making code cleaner and easier to write. This applies when the programmer doesn't explicitly provide a
lifetime, allowing the compiler to add them in common scenario for function signature and references. 

### How Elision rules work:

The Elision Rules are applied by the compiler to **function parameters** and **return types** to infer 
lifetimes in a few common cases:

- **Unique input lifetimes**: If there is exactly one input lifetime, that lifetime is assigned to all
  elided output lifetimes. 

- Multiple input lifetimes: If there are multiple input lifetimes, but one of them is `&self` or  
  `&mut self`, the lifetime of `self` is assigned to all elided output lifetimes. 

- No output lifetime: If the function does not return a reference, there is no output lifetime to elide. 

Ex: Without Elision:

```rust 
fn annotated_input<'a> (x: &'a i32) {
    println!("annoted_input : {} ", x);
}
```
 With Elision: the compiler infers a lifetime for &i32 and adds it automatically, resulting in the same
 signature as the annotated version:

 ```rust 
 fn elided_input(x: i32) {
    println!("annoted_input : {} ", x);
 }
 ```
 
 This reduced boilerplate, Improves readability, and Ensure correctness.

 Recap:

 In function signature Rust can automatically infer lifetimes in many common cases called as Elision Rules: 

 - Each part that's a reference gets its own lifetime parameter. 
 - If there is only one input lifetime, it is assigned to all output references.
 - If there are multiple input lifetimes, but one of them is `&self` or `&mut self` that lifetime is
   assigned to all output references. 

example:

```rust 
#[derive(Debug)]
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}

#[derive(Debug)]
struct Library<'a> {
    books: Vec<Book<'a>>,
}

impl<'a> Library<'a> {
    fn new() -> Self {
        Library { books: Vec::new() }
    }

    fn add_book(&mut self, title: &'a str, author: &'a str) {
        let book = Book { title, author };
        self.books.push(book);
    }

    // This function returns the book with the longer title
    fn book_with_longer_title<'b>(&'b self, b1: &'b Book<'a>, b2: &'b Book<'a>) -> &'b Book<'a> {
        if b1.title.len() >= b2.title.len() {
            b1
        } else {
            b2
        }
    }
}

fn main() {
    let title1 = String::from("The Rust Programming Language");
    let author1 = String::from("Steve Klabnik and Carol Nichols");

    let title2 = String::from("Rust in Action");
    let author2 = String::from("Tim McNamara");

    // Titles and authors must live long enough for the library to borrow them
    let mut library = Library::new();

    library.add_book(&title1, &author1);
    library.add_book(&title2, &author2);

    let book1 = &library.books[0];
    let book2 = &library.books[1];

    let longer_title_book = library.book_with_longer_title(book1, book2);

    println!("Book with the longer title: {:?}", longer_title_book);
}
```
- **Book<'a>** : The `'a` lifetime means the `title` and `author` must live as long as the struct is alive.
```rust 
struct Book<'a> {
    title: &'a str,
    author: &'a str,
}
```

- **Library<'a>** : Holds `Vec<Book<'a>`, so all books borrow strings that live at least as long as `'a`. 

- **fn book_with_longer_title<'b>** : Here we have another lifetime `'b` for the function's arguments, which
  must be valid as long as `'a` ( inpractice: `'b`: `'a` must hold if `'b` is longer )

- **No Elision in Struct**: Rust does not apply lifetime elision to struct fields. (Lifetime must be explicit)

- **Elison in `add_book` method**: 

    `fn add_book( &mut self, title: &'a str, author: &'a str)`

    Lifetime is needed as `self` is mutably borrowing `self.book` and input need to match the `'a` lifetime
    of the struct.


=> Elision Rules are applied to **functions signature** ( that includes parameters and return type) and not
directly to **struct fields**.

=> **Struct Field Lifetimes**: Every reference (&T) used as a field within a struct must have an explicit 
   lifetime parameter. This lifetime parameter must also be declared on the struct itself.
   ```rust
        struct Example<'a> {
            // The reference `name` borrows its data for the lifetime 'a.
            name: &'a str,
            // The reference `data` also borrows for the lifetime 'a.
            data: &'a [u8],
            // Non-reference fields don't need lifetime parameters.
            id: u32,
        }
   ```

