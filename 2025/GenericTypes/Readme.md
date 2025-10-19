# Rust Generics:

## Introduction:

Generic types let you write code that works for multiple data types, without duplicating logic.

i.e: Generic *types* ( generic data types ) in Rust are way to write flexible code that can work with
various data types without needing to specify a concrete type at the time of definition. 

=> Generics are essentially **placeholders** for types.

### What Generic Type Means:

A generic type is a type parameter, often denoted by a single uppercase letter like **T** (for "Type") or
**U**, that is declared in a function, *struct*, *enum*, or *trait* definition. 
When the code is used, these placeholders are replaced by concrete types (like `i32`, `String`, or a 
custom `struct` ).
Example:
    - Cookie Cutter: Defines the structure or logic but not the material its made with.
    - Cookie dough: Gingerbread dough, cake dough.. play dough.. we use concrete type (i32, u32,
    String,struc )
    - Resulting Cookie: Specific implementation that works with that type.

Generic: Code reuse and Abstraction. 

1. Code that works for multiple data types:

A Single function that can sort array of integers or sort array of strings, the function takes `T` type
parameter.

Without generics we would have 

```rust
    fn get_largest_i32(list: &[i32]) -> i32 { /* logic for i32 */ }
    fn get_largest_char(list: &[char]) -> char { /* logic for char */ }
```

With generics we have one single definition:

```rust 
    fn get_largest<T: PartialOrd + Copy>(list: &[T]) -> T { /* logic for any type T */ }
    // T is the generic type parameter.
    // PartialOrd and Copy are "trait bounds" ensuring T can be compared and copied.
```

The Single generic function `get_largest` can be called with a *vector of i32* or a *vector of char* or any
type that satisfies the necessary traits.

The core logic i.e the steps to find the largest element ( iterating through the list comparing elements and
keeping tract of the largest found so far) is **identical** regardless of whether the elements are integers
or characters.

By using generics you write this comparison and tracking logic only once inside the generic function. 

=> The Rust compiler handles generating the specific, optimized code for `i32` when you call it with 
integers, and the code for `char` when you call it with characters. 
This process, called `monomorphization`, ensures that the final compiled code is just as fast as if you had 
written the specialized, duplicated functions by hand.

**Generic types allow you to abstract over the data type, focusing on the structure and algorithm of your
code, which reduces repetition and makes your code cleaner and easier to maintain.**

Some familiar generic types:
`Option<T>`, `Vec<T>`, `HashMap<K,V>`, `Result<T,E>` 


### How to extract a function to reduce code duplication: 

Before starting to make a generic function, is to explore how to extract a function to reduce code
duplication.

Using an example:  ( program that find the largest number in a list )

```rust 
    fn main() {
        let number_list = vec![12,32,54,42,100,65 ];

        let mut largest = &number_list[0];

        for number in number_list {
            if number > largest {
                largest = number;
            }
        }
        println!("the largest number is {largest}");
    }
```

Next finding the largest number in two different list of numbers.

```rust 
    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {largest}");

        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];

        let mut largest = &number_list[0];

        for number in &number_list {
            if number > largest {
                largest = number;
            }
        }

        println!("The largest number is {largest}");
    }
```
this has duplicate code and it can get error prone as the code includes many of such code blocks. 
The common functionality can be built into a function as:

```rust 
    fn largest(list: &[i32]) -> &i32 {
        let mut largest = &list[0];
        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {result}");
        let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
        let result = largest(&number_list);
        println!("The largest number is {result}");
    }
```
Summarizing:
1. Identify duplicate code.
2. Extract the duplicate code into the body of the function, and specify the inputs and return values of
   that code in the function signature. 
3. Update the two instance of duplicate code to call the function instead.

### Generic Data Types:

Generics can be used to create definitions for items like *functions signatures* or *struct* or *enums*, 
which we can then use with many different data types.

Generics also affect code performance

Using similar example to find the largest of numbers and largest of char :

```rust 
    fn largest_i32(list: &[i32]) -> &i32 {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn largest_char(list: &[char]) -> &char {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }
        largest
    }

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest_i32(&number_list);
        println!("The largest number is {result}");

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest_char(&char_list);
        println!("The largest char is {result}");
    }
```
Since both function **largest_i32** and **largest_char** have similar body we can eliminate the duplication
by introducing *Generic type parameters* in a single function.

