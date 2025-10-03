/*
 * Explanation by topic:
 * ---------------------
 *
 * Option<T>: Used when a value may or may not be present (Some or None).
 * Result Enum: Used for functions that may succeed (Ok) or fail (Err).
 * Ordering Enum: Result of comparisons (Less, Equal, Greater).
 * IpAddr, SocketAddr Enums: Represent IP addresses and socket addresses supporting both IPv4 and IPv6.
 * Poll Enum: Used in async programming to indicate readiness or pending state.
 * SeekFrom Enum: Used to specify how to seek within a file (Start, Current, or End).
 * IoPriority: Simulated here to represent different priority levels for I/O operations.
 * UnixRights: Simulated to represent different file descriptor permissions in Unix.
 * SocketType: Simulated to represent socket types (Stream, Datagram, Raw).
 *
 * Other concepts
 * Pattern Matching with match and if let (though partially covered)
 * Enum with associated data (variants holding different types of data)
 * Enum methods and impl blocks
 * Using enums with Option and Result chaining (? operator, combinators like map, and_then)
 * Recursive enums (e.g., linked lists, trees)
 * Enum derive traits (e.g., Debug, Clone, Copy, PartialEq)
 * C-like enums with explicit discriminants
 * Using enums in const and static contexts
 * Using enums with pattern guards
 * Enums and unsafe code (rare but sometimes used)
 */
use std::cmp::Ordering;
use std::fmt;
use std::io::{self, SeekFrom};
use std::net::{IpAddr, SocketAddr};
use std::task::Poll; // used below to implement a Display trait for enums ( similar to strum crate)

// Simulated IoPriority enum
#[derive(Debug, Clone, Copy)]
enum IoPriority {
    Low,
    Medium,
    High,
}

// Simulated UnixRights enum with associated data for file descriptors
#[derive(Debug)]
enum UnixRights {
    Read(u32), // file descriptor number
    Write(u32),
    Exec(u32),
}

// Example: enum with associated data
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// Enum with explicit discriminants (C-like enum)
#[repr(u16)]
enum HttpStatus {
    Ok = 200,
    NotFound = 404,
    InternalServerError = 500,
}

// Recursive enum (simple linked list)
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self, elem: i32) -> List {
        List::Cons(elem, Box::new(self))
    }

    fn len(&self) -> usize {
        match self {
            List::Cons(_, tail) => 1 + tail.len(),
            List::Nil => 0,
        }
    }
}

// Implement a Display trait for enum
// First Define the Enum ( ex: enum that  represents different types of currency.
enum Currency {
    TWD, // NewTaiwan Dollar
    INR, // Indian Rupee
    JPY, // Japanese Yen
    CNY, // Chinese Yuan
}

// Next we implement a Display trait:
// This defines how currency value should be formatted as string
// i.e this crate helps to represent enum to string

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        //use match expression to handle each enum variant:
        match self {
            Currency::TWD => write!(f, "New Taiwan Dollar!"),
            Currency::INR => write!(f, "Bharat Rupee!"),
            Currency::JPY => write!(f, "Japanese Yen!"),
            Currency::CNY => write!(f, "Chinese Yuan!"),
        }
    }
}

fn main() {
    // Using Message enum with associated data
    let msg = Message::Move { x: 10, y: 20 };
    match msg {
        Message::Quit => println!("Quit message"),
        Message::Move { x, y } => println!("Move to x: {}, y: {}", x, y),
        Message::Write(text) => println!("Write message: {}", text),
        Message::ChangeColor(r, g, b) => println!("Change color to RGB({}, {}, {})", r, g, b),
    }

    // Using explicit discriminants
    let status = HttpStatus::NotFound;
    println!("HTTP Status code: {}", status as u8);

    // Using recursive enum List
    let list = List::new().prepend(1).prepend(2).prepend(3);
    println!("List length: {}", list.len());

    // Using UnixRights enum with associated file descriptor data
    let rights = UnixRights::Read(42);
    match rights {
        UnixRights::Read(fd) => println!("Read rights for fd {}", fd),
        UnixRights::Write(fd) => println!("Write rights for fd {}", fd),
        UnixRights::Exec(fd) => println!("Exec rights for fd {}", fd),
    }

    // Pattern guard example with Ordering
    let x = 5;
    let y = 10;
    match x.cmp(&y) {
        Ordering::Less if x % 2 == 1 => println!("{} is less and odd", x),
        Ordering::Less => println!("{} is less but even", x),
        Ordering::Greater => println!("{} is greater", x),
        Ordering::Equal => println!("{} is equal to {}", x, y),
    }

    // Using if let for Option
    let maybe_name: Option<String> = Some("Alice".to_string());
    if let Some(name) = maybe_name {
        println!("Found a name: {}", name);
    } else {
        println!("No name found");
    }

    let price_tag = Currency::TWD;
    let exchange_rate = Currency::JPY;
    let saving_acct = Currency::INR;
    println!("Required currency is {}", price_tag);
    println!("Tracking exchange rate for {}", Currency::JPY);

    let message = format!("Your savings are in {}", saving_acct);
    println!("{}", message);
}
