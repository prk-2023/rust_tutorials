# Common Collections: 

---

Includes all the main ideas (vectors, strings, hash maps), explain behind‑the‑scenes reasoning, show  
examples and highlight pitfalls & best practices. 

---

## Why “Collections” and What Makes Them Special

* Collections are data structures that can contain multiple values (not just a single value).

* Unlike fixed-size arrays or tuples (where sizes/types are known at compile time), collections are dynamic
  and stored **on the heap**, so they can grow or shrink at runtime. 

* The main collections covered in this chapter are:

  1. **Vectors** (`Vec<T>`)

  2. **Strings** (`String`)

  3. **Hash maps** (`HashMap<K, V>`)

Each collection has its own strengths, costs (memory, performance), and rules (ownership, borrowing, 
indexing, iteration). Choosing the right one is part of becoming effective in Rust. 

---

## 1. Vectors (`Vec<T>`)

*Vectors* are perhaps the most commonly used collection. Think of them as growable arrays: you can push, pop, 
index, iterate, etc.

### Creating Vectors

You can create an empty vector and later add elements:

```rust
    let mut v: Vec<i32> = Vec::new();
    v.push(10);
    v.push(20);
```

Because the empty vector has no elements initially, Rust can’t infer the `T` type, so we often need the 
explicit type annotation (`Vec<i32>`). ([jasonwalton.ca][2])

Alternatively, if you already know the contents, you can use the `vec!` macro:

```rust
    let v = vec![1, 2, 3];
```

Here the macro infers the element type from its arguments.

### Updating (Mutating) a Vector

* To mutate a vector, it must be declared `mut`.
* You can `push` to append to the end.
* You can `pop` to remove and return the last element (as `Option<T>`).

```rust
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    let last = v.pop();  // Option<i32>
```

### Reading Elements: Indexing vs `get`

You can access elements by index ( `&v[index]` ) or via `get(index)`.

```rust
    let v = vec![10, 20, 30, 40, 50];
    let third = &v[2];        // gets a reference to the element at index 2
    let maybe_fourth = v.get(3);  // returns Option<&i32>
```

* Using `[index]` will **panic at runtime** if the index is out of bounds.
* Using `get` returns `None` if out of bounds, instead of panicking.
* `get` is safer when you're not sure whether the index is valid.

One subtlety: combining references and mutation can create borrowing conflicts. For instance:

```rust
    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    v.push(6);   // ❌ error: cannot borrow `v` as mutable because it is also borrowed immutably
    println!("First is: {}", first);
    v.push(6); // previous borrow is no longer active and push is allowed
    // Review of borrowing:
    // When a variable is borrowed immutable ( & )
    // - If it was borrowed immutably then you can still read/access that variable data it points to 
    //   through immutabe reference.
    // - You cannot mutate the original variable while the immutable borrow is active 
    // - You cannot create a mutable borrow while any immutable borrow exists
    // When var is borrowed mutably (&mut ):
    //  - you get exclusive access ( read and mutate the data)
    //  - But no other borrow (mutable or immutable) are allowed at the same time 
    //  - Original Variable cannot be accessed elsewhere while the mutable borrow is active 
```

This fails because `first` holds an immutable reference to `v[0]`, and pushing might reallocate memory, 
invalidating that reference. Rust prevents the unsafe behavior. ([jasonwalton.ca][2])

### Iterating Over Vectors

You can loop over a vector:

```rust
    let v = vec![100, 32, 57];
    for val in &v {
        println!("{}", val);
    }
```

If you have a mutable vector and want to mutate elements:

```rust
    let mut v = vec![100, 32, 57];
    for val in &mut v {
        *val += 50;
    }
```

You need to dereference (`*val`) to modify the actual element. ([jasonwalton.ca][2])

### Heterogeneous Types Using Enums

Vectors require all elements to have the *same* type `T`. 

If you need to mix types, you can wrap them in an enum:

