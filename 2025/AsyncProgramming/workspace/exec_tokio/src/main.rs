use tokio;

async fn adder(x: i32) -> i32 {
    x + 1
}

#[tokio::main]
async fn main() {
    let mut x = 0;
    while x < 10 {
        x = adder(x).await;
    }
    println!("{x}");

    let response = reqwest::get("https://www.rust-lang.org").await.unwrap();
    println!("Response: {} ", response.status());
}
