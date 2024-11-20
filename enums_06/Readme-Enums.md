# Rust Enums:

- Rust enums is a type that represents data that is one of several possible variants. Each variant in the
  'enum' can optionally have data associated with it.

- Enums are way to define a set of named values. mainly useful when you have a fixed set of distinct values
  that have a particular meaning in the code.

- Define enum:

    enum Color {  // like structures Enums names begin with Capital letter 
        Red, 
        Green, 
        Blue,
    }

In this example, we define an enum called `Color` with three possible values: `Red`, `Green`, and `Blue`.

Enum Values
------------

Enum values can be used in the same way as any other value in Rust. Here is an example of how to use the 
`Color` enum:

```rust
fn main() {
    let color = Color::Green;
    match color {
        Color::Red => println!("The color is red"),
        Color::Green => println!("The color is green"),
        Color::Blue => println!("The color is blue"),
    }
}
```

Above we define a variable `color` and assign it the value `Color::Green`.
We then use a `match` statement to print out a message depending on the value of `color`.

Enum Values with Associated Data
---------------------------------

One of the powerful features of Rust enums is that they can have associated data. Here is an example:

```rust
enum IpAddress {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let ip = IpAddress::V4(127, 0, 0, 1);
    match ip {
        IpAddress::V4(a, b, c, d) => println!("IPv4 address: {}.{}.{}.{}", a, b, c, d),
        IpAddress::V6(addr) => println!("IPv6 address: {}", addr),
    }
}
```

In this example, we define an enum called `IpAddress` with two possible values: `V4` and `V6`. The `V4`
value has four associated `u8` values, and the `V6` value has an associated `String` value.

Methods on Enums
-----------------

Enums in Rust can also have methods. Here is an example:

```rust
enum Color {
    Red,
    Green,
    Blue,
}

impl Color {
    fn to_string(&self) -> String {
        match self {
            Color::Red => "red".to_string(),
            Color::Green => "green".to_string(),
            Color::Blue => "blue".to_string(),
        }
    }
}

fn main() {
    let color = Color::Green;
    println!("{}", color.to_string());
}
```

In this example, we define a method called `to_string` on the `Color` enum. This method returns a `String` 
representation of the enum value.

Comparison to C and C++
-------------------------

Enums in Rust are more powerful than enums in C and C++ in several ways:

* **Associated data**: Rust enums can have associated data, which allows for more complex and expressive
  data structures. In C and C++, enums are simply a way to define a set of named values, without any 
  associated data.

* **Methods**: Rust enums can have methods, which allows for more object-oriented programming. In C and C++,
  enums are not objects and cannot have methods.
* **Pattern matching**: Rust enums can be used with pattern matching, which allows for more expressive and
  concise code. In C and C++, enums are typically used with switch statements, which can be more verbose and
  error-prone.
* **Type safety**: Rust enums are type-safe, which means that the compiler will prevent you from assigning
  an invalid value to an enum variable. In C and C++, enums are not type-safe, which can lead to bugs and 
  errors.

Why Rust Enums are Unique and Powerful
----------------------------------------

Rust enums are unique and powerful for several reasons:

* **Associated values**: Rust enums can have associated values, which allows you to store data with each
  variant of the enum.

* **Methods**: Rust enums can have methods, which allows you to define behavior for each variant of the enum

* **Pattern matching**: Rust enums are often used with pattern matching, which allows you to handle
  different variants of an enum in a concise and expressive way.

* **Type safety**: Rust enums are type-safe, which means that the compiler will prevent you from using an
  enum in a way that's not valid.

* **Null safety**: Rust enums are null-safe, which means that you can't have a null enum value.

Example Use Cases
-----------------

Here are some example use cases for Rust enums:

* **Error handling**: Rust enums can be used to define a set of error codes, each with its own associated
  data.

* **State machines**: Rust enums can be used to define a set of states in a state machine, each with its
  own associated data and behavior.

* **Configuration options**: Rust enums can be used to define a set of configuration options, each with its
  own associated data and behavior.

Conclusion
----------

Enums in Rust are a powerful tool for defining and working with sets of named values. 
They offer a range of features, including associated data, methods, and pattern matching, that make them 
more expressive and flexible than enums in C and C++. 
By using Rust enums, you can write more concise, expressive, and type-safe code.

---

Rust enums are special and they are designed with a deep thought allowing them to be used in a way that
expand how programming can be done.

- For a regular enum like in other langs 

    enum Color {
        Red,
        Blue,
        Green,
    }
    // a function that will print the color
    fn print_color ( color : Color ) {
    
        match color {
            Color::Red => println!("red"),
            Color::Blue => println!("blue"),
            Color::Green => println!("green"),
        }

    }

    Instead of using a switch(){} operation  like in other languages Rust we use the "match" operation.
    Unlike switch which does comparision, Rust match does pattern matching.
    So in the above code if the color was Red then match would print 'red'... 

    enum Color {
        Yellow,
        Red,
        Blue,
        Green,
    }
    If we add 'Yellow' color to the Color enum then the match() Statement will through an error on
    compilation. Forcing the programmer to handle the messing case "Yellow".

=>  where as in other languages there is no way to track this .. 

- Apart from the above advantage Rust enums allow to attach functions, which many programming languages do
  not support.
  These functions effectively become methods and we declare "implementations" for the enum Color as below:


    enum Color {
        Red,
        Blue,
        Green,
    }

    impl Color {
        fn green_part (&self) -> bool {
            match self {
                Color::Yellow => true,
                Color::blue => true,
                _ => false,
            }
        }
        fn is_green (&self) -> bool {    //this fun takes self and pattern matches on itself
            if let Color::Green = self { //enums that are of type color green will return true else false
                return true;
            }
            return false;
        }
    }
    // a function that will print the color
    fn print_color ( color : Color ) {
    
        match color {
            Color::Red => println!("red"),
            Color::Blue => println!("blue"),
            Color::Green => println!("green"),
            Color::Yellow => println!("yellow"),
        }
    }

