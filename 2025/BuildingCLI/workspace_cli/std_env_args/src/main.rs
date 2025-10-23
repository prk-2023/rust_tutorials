use std::env;

fn main() {
    // let args = env::args();
    // for i in args {
    //     println!("=>{i}");
    // }

    // or
    // let args = env::args();
    // for (i, value) in args.enumerate() {
    //     println!("Argument #{} = {}", i, value);
    // }

    //or
    let args: Vec<String> = env::args().collect(); /* collect() method on iterator turn in to relevant type*/

    let tmpargs = args.clone(); // clone args to prevent from moving
    dbg!(tmpargs);

    // Save the arguments to variables
    let query = &args[1];
    let file_path = &args[2];
    println!("Searching for {query}");
    println!("In file : {file_path}");
}
