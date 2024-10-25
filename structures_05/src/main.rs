use std::io;

#[allow(dead_code)]
struct Person {
    name: String,
    age: u8,
}

fn describe(person: &Person) {
    println!("{} is {} years old ", person.name, person.age);
}
//Tuple struct
struct Point(i32, i32);

#[allow(unused_variables)]
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
}
