fn main() {
    println!("=== Part 1: Simple Ownership Basics ===");
    part1_simple_ownership();

    println!("\n=== Part 2: Ownership and Functions ===");
    part2_ownership_and_functions();

    println!("\n=== Part 3: Borrowing and References ===");
    part3_borrowing_and_references();

    println!("\n=== Part 4: Mutable References and Slices ===");
    part4_mutable_references_and_slices();

    println!("\n=== Part 5: Ownership with Structs and Methods ===");
    part5_ownership_with_structs();
}

// Part 1: Simple Ownership Basics
fn part1_simple_ownership() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2, s1 is no longer valid

    // println!("{}", s1); // Error: value borrowed here after move
    println!("{}", s2); // works fine

    let x = 5;
    let y = x; // for integers, copy happens, not move
               // This is true for all types that implement "Copy Trait"
    println!("x = {}, y = {}", x, y); // both valid
}

// Part 2: Ownership and Functions
fn part2_ownership_and_functions() {
    let s = String::from("ownership");
    takes_ownership(s);
    // println!("{}", s); // Error: s moved into function

    let x = 10;
    makes_copy(x); //x is integer and it implements "copy trait"
    println!("x still usable after function: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("Took ownership of: {}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("Got a copy of: {}", some_integer);
}

// Part 3: Borrowing and References
fn part3_borrowing_and_references() {
    let s1 = String::from("borrow me");
    let len = calculate_length(&s1);
    println!("Length of '{}' is {}", s1, len);

    // Mutable reference example
    let mut s2 = String::from("change me");
    change(&mut s2);
    println!("Changed string: {}", s2);

    // Multiple immutable references allowed
    let r1 = &s1;
    let r2 = &s1;
    println!("Two immutable refs: {}, {}", r1, r2);

    // let r3 = &mut s1; // Error: cannot borrow `s1` as mutable because it is also borrowed as immutable
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) {
    let tmp_string = some_string.clone();
    some_string.push_str("!");
    some_string.push_str(&tmp_string);
    some_string.push('!');
}

// Part 4: Mutable References and Slices
fn part4_mutable_references_and_slices() {
    let mut s = String::from("hello");

    {
        let r1 = &mut s;
        r1.push_str(" world");
        println!("r1: {}", r1);
    } // r1 goes out of scope here

    let r2 = &mut s;
    r2.push_str("!!!");
    println!("r2: {}", r2);

    // String slices (immutable borrows)
    let hello = &s[0..5];
    let world = &s[6..11];
    println!(
        "string s = {}
        create to new string using slices of string \'s\'
        hello :{} and world {}",
        s, hello, world
    );
    println!("Slices: '{}' and '{}'", hello, world);
}

// Part 5: Ownership with Structs and Methods
// use the Debug Attribute: which helps to automatically generate code that allows a type to be
// printed in a readable format for debugging. i.e it provides println!() macro to use {:?}
// formatter on a 'Book' instance, with out this directive the code would fail to compile
#[derive(Debug)]
struct Book {
    title: String,
    author: String,
}

impl Book {
    // method that takes ownership of self
    fn consume(self) {
        println!("Consuming book: {:?}", self);
        // self goes out of scope and is dropped here
    }

    // method borrowing self immutably
    fn describe(&self) {
        println!("'{}' by {}", self.title, self.author);
    }

    // method borrowing self mutably
    fn change_title(&mut self, new_title: &str) {
        self.title = String::from(new_title);
    }
}

fn part5_ownership_with_structs() {
    let mut book = Book {
        title: String::from("1984"),
        author: String::from("George Orwell"),
    };

    book.describe();

    book.change_title("Animal Farm");
    book.describe();

    book.consume();
    // book.describe(); // Error: book was moved
}
