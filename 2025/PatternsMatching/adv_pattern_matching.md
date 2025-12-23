# Patterns and Matching in Rust


## Introduction to Patterns in Rust

Patterns in Rust represent a specialized syntax that allows developers to match against the structure of 
various types, whether they are complex or simple. 

This capability enhances control over a program's flow, enabling more precise decision-making based on the 
shape of data. 

Patterns can be utilized in conjunction with match expressions and other constructs within the Rust.

### Components of Patterns

Patterns are composed of several elements, which can be combined in various ways to create effective matches.

The main components include:
    **Literals**: 
        Fixed values such as *numbers*, *strings*, or *characters*.
    **Destructured Types**: 
        Includes *arrays*, *enums*, *structs*, or *tuples* that are broken down into their constituent parts 
        for easier access and manipulation.
    **Variables**: 
        Named identifiers that can hold values and are used to capture data from patterns.
    **Wildcards**: 
        Denoted by an underscore (_), wildcards are placeholders that match any value without binding it to 
        a variable.
    **Placeholders**: 
        Specific constructs that allow for matching without requiring a direct reference to a variable.

**Example Patterns**

Some illustrative examples of patterns include:

A simple variable pattern, such as `x`, which matches any value and binds it to the variable `x`.
A tuple pattern, such as `(a, 3)`, which matches a two-element tuple where the first element is bound to a 
and the second element must be the integer 3.

An enum pattern, such as `Some(Color::Red)`, which matches an `Option` type containing a specific variant 
of the Color enum.

**Matching Values Against Patterns**

To utilize a pattern, it must be compared against a specific value. 

The process is as follows:

If the value conforms to the shape of the pattern, the program can proceed to use the components of the 
matched value within the code.

Conversely, if the value does not match the pattern, the associated code block will not execute.

This matching mechanism is pivotal in controlling the flow of a program, as it allows for conditional 
execution based on the structure and type of data being processed. 

This article the below items are covered:
- Valid Contexts for Patterns: 
    An exploration of where patterns can be effectively utilized within the Rust language.

- Refutable vs. Irrefutable Patterns: 
    A distinction between patterns that can fail to match (refutable) and those that will always match
    (irrefutable).

- Syntax Variations:
    An overview of the different syntactic forms that patterns can take, enhancing the developer's ability 
    to express various concepts clearly and concisely.


## Places where Patterns can be used:


Patterns in Rust are versatile and can be used in a variety of contexts. 
They can be applied, enhancing the control flow and data handling in Rust programs.

**Contexts for Using Patterns**

### `match` Expressions: 
    Patterns are primarily used in `match` expressions to compare a value against multiple patterns and 
    execute the corresponding code block for the first matching pattern.

```rust 
let number = 13;
match number {
    1 => println!("One"),
    2 => println!("Two"),
    3..=12 => println!("Between three and twelve"),
    _ => println!("Something else"),
}
```

### `if let` Syntax: 
    This syntax allows for a more concise way to match a single pattern against a value, particularly useful 
    for handling `Option` and `Result` types.

```rust 
let some_value = Some(10);
if let Some(x) = some_value {
    println!("Value is: {}", x);
} else {
    println!("No value");
}
```

### `while let` Loops: 
    Similar to `if let`, this construct allows for looping as long as a pattern matches a value, 
    facilitating operations on iterators or optional values.

```rust 
let mut optional_value = Some(0);
while let Some(x) = optional_value {
    println!("Value is: {}", x);
    optional_value = if x < 5 { Some(x + 1) } else { None };
}
```

### **Function Parameters**:
    Patterns can be used in function parameters to de-structure complex types directly in the function 
    signature, making it easier to work with structured data.
```rust
fn print_coordinates(&(x, y): &(i32, i32)) {
    println!("x: {}, y: {}", x, y);
}

let point = (10, 20);
print_coordinates(&point);
```


### `Structs` and `Enums`: 
    When defining `structs` or `enums`, patterns can be utilized to destructure their fields when accessing 
    or manipulating data.

```rust
struct Point {
    x: i32,
    y: i32,
}

let point = Point { x: 10, y: 20 };
let Point { x, y } = point;
println!("Point coordinates: x = {}, y = {}", x, y);
```

### `for` Loops: 
    Patterns can be used in `for` loops to destructure items in collections, enabling easier access to their
    components.

