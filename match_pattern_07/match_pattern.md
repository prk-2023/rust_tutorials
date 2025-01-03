# match and pattern 

Match: Its a pattern matching control flow construct
Patterns: are common in rust which get used in variable binding, match expressions and many other places.

This article covers match along with pattern matching:

---

**`match`** statement in Rust is a powerful, pattern-matching control flow construct that allows you to 
compare a value against a series of patterns and execute the corresponding block of code when a pattern 
matches. 

It's a central feature of Rust's expressive and safety-oriented syntax. 

The `match` statement is somewhat similar to `switch` statements in languages like C and C++, but it has 
significant differences and advantages that make it more powerful and safer.

### 1. **Basic Syntax of `match` in Rust**

The syntax for a `match` expression looks like this:

    ```rust
    match value {
        pattern1 => expression1,
        pattern2 => expression2,
        _ => default_expression,  // _ is a catch-all wildcard pattern
    }
    ```

- **`value`** is the value being matched.
- **`pattern1`, `pattern2`**, etc., are the patterns you're comparing the value against.
- **`expression1`, `expression2`**, .., are the blocks of code that are executed if the pattern matches.
- **`_`** is a wildcard pattern that matches any value and serves as a default case 
    (similar to `default` in C/C++ `switch`).

### 2. **Pattern Matching in Rust**

Rust's `match` supports a rich set of patterns, making it more flexible than `switch` in C/C++. 

Here are some of the key features:

#### a. **Literal Patterns**

You can match against specific values or literals, like numbers or strings.

    ```rust
    let x = 1;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other"),  // default case
    }
    ```

#### b. **Range Patterns**

Rust allows you to match a value against a range of values, which is not available in C/C++ `switch` 
statements.

    ```rust
    let x = 5;
    match x {
        1..=5 => println!("Between 1 and 5"),  // Matches any value from 1 to 5
        _ => println!("Other"),
    }
    ```

#### c. **Pattern Binding**

You can bind values to variables inside patterns to capture parts of the value.

    ```rust
    let x = Some(42);
    match x {
        Some(n) => println!("The number is {}", n),
        None => println!("No value"),
    }
    ```

#### d. **Destructuring Patterns**

You can destructure tuples, structs, and enums in patterns to extract and match their components.

    ```rust
    let point = (3, 4);
    match point {
        (0, 0) => println!("Origin"),
        (0, y) => println!("On the Y axis at {}", y),
        (x, 0) => println!("On the X axis at {}", x),
        (x, y) => println!("Point at ({}, {})", x, y),
    }
    ```

#### e. **Enums and Option/Result Matching**

Rust's `match` shines when working with `Option`, `Result`, and other enums, making it a perfect fit for 
handling different outcomes of operations. 'match' processes the possible variants of an enum.
    ```rust 
    #![allow(unused_variables)]
    fn main() {
        enum Message {
            Quit,
            ChangeColor(i32, i32, i32),
            Move { x: i32, y: i32 },
            Write(String),
        }
        fn quit() { /* ... */ }
        fn change_color(r: i32, g: i32, b: i32) { /* ... */ }
        fn move_cursor(x: i32, y: i32) { /* ... */ }

        fn process_message(msg: Message) {
            match msg {
                Message::Quit => quit(),
                Message::ChangeColor(r, g, b) => change_color(r, g, b),
                Message::Move { x, y: new_name_for_y } => move_cursor(x, new_name_for_y),
                Message::Write(s) => println!("{}", s),
            };
        }
    }
    ```

    ```rust 
    let result: Result<i32, &str> = Ok(42);
    match result {
        Ok(val) => println!("Success with value: {}", val),
        Err(err) => println!("Error: {}", err),
    }
    ```

### 3. **`match` vs `switch` in C/C++:**

#### a. **Exhaustiveness Checking**

In Rust, the `match` statement is **exhaustive**: the compiler ensures that all possible cases are covered. 
If a match statement is not exhaustive(i.e: there are missing patterns), Rust compiler will raise an error.

