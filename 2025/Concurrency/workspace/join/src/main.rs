use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("From Spawned thread : {i}");
            thread::sleep(Duration::from_millis(100));
        }
    });

    for i in 1..5 {
        println!("From Main thread: {i}");
        thread::sleep(Duration::from_millis(100));
    }
    handle.join().unwrap(); // this puts the main threads to wait till the spawned threads have
                            // finished there task.
}
