//Attributes : passed to compiler
#[allow(dead_code)]
#[allow(unused_variables)]
use std::io;

struct Person {
    name: String,
    age: u32,
}

fn describe(person: &Person) {
    println!("{} is {} years old ", person.name, person.age);
}
//Tuple struct
#[allow(dead_code)]
struct Point(i32, i32);
#[allow(dead_code)]
struct Color(u8, u8, u8);

//---
#[allow(dead_code)]
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

//---
// Define a Rectangle structure
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// Implement methods for the Rectangle structure
impl Rectangle {
    // Constructor method to create a new Rectangle
    fn new(width: u32, height: u32) -> Rectangle {
        Rectangle { width, height }
    }

    // Method to calculate the area of the Rectangle
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // Method to calculate the perimeter of the Rectangle
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }

    // Method to scale the Rectangle by a factor
    fn scale(&mut self, factor: u32) {
        self.width *= factor;
        self.height *= factor;
    }
}

fn main() {
    println!(
        "struct example selection: \
            \n 1. Named Structure example \
            \n 2. tuple structure example \
            \n Input your choice:"
    );

    let mut selection_item = String::new();

    //selection_item = io::stdin().read_line(&mut selection_item);
    match io::stdin().read_line(&mut selection_item) {
        Ok(n) => {
            println!("Number of bytes read: {n}");
        }
        Err(_) => println!("Faild to read input value"),
    };
    //convert the input to u32:
    let selection_item: u32 = match selection_item.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("requires a u32 input, try again");
            selection_item.clear();
            0 // we pass 0 since in Rust match statement when used in assignment must return a value
              // this can be omited if the logic is inside a loop { } and 0 can be replaced with
              // continue or break as per the program logic
        }
    };

    match selection_item {
        1 => {
            // example of Named structure
            let mut user1 = Person {
                name: String::from("batman"),
                age: 27,
            };
            describe(&user1);
            user1.age = 28;
            println!("user: {}  is age : {} old", user1.name, user1.age);

            let usr2_name = String::from("mesha");
            let user2 = Person {
                name: usr2_name,
                ..user1
            };
            describe(&user2);
        }
        2 => {}
        _ => println!("Error! Invalid Input try again"),
    }
    //
    let user1 = User {
        active: true,
        username: String::from("someusername"),
        email: String::from("someusername@m.com"),
        sign_in_count: 1,
    };

    //create instance from other instance:
    let _user2 = User {
        email: String::from("another@email_addr.com"),
        ..user1 // this copies most of the elements from user1
    };

    //Tuple Struct:

    let _back = Color(0, 0, 0);
    let _white = Color(255, 255, 255);

    // Unit like struct:
    struct IamEmptyStruct;

    let _subject = IamEmptyStruct;

    let _builtuser = build_user(
        String::from("externaluser@internal.in"),
        String::from("xyz"),
    );

    //unit like struct
    struct Electron {} // use empty braces
    struct Proton {}

    let _x = Electron {};
    let _y = Proton {};

    // Use the impl methods defined up
    // Create a new Rectangle using the constructor method
    let mut rect = Rectangle::new(3, 4);

    // Calculate and print the area and perimeter of the Rectangle
    println!("Area: {}", rect.area());
    println!("Perimeter: {}", rect.perimeter());

    // Scale the Rectangle by a factor
    rect.scale(2);

    // Calculate and print the new area and perimeter of the Rectangle
    println!("New Area: {}", rect.area());
    println!("New Perimeter: {}", rect.perimeter());
}

// Return a struct from a function
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username,
        sign_in_count: 1,
    }
}