For example:

    ```rust
    let x = 1;
    match x {
        1 => println!("One"),
        _ => println!("Something else"),
    }  // This is exhaustive because _ is a wildcard for any value.
    ```

In contrast, in C and C++, if you forget to cover a case, the code will silently fall through to the next 
case (unless you specify a `default` case).

    ```cpp
    int x = 1;
    switch (x) {
        case 1:
            std::cout << "One";
            break;
        // No default, and no error if case is missing.
    }
    ```

#### b. **Pattern Matching**

Unlike C/C++, Rust allows you to match against **complex patterns**, including destructuring, ranges, and 
enums. This makes `match` far more powerful and flexible.

In C/C++, you would need to use multiple `if` statements or break down complex conditions into simpler ones:

    ```cpp
    int x = 5;
    switch (x) {
        case 1: // check single value
        case 2:
        case 3:
            // do something
            break;
        default:
            // do something else
    }
    ```

In Rust, this would be more straightforward using range patterns:

    ```rust
    let x = 5;
    match x {
        1..=3 => println!("Between 1 and 3"),
        _ => println!("Something else"),
    }
    ```

#### c. **No Fall-through**

Rust `match` does **not** have "fall-through" behavior like `switch` in C/C++. 
This means once a match arm is executed, no other arms are evaluated. Each arm is considered a complete 
branch.

This removes a common source of bugs in C/C++ when a `case` doesn’t have a `break` statement:

    ```cpp
    int x = 1;
    switch (x) {
        case 1:
            std::cout << "One";
        case 2:  // Fall-through bug, will execute this even if x is 1
            std::cout << "Two";
            break;
    }
    ```

In Rust:

    ```rust
    let x = 1;
    match x {
        1 => println!("One"),
        2 => println!("Two"),
        _ => println!("Other"),
    }
    ```

#### d. **Performance and Optimization**

Rust's `match` is **optimized** during compilation. 
The compiler can transform `match` statements into efficient jump tables or similar optimized code, making 
it faster than the typical `switch` in C/C++ in some cases. Rust’s `match` patterns are also 
**const evaluable**, meaning the compiler can optimize away certain patterns entirely if the value is known 
at compile time.

### 4. **Benefits of Rust's `match` Statement:**

- **Exhaustiveness**: The Rust compiler ensures that all possible patterns are covered, reducing the 
  chances of runtime errors due to unhandled cases.

- **Pattern Matching**: Rust's `match` allows matching against complex patterns, like ranges, tuples, enums,
  and more, which makes it far more expressive than `switch` in C/C++.

- **Safety**: Rust encourages you to handle all possible cases explicitly (even with the wildcard `_`), 
  reducing the chances of missing edge cases.
- **Destructuring**: You can destructure complex data types (like structs or tuples) directly in `match` 
  arms, making it easier to work with such types.
- **No Fall-through**: Rust's `match` prevents accidental fall-through between cases, which is a common 
  mistake in C/C++.

- **Cleaner Syntax**: Unlike `switch`, which often requires break statements to prevent fall-through, 
  `match` arms in Rust are always self-contained.

### 5. **Pitfalls to Avoid:**

- **Non-exhaustive matches**: Always ensure that you handle all possible cases. 
  The compiler will catch missing patterns, but it’s easy to leave out a case if you're not careful.
  
    ```rust
    let x = Some(5);
    match x {
        Some(5) => println!("Five"),
        // Missing None case
    }  // Error: non-exhaustive patterns
    ```

- **Wildcard `_`**: While `_` is a catch-all pattern, it can sometimes be too broad. 
   If you use `_` too liberally, you may inadvertently ignore important cases. 
   Always make sure your `_` is not swallowing cases that could lead to unexpected behavior.

- **Unnecessary complex patterns**: While Rust's `match` is very powerful, sometimes complex patterns can 
  make the code harder to understand. Try to balance readability with the expressive power of `match`.

