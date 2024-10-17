use std::io;

fn main() {
    loop {
        println!("--------------------");
        println!("Select the example to run");
        println!("0. quite the program");
        println!("1. Syntax and Symantics");

        //read input and convert to u32
        let mut selection_no = String::new();
        match io::stdin().read_line(&mut selection_no) {
            Ok(_) => println!("You guessed: {}", selection_no),
            Err(e) => println!("Error reading input: {}", e),
        }
        let selection_no: u32 = match selection_no.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("requires a u32 input, try again");
                continue;
            }
        };
        match selection_no {
            0 => {
                println!("Thank You hava a good day");
                break;
            }
            1 => {
                println!(
                    "variable
                    binding,
                    patterns,
                    type annotation,
                    mutability,
                    initalizing binding,
                    scope shadowing"
                );
                let _x = 5; // x type is infered by rust compiler. in this case is i32.
                            //rust variable binding goes a level up and the let statement is "pattern" not a
                            //variable name: this means we can do some thing as below:
                let (_x, _y) = (1, 5); // _x value is
            }

            _ => println!("Error! Invalid Input try again"),
        };
    }
}
