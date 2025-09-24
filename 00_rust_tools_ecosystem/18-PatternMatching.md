# Pattern Matching:


Pattern matching is one of the most powerful features in Rust programming language:
- it allows to check the shape of values and deconstruct them in a concise and expressive way. 
- Most used tools for pattern matching in Rust are: `match`, `if let` and `while let`


## `match` : 

Allows you to compare a value against pattern and execute code based on which pattern it matches. 

Its similar to `swich` in C,C++, but is much more powerful.

- Exhaustiveness check: Rust ensures that all possible patterns are matched and if not, it will give 
  compile-time error.

- Pattern de-structuring: we can de-structure `enums`, `tuples`, `structs`, and even primitive values 
  in the patterns.

Syntax:
    
    ```rust 

        match value {
            Pattern1 => { /* code to execute */},
            Pattern2 => { /* code to execute */},
            ...
            _ => { /* default case */},  // wild card pattern.
        }
    ```

Example :

```rust 
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_player (direction: Direction) {
    match direction {
        Direction::Up => println!("Moving Up"),
        Direction::Down => println!("Moving Down"),
        Direction::Left => println!("Moving Left"),
        Direction::Right => println!("Moving Right"),
    }
}

fn main () {
    let direction = Direction::Up;

    move_player(direction);
}
```
- `match` statement checks the value of `direction` and matches it to the one of the variants of the 
  Direction enum.

- The `_` (wildcard) pattern can be used to match any value, acting as default case. This should be the last
  patten inside match {}

Example ( complex pattern )


```rust 

fn describe_number(x: i32) {
    match x {
        1 => println!("One!"),
        2..=5 => println!("between 2 and 5"),
        _ => println!("other numbers"),
    }
}

fn main() {
    describe_number(3);
}
```
- the `2..=5` this is *range pattern*, matching any value from 2 to 5 inclusive.
- the `_` wildcard matches any other value.

## `if let` :

`if let` is a more concise way to `match` on a single pattern when you only care about one match case.

It is useful for handling `enums` where you only need to match one variant and don't want to write a full
`match` block.

- Useful when you want to match and de-structure a single variant. 
- you can use `else` clause to handle the "default" case.

Syntax:

    ```rust 
        if let Pattern = value {
            // code to run if the pattern matches 
        } else {
            // code to run if the pattern doesn't match
        }
    ```

Example:

```rust 
enum Option<T> {
    Some(T),
    None,
}

fn check_option(opt: Option<i32>) {
    if let Option::Some(val) = opt {
        println!("we got a value: {}", val);
    } else {
        println!("No Value");
    }
}

fn main() {
    let x = Option::Some(42);
    check_option(x);
}
```
- The `if let` syntax is used to check if the `opt` is `Some(val)` and, if so, to execute the code inside
  the block.

- If `opt` is `None` the `else` block will run.

Example:

```rust 
fn check_value(x: Option<i32>) {

    if let Option::Some(val) = opt {
        println!("we got a value: {}", val);
    } else {
        println!("No Value found");
    }
}

fn main() {
    let some_val = Some(5);
    let no_val: Option<i32> = None;

    check_value(some_val); //prints got a val 5
    check_value(no_val);   // prints no value found
}
```

## `while let` : 

`while let` is used for looping while the pattern matches. 
useful when you want to process elements from an iterator, a linked list, or other Collections 
that can be de-structured in each iteration.

- Allows for looping through patterns typically with an `Option` or `Result` until the value doesn't match 
  the pattern.


Syntax:

    ```rust 
        while let Pattern = value {
            // code to run for each match
        }
    ```

Example:

```rust 
fn process_queue ( queue: Option<Vec<i32>>) {
    //let mut queue = queue;
    let queue = queue;

    while let Some(v) = queue {
        for num in v {
            println!("{}", num);
        }
        break; // to avoid infinite loop in this case
    }
}

fn main() {
    let my_queue = Some(vec![1,2,3,4]);
    process_queue(my_queue);
}
```
- `while let Some(v) = queue` will continue looping untill `queue` is `None`
- The Loop processes elements in the `Option<Vec<i32>>` de-structuring the vector and printing its elements.


## Combining all the matching statements:

