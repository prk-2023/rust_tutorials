use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0)); // Wrap the mutex in an Arc
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter); // Clone the Arc to share ownership
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap(); // Acquire the lock
            *num += 1; // Increment the counter
        });
        handles.push(handle); // Collect thread handles
    }

    for handle in handles {
        handle.join().unwrap(); // Wait for all threads to finish
    }

    // Print the result
    println!("Result: {}", *counter.lock().unwrap());
}
