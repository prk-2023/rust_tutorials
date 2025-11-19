// extern crate trpl;
use trpl::Html;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let url = &args[1];
        match get_page_title(url).await {
            Some(title) => println!("The tile of the url: {url} is {title}"),
            None => println!("The url {url} has no title"),
        }
    })
}

async fn get_page_title(url: &str) -> Option<String> {
    let response_text = trpl::get(url).await.text().await;
    Html::parse(&response_text)
        .select_first("title")
        .map(|title| title.inner_html())
}