To define the generic *largest* function, we place type name declarations inside angle bracket <>, between
the name of the function and the parameters list as below:

    `fn largest<T>(list: &[T]) -> &T {...} `

This is read as "the function largest is generic over some type `T`". The function has one parameter named
`;ist` which is a slice of values of type `T`. The largest function will return a reference to a value of
the same type `T`.

```rust 
    fn largest<T>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];

        let result = largest(&number_list);
        println!("The largest number is {result}");

        let char_list = vec!['y', 'm', 'a', 'q'];

        let result = largest(&char_list);
        println!("The largest char is {result}");
    }
```
The above code works for i32 and char, but its not generic and may now work for other types, thus this code
fails to compile with the error: 
```bash
....
help: consider restricting type parameter `T` with trait `PartialOrd`
  |
1 | fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {

```
**std::cmp::PartialOrd** is a trait which is required to enable comparisons.  

The fixed code is as below: we add trait bound on T in the function signature: specifying **PartialOrd**
this bounds put a restriction that type valid for T to only those that implement PartialOrd on both i32 and
char.

```rust
    fn largest<T: PartialOrd>(list: &[T]) -> &T {
        let mut largest = &list[0];

        for item in list {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    fn main() {
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        println!("The largest number is {result}");

        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        println!("The largest char is {result}");
    }
```

#### Similarly we can also define structs enums.. with generic members:

```rust 

struct Point<T> {
    x: T,
    y: T,
}
fn main () {
    let integer = Point{x:5, y: 10};
    let float = Point((x ))
}
``` 

enum:
```rust 
    enum Option<T> {
   
   Some(T),
   None,
}
// Option<T> enum is generic over type T and has two variants: 
// - Some, which holds one value of type T, 
// - None variant that doesn’t hold any value

enum Result<T, E> {
    Ok(T),
    Err(E),
}
// Result is generic over two types T and E, and two variants 
// - `Ok` which holds T
// - Err which holds a value of type E. 

```

In error_handle example we used: 
T : to std::fs::File 
and
E : to std::io::Error 

Similar to functions when we detect multiple struct or enum definitions that differ only in the types of 
values they hold, you can avoid duplicating by use of `generic types` instead.

#### Generic In Method definitions:

We can implement methods on structs and enums and use generic types in their definitions.

Ex:
```rust 
    struct Point<T> {
        x: T,
        y: T,
    }

    impl<T> Point<T> {
        fn x(&self) -> &T {
            &self.x
        }
    }

    fn main() {
        let p = Point { x: 5, y: 10 };

        println!("p.x = {}", p.x());
    }
```
- implementing a method `x` on `Point<T>` struct that will return a reference to `x` 
- Note: that we have to declare `T` just after `impl` so we can use `T` to specify that we’re implementing 
  methods on the type Point<T>. 
  By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in 
  Point is a generic type rather than a concrete type.


### Performance of Code Using Generics:

Using generics does not hurt performance, they often improve it.

- Rust generics are zero-cost abstractions. 

- The compiler generates highly optimized concrete code at compile time for each use of a generic this
  process is called  "Monomorphization". 

Monomorphization: 

When you use generics in Rust, the compiler performs monomorphization which means:
- For each concrete type used with generic, the compiler generates a specialized version of that code. 
- Its like copy-pasting the function/struct with the actual type substituted in.

```rust

    fn echo<T>(x: T) -> T {
        x
    }

    fn main() {
        let a = echo(42);       // T = i32
        let b = echo(3.14);     // T = f64
    }
```

After monomorphization the compiler produces something like:

```rust 
    fn echo_i32(x: i32) -> i32 { x }
    fn echo_f64(x: f64) -> f64 { x }
```
ie you are not paying any abstraction cost at runtime. You are getting the performance of handwritten
type-specific code. 

1. Code Size (Bloat):
    - Since Rust generates separate versions for each type, your binary can grow if you use lots of
      different type parameters.
    - This is called code bloat.
    - Usually negligible, but you can mitigate it using trait objects (dyn Trait) if needed.

2. Inline Opportunities: 
    - generic code can often be in-lined, making it faster then non-generic code ( especially with small
      functions and iterators)

3. Trait Bounds and Dynamic Dispatch:
    - If you use **trait objects** like `Box<dyn Trait>` instead of generic:
        * You get **dynamic dispatch** ( like vtables in C++ )
        * This can cost a single pointer indirection at runtime. 
        * But it reduces code bloat. 


