//extern crate rand; // this is ommited since 2018, which told the rust compiler to link against
//the "rand"
use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        //generate a random number
        //let rand_num: u32 = rand::thread_rng().gen_range(0..=100);
        let rand_num: u32 = rand::thread_rng().gen_range(1..101);

        //let rand_num: u32 = rand::thread_rng().gen_range(0..101);

        // read user input and convert to u32: ( illegal input print usage:)
        let mut guess = String::new(); // this will generate a new mutable string

        /*io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line"); // passes a mutable reference to std::io::stdin().read_line()
        */
        match io::stdin().read_line(&mut guess) {
            Ok(_) => println!("You guessed: {}", guess),
            Err(e) => println!("Error reading input: {}", e),
        }

        //convert the number to u32:
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Input not u32 ");
                continue;
            }
        };
        //hit zero to break the program loop
        if guess == 0 {
            break;
        }

        println!("randand number : {}", rand_num);
        println!("guess  : {}", guess);
        //comparision: use the string cmp method which returns Ordering type:
        match guess.cmp(&rand_num) {
            Ordering::Less => println!("smaller"),
            Ordering::Greater => println!("Larger"),
            Ordering::Equal => {
                println!("You Win!");
                break;
            }
        }
    }
}
