# Functional Programming:
---

## 1.0 What is FP?
---

FP is a paradigm that treats computation as the evaluation of mathematical functions and avoids changing
state of mutable data.

Key Concepts of FP:

1. First-Class Functions: Functions can be passed as arguments, returned as values, and assigned to
   variables. 

2. Immutability: Once a data is created, it can not be changed. This avoid side effects, making code easier
   to reason about.

3. Pure Functions: Functions that always produce the same output for the same input and have no side
   effects.

4. Higher-Order Functions: Functions that take other functions as arguments and return them as results.

5. Recursion: Functions that call themselves, often used in place of looping constructs (e.g, `for` or
   `whilw` )

6. Laziness: Delaying computation until it's needed, which can improve efficiency. 


## 2.0 Functional Programming Features in Python:
---

Python is a multi-paradigm language that supports some features of FP. 
Python is not purely functional it offers several FP constructs that can be useful for certain problems.

### 2.1 Key FP features in Python:

- First-class functions:

```python 
def square(x):
    return x*x

def apply_func(f, x):
    return f(x)

print(apply_func(square, 5)) # this outputs 25.
```
- Lambda functions ( Anonymous Functions )
  In short Lambda functions are defined as small anonymous functions:

```python 
add lambda x,y: x+y
print(add(2,3)) # outputs 5
```

- Map, Filter, and Reduce:
  These are higher order functions that operate on iterables and help in expressing FP concepts:

  * `map()`: applies a function to each item of an iterable. 
  * `filter()`: filters elements based on condition.
  * `reduce()`: applies a function cumulatively to the items of an iterable.

```python
from functools import reduce

numbers = [1, 2, 3, 4, 5]

# Using map to square numbers
squares = list(map(lambda x: x * x, numbers))
print(squares)  # [1, 4, 9, 16, 25]

# Using filter to keep only even numbers
evens = list(filter(lambda x: x % 2 == 0, numbers))
print(evens)  # [2, 4]

# Using reduce to calculate the product of all numbers
product = reduce(lambda x, y: x * y, numbers)
print(product)  # 120
```
- Immutability (Mostly by Convention)
  While Python doesn't enforce immutability, you can use `tuple` (immutable sequences) and `frozenset` 
  (immutable sets). 
  Libraries like `pyrsistent` also provide immutable data structures.

- List Comprehensions and Generators
  Python’s list comprehensions and generator expressions are a functional-style approach to building 
  iterables.

```python
squares = [x * x for x in range(10)]
print(squares)  # [0, 1, 4, 9, 16, 25, 36, 49, 64, 81]
```
### 2.2 FP Limitations in Python:

- **Lack of Immutable Data Structures**: 
    Unlike languages like Haskell, Python doesn't enforce immutability. 
    However, you can still write functional-style code with some care.

- **No Tail-Call Optimization**: 
Python does not optimize tail recursion, so deep recursion may lead to stack overflow errors.

## 3.0  Functional Programming Features in Rust
---

Rust is Systems Programming language that supports functional programming alongside imperative and
object-oriented paradigm. 

Rust supports most of the FP requirements (leaving out Lazy Evaluation , no tail-call optimization..)

Rust is more restrictive in some ways, but it also provides strong guarantees about performance and safety, 
making it ideal for low-level and high-performance functional programming.

### 3.1 Key FP features of Rust:

- First-Class functions: 
  Functions are first-class citizens in Rust, => they can be passed around, returned from other functions, 
  and assigned to variables.


```rust
fn square(x: i32) -> i32 {
    x * x
}

fn apply_func(f: fn(i32) -> i32, x: i32) -> i32 {
    f(x)
}

fn main() {
    let result = apply_func(square, 5);
    println!("{}", result);  // Outputs: 25
}
```

- Closures (Lambda Functions) ( Anonymous Functions )
  Rust supports closures, which are anonymous functions that can capture their environment.

```rust
    let add = |x, y| x + y;
    println!("{}", add(2, 3));  // Outputs: 5
```

- Immutability
  Rust emphasizes immutability by default. 
  Once you bind a variable, you cannot change it unless you explicitly make it mutable with the `mut` keyword.

  ```rust
  let x = 5;  // Immutable by default
  // x = 10;  // Error: cannot assign twice to immutable variable
  ```
- Pattern Matching: 
  Pattern matching is an expressive tool in Rust and is used extensively in FP for deconstructing and 
  matching values.

```rust
    let number = Some(10);

    match number {
        Some(n) => println!("Got a number: {}", n),
        None => println!("No number found"),
    }
```

- `Option` and `Result` Types ( Pure functions )
  Rust’s `Option` and `Result` types provide a functional way to handle the presence/absence of values and 
  errors, respectively. 

  These types are immutable and don't require null values or exceptions.

```rust
fn divide(x: i32, y: i32) -> Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x / y)
    }
}

match divide(10, 2) {
    Some(result) => println!("Result: {}", result),
    None => println!("Error: Division by zero"),
}
```

- Higher-Order Functions
  Rust’s iterators support higher-order functions like `map`, `filter`, and `fold`.

```rust
    let numbers = vec![1, 2, 3, 4, 5];
    
    let squares: Vec<i32> = numbers.iter().map(|&x| x * x).collect();
    
    println!("{:?}", squares);  // Outputs: [1, 4, 9, 16, 25]
```

- Recursion:
  While Rust allows recursion, it’s more optimized for iterative solutions due to the lack of tail call 
  optimization. However, recursion is still a useful pattern for functional-style programming.

```rust
fn factorial(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn main() {
    println!("{}", factorial(5));  // Outputs: 120
}
```

### 3.1 Functional Programming Limitations in Rust:

- No Tail-Call Optimization: 
    Like Python, Rust does not support tail-call optimization, which means recursion depth can still lead 
    to stack overflow for large inputs.

- Complexity of Ownership and Borrowing: While these features give Rust its performance and safety, they may
  complicate some functional programming techniques that rely heavily on immutability and passing around 
  values.

## 4. Comparison: Python vs. Rust in Functional Programming

| Feature                  | Python                                          | Rust                                           |
| ------------------------ | ----------------------------------------------- | ---------------------------------------------- |
| First-Class Functions    | Supported                                       | Supported                                      |
| Closures / Lambdas**     | Supported                                       | Supported                                      |
| Immutability             | Mostly by convention (e.g., tuples, frozensets) | Enforced by default, `mut` for mutable data    |
| Higher-Order Functions   | Supported (e.g., `map`, `filter`, `reduce`)     | Supported (iterators, `map`, `filter`, `fold`) |
| Recursion                | Supported (but not optimized)                   | Supported (but not optimized)                  |
| Pattern Matching**       | Not available                                   | Available (powerful `match` statement)         |
| Error Handling**         | Exceptions (not FP-style)                       | `Result` and `Option` (FP-style, immutability) |

## 5. Conclusion

Both Python and Rust provide functional programming features, but their levels of support differ:

* Python is more relaxed, offering functional constructs without enforcing immutability. 
  It's suitable for many types of applications where functional programming can provide elegant solutions 
  but may not always be the most efficient for low-level systems work.

* Rust embraces FP alongside its system programming capabilities. 
  It provides more advanced FP features like pattern matching and enforced immutability, making it better 
  suited for building performance-critical systems with functional paradigms.