Example 2: 

Using stand lib generic Option<T> enum:

```rust
    let integer = Some(5);
    let float = Some(5.0);
```

When Rust compile this code it performs monomorphization. During which the compiler reads the value  that
has been used in `Option<T>` instance and identifies two kids of `Options<T>` for i32 and f64. 
Next it expands the generic definition of `Option<T>` into two definition specific to i32 and f64, there by
replacing the generic defenition with the specific ones.

The Monomorphhized version looks as:

```rust 
    enum Option_i32 {
        Some(i32),
        None,
    }

    enum Option_f64 {
        Some(f64),
        None,
    }

    fn main() {
        let integer = Option_i32::Some(5);
        let float = Option_f64::Some(5.0);
    }
```
The generic `Option<T>` is replaced with the specific definitions created by the compiler. 
Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime 
cost for using generics.

The process of Monomorphization makes Rust code extremely efficient at runtime.

------------------------

# Recap Key features of Generics:

1. Generic Functions
2. Generic `structs` and `enums`
3. Trait Bounds 
4. Trait implementations for generic types 
5. Associated Types 
6. Default Generic Type parameters 
7. Lifetimes with generics 
8. PhantomData ( zero-sized types for type hints )

Example that covers most of the above items:
Create a generic in-memory cache that stores and returns results of computations for any type, with the
ability to plug in custom key/value types and hashes. 
Example will define:
- A generic function that retrieves or computes a value.
- A generic struct Cache with constraints on K and V.
- A generic enum CacheResult with methods.
- Use of associated types via a trait Cacheable.
- Lifetimes for referencing data.
- PhantomData to enforce type constraints without storing real data.

```rust 
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;

// ---------- Trait with Associated Type ----------
pub trait Cacheable {
    type Key: Eq + Hash;
    type Value;

    fn compute(key: &Self::Key) -> Self::Value;
}

// ---------- Enum with Generics ----------
#[derive(Debug)]
pub enum CacheResult<V, E> {
    Hit(V),
    Miss(E),
}

impl<V, E> CacheResult<V, E> {
    pub fn is_hit(&self) -> bool {
        matches!(self, CacheResult::Hit(_))
    }

    pub fn unwrap(self) -> V {
        match self {
            CacheResult::Hit(v) => v,
            CacheResult::Miss(_) => panic!("Tried to unwrap a Miss"),
        }
    }
}

// ---------- Struct with Generic Types ----------
pub struct Cache<T: Cacheable> {
    store: HashMap<T::Key, T::Value>,
    _marker: PhantomData<T>,
}

impl<T: Cacheable> Cache<T> {
    pub fn new() -> Self {
        Self {
            store: HashMap::new(),
            _marker: PhantomData,
        }
    }

    pub fn get_or_compute(&mut self, key: T::Key) -> CacheResult<&T::Value, &T::Key> {
        if self.store.contains_key(&key) {
            CacheResult::Hit(self.store.get(&key).unwrap())
        } else {
            let value = T::compute(&key);
            self.store.insert(key.clone(), value);
            CacheResult::Miss(&key)
        }
    }

    pub fn get(&self, key: &T::Key) -> Option<&T::Value> {
        self.store.get(key)
    }
}

// ---------- Implement Cacheable for a concrete type ----------
struct SquareCalculator;

impl Cacheable for SquareCalculator {
    type Key = u32;
    type Value = u64;

    fn compute(key: &Self::Key) -> Self::Value {
        (*key as u64) * (*key as u64)
    }
}

// ---------- Generic function using type parameters ----------
fn print_cache_result<K, V>(key: K, result: CacheResult<V, &K>)
where
    K: std::fmt::Debug,
    V: std::fmt::Debug,
{
    match result {
        CacheResult::Hit(value) => println!("Cache hit: {:?} => {:?}", key, value),
        CacheResult::Miss(k) => println!("Cache miss for key: {:?}", k),
    }
}

// ---------- Main usage ----------
fn main() {
    let mut cache = Cache::<SquareCalculator>::new();

    let keys = vec![2, 3, 2];

    for key in keys {
        let result = cache.get_or_compute(key);
        print_cache_result(key, result);
    }

    println!("Final cache state: {:?}", cache.get(&2));
}

```
