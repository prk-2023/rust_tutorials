use std::{i32, io};

fn main() {
    loop {
        println!("--------------------");
        println!("Select the example to run");
        println!("0. quite the program");
        println!("1. variable assignemnet ");
        println!("2. Functions and return vals ");
        println!("3. match, conditions, loops");
        println!("4. vectors");
        println!("5. Ownership, Reference, borrowing, slices ");

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
                    "variable \
                    binding, \
                    patterns, \
                    type annotation, \
                    mutability, \
                    initalizing binding,\
                    scope shadowing \n"
                );
                let _x = 5; // x type is infered by rust compiler. in this case is i32.
                            //rust variable binding goes a level up and the let statement is "pattern" not a
                            //variable name: this means we can do some thing as below:
                let (_x, _y) = (1, 5); // _x value is
            }
            2 => {
                //functions: example greetings ( ) and funtion_ptr()
                let mut var_int = 0;
                println!("var_int: {}", var_int);
                let (times, result) = greeting("hello", 3, &mut var_int);
                println!("times: {}", times);
                println!("result: {}", result);
                println!("var_int: {}", var_int);
                //function pointer
                fn plus_one(i: i32) -> i32 {
                    i + 1
                }
                let f = plus_one;
                let _six = f(5);
            }
            3 => {
                //match example
                let x = 5;
                match x {
                    1 | 2 | 3 => println!("x is 1 or 2 or 3"),
                    4..=6 => println!("x is in between 4 and 6"),
                    _ if x % 2 == 0 => println!("x is even"),
                    _ => println!("x is something else"),
                };
                for _ in 0..x {
                    println!("for iteration time = {x} ");
                }
                let mut x = 5; // shadowing
                while x > 0 {
                    println!("while iteration time = {x} ");
                    x -= 1;
                }
                // keep track of iteration index:
                let _val = 5;
                for (index, _val) in (5..10).enumerate() {
                    println!("index = {} and value = {}", index, _val);
                }
            }
            4 => {
                // create a vector :
                let _v1 = vec![0; 10]; //vector of ten zeros.
                let _v = vec![1, 2, 3]; // or  let v = vec![0; 10]; // Vector of ten zeros.

                // safe access of out off bound index:
                match _v.get(7) {
                    // safe access : get returns None for outof bound access.
                    Some(x) => println!("Item 7 is {}", x),
                    None => println!("Sorry, this vector is too short."),
                }
                //unsafe outof bound access ( Gives error while running )
                //println!("Item 7 is {}", _v[7]);
                /* operations and methods on vectors*/
                let mut my_vector: Vec<i32> = Vec::new();
                println!("Initial vector: {:?}", my_vector);
                // Reserve memory to avoid reallocations
                my_vector.reserve(10);
                println!("Capacity after reserve: {}", my_vector.capacity());

                // Add elements to the vector
                my_vector.push(10);
                my_vector.push(20);
                my_vector.push(30);
                println!("Vector after adding elements: {:?}", my_vector);

                // Access elements of the vector
                println!("First element: {}", my_vector[0]);
                println!("Second element: {}", my_vector.get(1).unwrap());

                // Update value at an index
                my_vector[0] = 100;
                println!("Vector after updating the first element: {:?}", my_vector);

                // Add a new element to the vector
                my_vector.push(40);
                println!("Vector after adding a new element: {:?}", my_vector);

                // Delete an element from the vector
                my_vector.remove(1);
                println!("Vector after removing the second element: {:?}", my_vector);

                // Additional useful methods
                println!("Vector length: {}", my_vector.len());
                println!("Is vector empty? {}", my_vector.is_empty());
                println!("Vector capacity: {}", my_vector.capacity());

                // Iterate over the vector
                for element in &my_vector {
                    println!("Element: {}", element);
                }

                // Sort the vector
                my_vector.sort();
                println!("Sorted vector: {:?}", my_vector);

                // Reverse the vector
                my_vector.reverse();
                println!("Reversed vector: {:?}", my_vector);

                // Create an iterator that borrows the vector's elements
                for element in my_vector.iter() {
                    println!("Element: {}", element);
                }
                // Slice the vectors
                let sliced_vector = &my_vector[1..3];
                println!("Sliced vector: {:?}", sliced_vector);

                let mut my_vector1: Vec<i32> = Vec::with_capacity(10);
                println!("Capacity after reserve: {}", my_vector1.capacity());

                // Add elements to the vector
                my_vector1.push(10);
                my_vector1.push(20);
                my_vector1.push(30);
                my_vector1.push(40);
                my_vector1.push(50);
                my_vector1.push(60);
                my_vector1.push(70);

                // Borrow the vector's elements
                let borrowed_vector = &my_vector1;
                println!("Borrowed vector: {:?}", borrowed_vector);

                // Clone the vector
                let cloned_vector = my_vector1.clone();
                println!("Cloned vector: {:?}", cloned_vector);

                // Create an iterator that takes ownership of the vector's elements
                for element in borrowed_vector {
                    println!("Element: {}", element);
                }

                my_vector.clear();
                my_vector1.clear();
                println!("my_vector after clear: {:?}", my_vector);
                println!("my_vector1 after clear: {:?}", my_vector1);
            }
            5 => {
                //ownership
                let s = String::from("hello"); // s is the owner of the string "hello"
                let _t = s; // _t takes ownership of the string "hello", s is no longer the owner
                            // In this case string 's' gets heap allocation and it gets ownership.

                //borrowing and reference
                let s = String::from("hello"); // s is the owner of the string "hello"
                let _len = calculate_length(&s); // s is borrowed, but still owns the string
            }

            _ => println!("Error! Invalid Input try again"),
        };
    }
}

fn greeting(message: &str, times: u8, int_var: &mut i32) -> (u8, i32) {
    let mut times_updated: u8 = times;
    for _ in 0..times {
        println!("{}", message);
        *int_var += 1;
        times_updated -= 1;
    }
    (times_updated, *int_var)
}

fn calculate_length(text: &str) -> usize {
    text.len()
}