=> Methods support of enums are very useful to work with in Rust. 

- Additionally with rust enums we can do as below:
    
    // define a Custom struct with fields
    struct Custom {
        name: String,
        age: usize,
    }
    // define a enum with the below methods:
    enum Item {
        Foo(String),
        Bar(usize),
        Baz(Custom),
    }

    let foo =  Item::Foo(String::from("hello")); // create a enum foo and pass in a string "hello"
    if let Item::Foo(s) = foo {   // pattern match the above item foo with a subtype s 
        println!("{}",s );        // where s is the sub-type that was passed in here its String and then
    }                             // finally we can print the s as string.
    
    this above code: Item::Foo(s) will match the "foo" variable and bind the string value inside Item::Foo
    to the variable 's', and "if let" statement will then print the string value. We can also use the 
    match foo { Item::Foo(s) => println!("{}",s),...}

We can do the same operation with Bar and Baz:

    let another_val = Item::Baz(Custom { name : String::from("daybreak"), agr: 69 });
    if let Item::Baz(custom) = another_val{
        println!("{}", custom.age);
    }


---

In Rust, the `Option` type is a way to handle situations where a value might be present or absent. 
It's a fundamental concept in Rust programming, and it's used extensively throughout the language.

**What is an Option?**
----------------------

An `Option` is an enum that can have two possible values:

* `Some(value)`: This means that a value is present.
* `None`: This means that no value is present.

**Creating Options**
--------------------

You can create an `Option` using the `Some` and `None` variants:

```rust
let some_value = Some(5);  // Some(i32)
let no_value = None;  // None
```

**Pattern Matching**
--------------------

One of the most common ways to work with `Option`s is through pattern matching. 
You can use a `match` statement to handle both `Some` and `None` cases:

```rust
let some_value = Some(5);

match some_value {
    Some(x) => println!("The value is: {}", x),
    None => println!("No value is present"),
}
```

**Unwrapping Options**
----------------------

If you're certain that an `Option` has a value, you can use the `unwrap` method to get the value out. 
However, if the `Option` is `None`, the program will panic:

```rust
let some_value = Some(5);
let value = some_value.unwrap();  // value is now 5

let no_value = None;
// let value = no_value.unwrap();  // This will panic!
```

**Using if let**
-----------------

Another way to handle `Option`s is by using `if let`. 
This allows you to handle the `Some` case without having to specify what to do with the `None` case:

```rust
let some_value = Some(5);

if let Some(x) = some_value {
    println!("The value is: {}", x);
}
```

**Using expect**
-----------------

If you want to provide a custom error message when unwrapping an `Option`, you can use the `expect` method:

```rust
let some_value = Some(5);
let value = some_value.expect("No value is present");  // value is now 5

let no_value = None;
// let value = no_value.expect("No value is present");  // This will panic with the custom message!
```

**Using map and and_then**
---------------------------

`Option` also has methods like `map` and `and_then` that allow you to transform the value inside the 
`Option` without having to unwrap it:

```rust
let some_value = Some(5);
let mapped_value = some_value.map(|x| x * 2);  // mapped_value is now Some(10)

let no_value = None;
let mapped_value = no_value.map(|x| x * 2);  // mapped_value is now None
```

```rust
let some_value = Some(5);
let and_then_value = 
    some_value.and_then(|x| if x > 10 { Some(x * 2) } else { None });  // and_then_value is now None

let some_value = Some(15);
let and_then_value = 
    some_value.and_then(|x| if x > 10 { Some(x * 2) } else { None });  // and_then_value is now Some(30)
```

**Real-World Example**
----------------------

Here's an example of using `Option` to handle a situation where a value might be present or absent:

```rust
fn get_user_name(user_id: i32) -> Option<String> {
    // Simulate a database query
    if user_id == 1 {
        Some("John Doe".to_string())
    } else {
        None
    }
}

fn main() {
    let user_id = 1;
    let user_name = get_user_name(user_id);

    match user_name {
        Some(name) => println!("Hello, {}!", name),
        None => println!("User not found"),
    }
}
```

In ex, the `get_user_name` function returns an `Option` containing the user's name if the user is found, 
or `None` if the user is not found. The `main` function then uses pattern matching to handle both cases.

The `Option` type in Rust is indeed a part of the standard library. 
However, it is not automatically brought into scope by default.

In Rust, the standard library is divided into several modules, and each module needs to be 
imported using the `use` keyword to bring its contents into scope.

The `Option` type is defined in the `std` module, which is the root module of the standard library. 
To use `Option`, you need to import it explicitly using the `use` keyword:

```rust
use std::option::Option;
```

However, this is not the most common way to import `Option`. 
Instead, you can import it using the following syntax:

```rust
use std::option::Option::{Some, None};
```

This imports the `Option` enum and its variants `Some` and `None` directly into the current scope.

Alternatively, you can also import the entire `std::option` module using the following syntax:

```rust
use std::option::*;
```

This imports all the contents of the `std::option` module, including the `Option` enum and its variants, 
into the current scope.

In practice, you rarely need to import `Option` explicitly, as it is often used in conjunction with other 
standard library types and functions that import it implicitly. 

For example, when working with `Result`, `Option` is often used as a return type, and the `Result` module 
imports `Option` implicitly.

