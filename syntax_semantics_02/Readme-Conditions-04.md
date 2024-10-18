# Conditions:


- "if" is a specific form of a more general concept, the ‘branch’, whose name comes from a branch in a 
  tree: a decision point, where depending on a choice, multiple paths can be taken.

    if x == 5 {
       println!("x is five!");
    } else if x == 6 {
        println!("x is six!");
    } else {
        println!("x is not five or six :(");
    }


    #![allow(unused_variables)]
    fn main() {
        let x = 5;
        let y = if x == 5 { 10 } else { 15 }; // y: i32
    } // this works are if is an expression

- loop: 
    The infinite loop is the simplest form of loop available in Rust. 

    loop {
        println!(" print this forever! ");
    }

- while: ( used when we are unsure how many times its required to loop)

    #![allow(unused_variables)]
    fn main() {
        let mut x = 5; // mut x: i32 
        let mut done = false; // mut done: bool 

        while !done {
            x += x - 3;
            println!("{}", x);

            if x % 5 == 0 {
                done = true;
            }
        }
    }

- for: Rust 'for' not same as c and looks as below:

    for x in 0..10 {

    }

    or in a more abstract way as 
    for item in list {
    ....
    }

    iterator : gives back one element from a group of collection per iteration of the loop.

-  Enumeration: when we want to keep track of how many times the program have looped we can use .enumerate()

    for (index, value) in (5..10).enumerate() {
        println!("index = {} and value = {}", index, value);
    }

    index = 0 and value = 5
    index = 1 and value = 6
    index = 2 and value = 7
    index = 3 and value = 8
    index = 4 and value = 9

    Don't forget to add the parentheses around the range.

- iterators: 

    let lines = "hello\nworld".lines();
    for (linenumber, line) in lines.enumerate() {
        println!("{}: {}", linenumber, line);
    }

 - Terminate, skip a iteration earlier:
    break, and continue

- loop labels: specify name to a loop 

    useful in case of nested loops: 

    example:

    #![allow(unused_variables)]
    fn main() {
        'outer: for x in 0..10 {
            'inner: for y in 0..10 {
                if x % 2 == 0 { continue 'outer; } // Continues the loop over `x`.
                if y % 2 == 0 { continue 'inner; } // Continues the loop over `y`.
                println!("x: {}, y: {}", x, y);
            }
        }
    }