- **Too many patterns**: If you're matching on a large number of possible patterns 
  (e.g., many enum variants), the code can become cumbersome and hard to maintain. 
  In some cases, refactoring into helper functions might make the code clearer.

### 6. if let ( Pattern matching )

"if let" can be used as a simpler alternative to match when you're only interested in handling a single 
pattern and don't need to exhaustively match all possible variants. 

It provides a more concise and readable way to destructure and work with enums (like Option, Result, etc.) 
when you're only concerned with one specific case, and all other cases can be ignored.

If you're only interested in handling one variant of an enum 
(for example, the Some case of an Option or the Ok case of a Result) and you want to ignore the other 
variants, 'if let' is a great choice.

"if let" is a more concise and limited version of match. 
It allows you to specify a single pattern to match against, and if the pattern matches, the code block 
associated with it will be executed. 
If the pattern does not match, the code block will be skipped.

    ```rust 
    let x = 1;
    if let 1 = x {
        println!("x is 1");
    }
    ```
**When to use `if let` instead of `match`**

You can use `if let` instead of `match` in the following situations:
    1. **Single pattern**: 
        When you only need to match against a single pattern, `if let` is more concise and easier to read.
    2. **No need for exhaustive matching**: 
        When you don't need to handle all possible values of a type, `if let` allows you to ignore the 
        non-matching cases.
    3. **Simple pattern**: 
        When the pattern is simple and doesn't require a complex `match` statement,`if let` is a good choice.

    Here are some examples where `if let` can be used instead of `match`:
    ```rust 
    // Example 1: Single pattern 
    let x = Some(1); 
    if let Some(y) = x { 
        println!("x is Some({})", y); 
    }

    // Example 2: No need for exhaustive matching
    let x = Ok(1);
    if let Ok(y) = x {
        println!("x is Ok({})", y);
    }

    // Example 3: Simple pattern
    let x = (1, 2);
    if let (1, y) = x {
        println!("x is (1, {})", y);
    }
    ```
### Conclusion:

Rust's `match` statement is an extremely powerful and expressive feature that far surpasses 
`switch` statements in C/C++ in terms of safety, flexibility, and readability. 

It forces exhaustiveness checking, allows complex pattern matching (including destructuring), and 
eliminates fall-through bugs. 

While powerful, you should be mindful of using `_` too broadly and ensuring that all cases are 
correctly handled, especially when matching on enums or complex data structures.


# Additional Examples
---

Here are some **difficult level** examples that demonstrate the power and flexibility of Rust’s `match` 
statement. 

These examples cover advanced features like pattern matching, destructuring, enums, ranges, and guards. 
They will help you understand the wide range of possibilities that `match` provides in Rust.

### 1. **Complex Pattern Matching with Structs and Enums**

In this ex, we’ll match on a combination of **enums** and **structs**, as well as **tuple destructuring**.

    ```rust
    #[derive(Debug)]
    enum Shape {
        Circle(f64),                    // Circle with radius
        Rectangle(f64, f64),             // Rectangle with width and height
        Triangle(f64, f64, f64),         // Triangle with three sides
    }

    fn match_shape(shape: Shape) {
        match shape {
            Shape::Circle(radius) => {
                println!("Circle with radius: {}", radius);
            }
            Shape::Rectangle(width, height) => {
                println!("Rectangle with width: {} and height: {}", width, height);
            }
            Shape::Triangle(a, b, c) if a + b > c && b + c > a && a + c > b => {
                println!("Valid Triangle with sides: {}, {}, {}", a, b, c);
            }
            Shape::Triangle(_, _, _) => {
                println!("Invalid Triangle, sides do not form a valid triangle.");
            }
        }
    }

    fn main() {
        let circle = Shape::Circle(5.0);
        let rectangle = Shape::Rectangle(10.0, 20.0);
        let triangle_valid = Shape::Triangle(3.0, 4.0, 5.0);
        let triangle_invalid = Shape::Triangle(1.0, 1.0, 10.0);

        match_shape(circle);
        match_shape(rectangle);
        match_shape(triangle_valid);
        match_shape(triangle_invalid);
    }
    ```