```rust
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    // let idx0 = &row[0]; // take the reference
    for idx in &row {
        match idx {
            SpreadSheetCell::Int(i) => println!("element is int {}", i),
            SpreadSheetCell::Text(i) => println!("element is String {}", i),
            SpreadSheetCell::Float(i) => println!("element is float {}", i),
        }
    }
```

Now `row` is `Vec<SpreadsheetCell>`. Each element is one of those variants. ([jasonwalton.ca][2])

### Dropping a Vector

When a vector goes out of scope, Rust drops (frees) it and calls drop on each element. 

Memory is freed on the heap just like other owned data. ([jasonwalton.ca][2])

---

## 2. Strings (`String` and `&str`)

Strings in Rust are more nuanced because of Unicode and encoding. In Rust:

* `&str` (string slice) is a borrowed string view, often pointing to a literal or part of a `String`.
* `String` is the growable, heap-allocated, mutable string type in the standard library.

### What Is a `String`?

A `String` is essentially a vector of bytes (`Vec<u8>`) that are guaranteed to form valid UTF‑8. 
You can mutate it, extend it, combine it, etc.  ([O'Reilly Media][3])

### Creating Strings

You can create an empty string:

```rust
    let mut s = String::new();
```

You can convert from a literal:

```rust
    let s = "hello".to_string();
```

or

```rust
    let s = String::from("hello");
```

These are equivalent in many contexts.

### Updating Strings

You can append to a `String` via:

* `push_str(&str)` — appends a string slice
* `push(char)` — appends a single character

```rust
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
```

String concatenation with the `+` operator also works, but with ownership patterns:

```rust
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // s1 is moved; s2 is borrowed
```

Here `s1` is consumed (moved), so you can’t use `s1` afterward. 
The `+` operator is defined roughly as `fn add(self, &str) -> String`. 
Rust also coerces `&String` to `&str` when needed. ([jasonwalton.ca][2])

If you need to concatenate multiple strings without taking ownership, `format!` is often more ergonomic:

```rust
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{s1}-{s2}-{s3}");
```

`format!` doesn’t take ownership of its inputs. ([jasonwalton.ca][2])

### Indexing and Slicing: Why It’s Tricky

Unlike vectors, `String` **cannot** be directly indexed with `[i]`. 
That’s because Rust strings are UTF‑8, and a single Unicode character may occupy multiple bytes. 
Indexing by byte could split an encoding, producing invalid data.

Instead you can slice strings in byte ranges — but you must ensure the slice boundaries align with valid 
character (UTF‑8) boundaries:

```rust
    let hello = "Здравствуйте";
    let s = &hello[0..4];  // gets a &str containing the first two Cyrillic chars
```

If the slice boundaries are invalid, the code will panic at runtime. ([jasonwalton.ca][2])

There are methods to iterate over strings safely in terms of characters or bytes:

* `.chars()` — iterate over `char` (Unicode scalar)
* `.bytes()` — iterate over raw bytes
* `.graphemes()` (via external crates) — iterate on user-perceived characters

Because string slicing and indexing are complex in Unicode, Rust intentionally disallows direct indexing to avoid subtle bugs.

---

## 3. Hash Maps (`HashMap<K, V>`)

Hash maps provide a way to associate keys with values — similar to dictionaries or maps in other languages.

### Basic Usage

You need to import it:

```rust
    use std::collections::HashMap;
```

Then:

```rust
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team = String::from("Blue");
    let score = scores.get(&team);  // Option<&i32>

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
```

Note: iteration order is arbitrary (not insertion order). If you need sorted or predictable order, consider 
`BTreeMap`. ([jasonwalton.ca][2])

### Ownership and HashMaps

When you insert keys and values into a `HashMap`, the map takes ownership of them (unless they are types 
that implement `Copy`). 

For example:

```rust
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // field_name and field_value moved and cannot be used afterward
```

If keys or values are references (e.g. `&str`), those references must remain valid for the lifetime of the 
map. That requires careful lifetime management. ([jasonwalton.ca][2])

For types that are `Copy` (like `i32`), inserting will copy the value, so you can still use the original.

### Updating a HashMap

#### Replacing existing value

If you insert a key that already exists, the old value is replaced:

```rust
    scores.insert(String::from("Blue"), 25);
```

#### Only inserting if key isn’t present (`entry` API)

One powerful idiom is using `entry()`:

```rust
    let mut map = HashMap::new();
    map.insert("Blue", 10);

    // entry returns an Entry enum: Vacant or Occupied
    let count = map.entry("Blue").or_insert(0);
    *count += 5;
```

* `or_insert(0)` returns a mutable reference to the value for the key, inserting `0` if the key was not 
  present.
* You can then mutate via `*count += 1`.

This pattern is especially useful for counting frequencies, aggregating data, or ensuring default values. 
([jasonwalton.ca][2])

**Example: Word counting**

```rust
    use std::collections::HashMap;

    fn main() {
        let text = "hello world wonderful world";
        let mut map = HashMap::new();

        for word in text.split_whitespace() {
            let count = map.entry(word).or_insert(0);
            *count += 1;
        }
        println!("{:?}", map);
    }
```

Here we split on white space, count each word’s occurrences, and insert or increment accordingly. 
([jasonwalton.ca][2])

### Hashing Functions & Custom Hashers

`HashMap<K, V>` uses a hashing function to map keys to buckets. 

Rust’s default hasher is not specified (it may change for security reasons). 
If you want deterministic hashes or custom behavior, you can supply your own hasher type by using 
`HashMap<K, V, S>` where `S` is a hasher builder. ([jasonwalton.ca][2])

---

## Summary & Tips

Here is a summary of what you should remember, along with best practices and warnings:

| Collection      | Use Case                        | Key Methods / Patterns                                  | Pitfalls / Considerations                                                                              |
| --------------- | ------------------------------- | ------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ |
| `Vec<T>`        | Ordered list of items, growable | `push`, `pop`, indexing, `get`, iteration               | Borrowing rules (no mutable and immutable references overlapping), reallocation invalidates references |
| `String`        | Mutable, UTF‑8 text data        | `push_str`, `push`, `+`, `format!`, `.chars()/.bytes()` | No direct indexing, slicing must respect UTF‑8 boundaries                                              |
| `HashMap<K, V>` | Key-value associations          | `insert`, `get`, `entry().or_insert()`, iteration       | Owning semantics (keys/values get moved), non-deterministic iteration order, custom hashers            |

### Additional Tips

1. **Choose safeness over convenience**: The fact that Rust disallows string indexing and prevents invalid
   references when vectors reallocate is part of the safety model.

2. **Prefer `get` over indexing when uncertainty exists**: For vectors, `get` gives you `Option<&T>`, which
   is safer.

3. **Use `entry` API smartly**: Many patterns (like counting, group-by) become concise and efficient.

4. **Leverage iteration APIs**: Methods like `.iter()`, `.iter_mut()`, `.into_iter()` provide flexibility
   over how you traverse or consume collections.

5. **Be mindful of ownership moves**: After inserting into a map or concatenating via `+`, you may no longer
   own those values.

6. **If order matters**: Use other maps (e.g. `BTreeMap` or external ordered map crates) instead of 
   `HashMap`.

---

[1]: https://docs.w3cub.com/rust/book/ch08-00-common-collections.html?utm_source=chatgpt.com "8. Common Collections - Rust - W3cubDocs"
[2]: https://jasonwalton.ca/rust-book-abridged/ch08-common-collections/?utm_source=chatgpt.com "8 - Common Collections | The rs Book"
[3]: https://www.oreilly.com/library/view/the-rust-programming/9781492067665/xhtml/ch08.xhtml?utm_source=chatgpt.com "The Rust Programming Language - The Rust Programming Language [Book]"