```rust
let pairs = vec![(1, 'a'), (2, 'b'), (3, 'c')];
for (number, letter) in pairs {
    println!("Number: {}, Letter: {}", number, letter);
}
```


### `let` Bindings: 
    Patterns can be employed in `let` statements to bind variables to specific parts of data structures, 
    allowing for more readable code.

```rust
let (x, y) = (10, 20);
println!("x: {}, y: {}", x, y);
```

Understanding where patterns can be applied in Rust is crucial for writing effective and idiomatic code. 


##  Refutability: Whether a pattern might fail to match:

**Refutability** in Rust refers to whether a pattern can fail to match a value. 

Understanding the difference between `refutable` and `irrefutable` patterns is essential for effectively 
using patterns in Rust.

### Irrefutable Patterns:

Irrefutable patterns are those that will always successfully match a value. 
They can be used in contexts where a match is guaranteed. 

Examples of irrefutable patterns include:
Simple variable bindings: A variable pattern like x will always match any value.

`let x = 5; // `x` is irrefutable`


`Struct` and `tuple` patterns: When destructuring a struct or tuple that is guaranteed to exist.

`let (x, y) = (1, 2); // Always matches since the tuple exists`

### Refutable Patterns

Refutable patterns, on the other hand, can fail to `match` a value. 
They are typically used in contexts where a `match` may not always succeed, such as in match expressions or 
`if let` statements. Examples of refutable patterns include:

Pattern matching with match: Using patterns that may not match all possible values.
```rust 
let number = 5;
match number {
    1 => println!("One"),
    2 => println!("Two"),
    _ => println!("Not one or two"), // This is a refutable pattern
}
```

Using `if let`: This allows for a more concise way to handle cases where a value might not match.

```rust
let some_value = Some(10);
if let Some(x) = some_value { // Refutable pattern
    println!("Value is: {}", x);
}
```

### Importance of Refutability

Understanding refutability is crucial because it affects how patterns can be used in different contexts. 
For example, irrefutable patterns can be used in `let` bindings without concern for failure, `while`
refutable patterns require handling for cases where they do not match.

This understanding helps in choosing the right patterns for the right contexts, ensuring that the program 
behaves as expected.


## Pattern Syntax:


Pattern syntax in Rust provides a way to match the structure of data. 
Understanding the various forms of pattern syntax is essential for effectively using patterns in Rust.

### Basic Pattern Syntax


- `Literals`: Patterns can match literal values directly.

```rust 
let x = 5;
match x {
    5 => println!("Matched five!"),
    _ => println!("Not five!"),
}
```

- `Variables`: Patterns can bind a value to a variable.

```rust
let x = 10;
match x {
    y => println!("y is: {}", y), // `y` binds to the value of `x`
}
```

- `Wildcards`: The underscore `(_)` is a wildcard pattern that matches any value without binding it.

```rust
let x = 7;
match x {
    5 => println!("Five!"),
    _ => println!("Not five!"), // Matches any value
}
```

- `Destructuring` Patterns

`Tuples`: Patterns can destructure tuples to access their elements.

```rust
let point = (3, 5);
match point {
    (x, y) => println!("Point is at ({}, {})", x, y),
}
```

- `Structs`: Patterns can destructure structs to access their fields.

```rust
struct Point {
    x: i32,
    y: i32,
}
let point = Point { x: 10, y: 20 };
match point {
    Point { x, y } => println!("Point is at ({}, {})", x, y),
}
```

- `Enums`: Patterns can match specific variants of enums.

```rust
enum Color {
    Red,
    Green,
    Blue,
}
let color = Color::Red;
match color {
    Color::Red => println!("Red!"),
    _ => println!("Not red!"),
}
```

- Additional Pattern Syntax Features

`Range` Patterns: Patterns can match a range of values.

```rust
let x = 7;
match x {
    1..=5 => println!("Between 1 and 5"),
    _ => println!("Greater than 5"),
}
```

- `Multiple` Patterns: Patterns can be combined using the `|` operator to match multiple values.
```rust
let x = 2;
match x {
    1 | 2 => println!("One or two!"),
    _ => println!("Something else!"),
}
```

- Guard Conditions: Additional conditions can be added to patterns using `if` guards.

```rust
let x = 10;
match x {
    n if n < 0 => println!("Negative"),
    n if n > 0 => println!("Positive"),
    _ => println!("Zero"),
}
```