```rust 

enum MyOption<T> {
    Some(T),
    None,
}

fn process_option(opt: MyOption<i32>) {
    match opt {
        MyOption::Some(val) => println!("Found value: {}", val),
        MyOption::None => println!("No value!"),
    }
}

fn main() {
    let values = vec![Some(5), None, Some(10)];
    
    for value in values {
        if let Some(v) = value {
            println!("Processing: {}", v);
        }
    }

    let my_opt = MyOption::Some(7);
    process_option(my_opt);
}
```
- `match` is used to handle `Option` cases
- `if let` used in loop to process each item in the `values` vector.
- `MyOption` is a custom enum used with pattern matching.

Summary:

To master pattern matching check:
- Enums ( `Option`, `Result`) 
- Tuples and Structs ( use pattern matching to de-structure them)
- Ranges, Arrays and Slices ( Use Range patterns and array matching)


## Other forms of pattern matching:

- `let` binding to de-structure data as soon as it's assigned. ( commonly used with tuples and structs, 
   enums ... )

   ```
   let (a,b) = (1,2);
   println!("a: {a}, b: {b}");
   ```

   * This allows to de-structure values right when they are bound to variable.
   * de-structure Tuples, Structs, and enums are common use cases.

ex:
```rust 
struct Point {
    x: i32,
    y: i32,
}

let Point { x, y } = Point { x: 1, y: 2 };
println!("x: {}, y: {}", x, y);


enum MyOption<T> {
    Some(T),
    None,
}

let MyOption::Some(val) = MyOption::Some(42);
println!("Value: {}", val);

//wild card _ , which matches any value and doesn't care about the value itself

let x = Some(5);

match x {
    Some(_) => println!("Found a value"),
    None => println!("No value"),
} 


let (a, _, c) = (1, 2, 3);
println!("a: {}, c: {}", a, c);

```
- Range patterns ( start..=end)
  useful when you want to match a set of consecutive values, such as matching numbers within a specific range.

```rust 

fn check_number(x: i32) {
    match x {
        1..=10 => println!("Between 1 and 10"),
        11..=20 => println!("Between 11 and 20"),
        _ => println!("Other number"),
    }
}

fn main() {
    check_number(5);   // Prints: Between 1 and 10
    check_number(15);  // Prints: Between 11 and 20
}
```

- `@` Binding a value to a variable:

The `@` operator allows you to bind the value of a pattern to a variable while still matching on the pattern.
This is useful if you want to capture the value for later use while still checking the pattern.

```rust 

fn check_number(x: Option<i32>) {
    match x {
        Some(val @ 1..=10) => println!("Found a small value: {}", val),
        Some(val) => println!("Found a larger value: {}", val),
        None => println!("No value found"),
    }
}

fn main() {
    check_number(Some(5));  // Prints: Found a small value: 5
    check_number(Some(15)); // Prints: Found a larger value: 15
}
```
- In the case Some(val @ 1..=10), val is bound to the value while matching against the range 1..=10. 
  This allows you to use the value later if necessary.


- `Result` Pattern Matching:

`Result` used for error handling also supports pattern matching. 
You  can match `Ok` and `Err` to handle success and failure cases.

```rust 
fn handle_result(r: Result<i32, String>) {
    match r {
        Ok(val) => println!("Success with value: {}", val),
        Err(e) => println!("Error: {}", e),
    }
}

fn main() {
    let success = Ok(42);
    let error: Result<i32, String> = Err("Something went wrong".to_string());

    handle_result(success); // Prints: Success with value: 42
    handle_result(error);   // Prints: Error: Something went wrong
}
```
   
## Conclusion

Rust provides a wide variety of pattern matching techniques beyond just match, if let, and while let. 
These include:

1. Destructuring patterns in let bindings (for tuples, structs, enums, etc.)

2. Wildcards (_) for ignoring certain values

3. Range patterns for matching a range of values

4. Deep destructuring for complex data types

5. Binding with @ to capture a value while matching a pattern

6. Tuple structs and struct patterns for complex data matching

7. Multiple patterns (|) for matching any of several values

8. Matching on Result types for error handling

These patterns enable very expressive and readable code. 
The Rust compiler ensures that your pattern matching is exhaustive (covering all possible cases), making 
it a robust tool for handling various data structures and control flows.
