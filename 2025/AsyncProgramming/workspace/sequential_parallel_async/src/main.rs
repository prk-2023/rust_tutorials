use tokio::time::{sleep, Duration};

async fn read_data_async(path: &str) -> String {
    println!("Reading data from {}...", path);
    // Simulate async I/O work
    sleep(Duration::from_millis(1000)).await;
    println!("Finished reading {}", path);
    format!("Data from {}", path)
}

#[tokio::main]
async fn main() {
    //Serial ex
    let start_time = std::time::Instant::now();
    // Spawn each task independently.
    let handle_a = tokio::spawn(async { read_data_async("config_A").await });
    let handle_b = tokio::spawn(async { read_data_async("config_B").await });
    let handle_c = tokio::spawn(async { read_data_async("config_C").await });

    // Await all of them.
    let (a, b, c) = tokio::join!(handle_a, handle_b, handle_c,);

    println!("Result A: {:?}", a.unwrap());
    println!("Result B: {:?}", b.unwrap());
    println!("Result C: {:?}", c.unwrap());

    let duration = start_time.elapsed();
    println!("--- Total time elapsed serial ex mode: {:.2?}", duration);

    // parallel ex
    let start_time = std::time::Instant::now();

    // Task A
    let a = tokio::spawn(async { read_data_async("config_A").await })
        .await
        .unwrap();

    // Task B (waits for A)
    let b = tokio::spawn(async { read_data_async("config_B").await })
        .await
        .unwrap();

    // Task C (waits for B)
    let c = tokio::spawn(async { read_data_async("config_C").await })
        .await
        .unwrap();

    println!("Result A: {}", a);
    println!("Result B: {}", b);
    println!("Result C: {}", c);

    let duration = start_time.elapsed();
    println!("--- Total time elapsed parallel ex mode: {:.2?}", duration);
}