**Explanation**:
- We match against **enum variants** `Shape::Circle`, `Shape::Rectangle`, and `Shape::Triangle`.
- For the **triangle**, we add a **guard condition** (with `if`) to ensure the triangle inequality holds
(i.e., the sides form a valid triangle).

### 2. **Matching Nested Structs and Enums**

This example involves matching on nested structs and enums to demonstrate how Rust can handle deeper pattern
matching.

    ```rust
    #[derive(Debug)]
    enum Animal {
        Dog(String),
        Cat { name: String, age: u8 },
    }

    #[derive(Debug)]
    struct PetOwner {
        name: String,
        pet: Animal,
    }

    fn match_pet(owner: PetOwner) {
        match owner.pet {
            Animal::Dog(name) => println!("{} has a dog named {}", owner.name, name),
            Animal::Cat { name, age } if age > 2 => {
                println!("{} has an adult cat named {} ({} years old)", owner.name, name, age);
            }
            Animal::Cat { name, age } => {
                println!("{} has a kitten named {} ({} years old)", owner.name, name, age);
            }
        }
    }

    fn main() {
        let dog_owner = PetOwner {
            name: "Alice".to_string(),
            pet: Animal::Dog("Buddy".to_string()),
        };

        let adult_cat_owner = PetOwner {
            name: "Bob".to_string(),
            pet: Animal::Cat {
                name: "Whiskers".to_string(),
                age: 4,
            },
        };

        let kitten_owner = PetOwner {
            name: "Eve".to_string(),
            pet: Animal::Cat {
                name: "Fluffy".to_string(),
                age: 1,
            },
        };

        match_pet(dog_owner);
        match_pet(adult_cat_owner);
        match_pet(kitten_owner);
    }
    ```

**Explanation**:
- The `Animal` enum has variants for **Dog** and **Cat**.
- The `PetOwner` struct contains the name of the owner and a nested `Animal`.
- The `match` statement destructures both the **enum** and **struct**. It uses guards to check the 
  **age of the cat** and differentiate between adult cats and kittens.

### 3. **Matching on Multiple Patterns with Complex Guards**

This Ex matching against multiple patterns and using **guards** to handle complex conditions.

    ```rust
    fn match_number(x: i32) {
        match x {
            1..=5 => println!("Small number: {}", x),
            6..=10 => println!("Medium number: {}", x),
            11..=100 if x % 2 == 0 => println!("Even number between 11 and 100: {}", x),
            11..=100 => println!("Odd number between 11 and 100: {}", x),
            _ => println!("Number out of range or a very large number: {}", x),
        }
    }

    fn main() {
        let numbers = [3, 7, 44, 99, 101, 5, 200];
        
        for &num in &numbers {
            match_number(num);
        }
    }
    ```

**Explanation**:
- We use **range patterns** (`1..=5`, `6..=10`, `11..=100`) to match numbers in specific ranges.
- We also use a **guard** (`if x % 2 == 0`) to differentiate between even & odd nums in the range 11–100.
- Finally, we handle a **catch-all** case (`_`) for numbers outside the expected range.

### 4. **Matching on Complex Data Structures (Option, Result, and Nested Data)**

This ex combines the use of **`Option`, `Result`**, and **nested patterns** to handle complex data struct's.

    ```rust
    #[derive(Debug)]
    enum Status {
        Active,
        Inactive,
    }
    
    #[derive(Debug)]
    struct User {
        name: String,
        status: Status,
    }

    fn check_user_status(user: Option<User>) {
        match user {
            Some(User { name, status: Status::Active }) => {
                println!("{} is active", name);
            }
            Some(User { name, status: Status::Inactive }) => {
                println!("{} is inactive", name);
            }
            None => println!("No user provided"),
        }
    }

    fn check_result(result: Result<i32, String>) {
        match result {
            Ok(val) if val > 0 => println!("Success with positive value: {}", val),
            Ok(val) => println!("Success with non-positive value: {}", val),
            Err(e) => println!("Error: {}", e),
        }
    }

    fn main() {
        let active_user = Some(User {
            name: "Alice".to_string(),
            status: Status::Active,
        });
        
        let inactive_user = Some(User {
            name: "Bob".to_string(),
            status: Status::Inactive,
        });

        let error_result: Result<i32, String> = Err("Something went wrong".to_string());
        let success_result: Result<i32, String> = Ok(42);

        check_user_status(active_user);
        check_user_status(inactive_user);
        check_user_status(None);

        check_result(error_result);
        check_result(success_result);
    }
    ```

