use std::thread;
use std::time::Duration;

#[tokio::main]
async fn main() {
    os_thread_main();
    user_thread_main().await;
}
fn os_thread_main() {
    // Create an OS thread
    let handle = thread::spawn(|| {
        for i in 1..5 {
            println!("Hi from the OS thread! Count: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Do work in the main thread
    println!("Hi from the main thread!");

    // Wait for the thread to finish
    handle.join().unwrap();
}
async fn user_thread_main() {
    // Create a user-level "task" (Green thread)
    let handle = tokio::spawn(async {
        println!("Hi from a lightweight Tokio task!");
        // This yields the thread instead of blocking it
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        println!("Task finished.");
    });

    println!("Main function is still running...");

    // Wait for the task to complete
    handle.await.unwrap();
}
