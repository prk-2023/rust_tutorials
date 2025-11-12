// use std::rc::Rc;
use std::sync::Arc;
use std::thread;

fn main() {
    // let dog = Rc::new(String::from("Lica!"));
    // let dog_ref1 = Rc::clone(&dog);
    // let _dog_ref2 = Rc::clone(&dog);
    //
    // println!("Count: {}", Rc::strong_count(&dog)); // Count: 3
    // println!("Dog name: {}", dog_ref1);
    let dog = Arc::new(String::from("Ralf"));
    let dog_clone = Arc::clone(&dog);

    let handle = thread::spawn(move || {
        println!("From thread: {}", dog_clone);
    });
    println!("From main: {}", dog);
    handle.join().unwrap();
}