**Explanation**:
- We use the **`Option`** type to represent the possibility of a user being `None` or `Some(User)`.
- The **`Result`** type is used to handle success (`Ok`) or failure (`Err`), and we apply guards to
  differentiate between positive and non-positive values.
- We also match against **nested enums** (`Status::Active`, `Status::Inactive`).

### 5. **Matching on Multiple Types with Type Guards**

Rust allows **multiple pattern matching** and **type guards**, which help you match against specific types.

    ```rust
    enum Either<T, U> {
        Left(T),
        Right(U),
    }

    fn match_either(value: Either<i32, String>) {
        match value {
            Either::Left(x) if x > 0 => println!("Positive Left value: {}", x),
            Either::Left(x) => println!("Negative Left value: {}", x),
            Either::Right(s) if s.len() > 5 => println!("Long Right string: {}", s),
            Either::Right(s) => println!("Short Right string: {}", s),
        }
    }

    fn main() {
        let left_positive = Either::Left(42);
        let left_negative = Either::Left(-10);
        let right_long = Either::Right("Hello, world!".to_string());
        let right_short = Either::Right("Hi".to_string());

        match_either(left_positive);
        match_either(left_negative);
        match_either(right_long);
        match_either(right_short);
    }
    ```

**Explanation**:
- We define a generic enum `Either`, which can be either a `Left` value or a `Right` value.
- We use **guards** to check the value of `Left` (whether it's positive or negative) and `Right` 
  (whether the string is long or short).

### 6. **Matching Complex Enums with Multiple Variants and Guards**

This example demonstrates how to handle complex enums with multiple variants, each with its own behavior 
and guards.

    ```rust
    #[derive(Debug)]
    enum Event {
        Click { x: i32, y: i32 },
        KeyPress(char),
        MouseMove(i32, i32),
        Quit,
    }

    fn handle_event(event: Event) {
        match event {
            Event::Click { x, y } if x > 100 && y > 100 => {
                println!("Click at a large coordinate: ({}, {})", x, y);
            }
            Event::Click { x, y } => {
                println!("Click at coordinate: ({}, {})", x, y);
            }
            Event::KeyPress(c) => println!("Key Pressed: {}", c),
            Event::MouseMove(x, y) => println!("Mouse moved to: ({}, {})", x, y),
            Event::Quit => println!("Quit event triggered"),
        }
    }

    fn main() {
        let click = Event::Click { x: 150, y: 200 };
        let key_press = Event::KeyPress('A');
        let mouse_move = Event::MouseMove(300, 400);
        let quit = Event::Quit;

        handle_event(click);
        handle_event(key_press);
        handle_event(mouse_move);
        handle_event(quit);
    }
    ```

**Explanation**:
- This ex involves an **enum with multiple variants**, some of which contain data 
  (e.g., `Click { x, y }` and `MouseMove(x, y)`).
- We apply **guards** to check for specific conditions, such as whether the click coordinates are large.

---

### Conclusion

These **advanced `match` examples** explore:

- **Pattern matching on structs, enums, and tuples**.
- **Guards** to introduce conditional logic inside a match arm.
- **Destructuring** of nested data structures.
- Combining different types of data (**Option**, **Result**, and more).

By working through these examples, you'll become comfortable using Rust's powerful `match` expression to 
handle complex control flow scenarios and data matching.

