// Traits in function parameters: We can write functions that accept any type implementing a Traits
// i.e you can use traits to accept any type that implements a trait in a function.

//1. Define a trait:  that requires a method summarize()
pub trait Summary {
    fn summarize(&self) -> String;
}
//2. Implement the Summary trait for a struct:
pub struct NewsArticle {
    pub headline: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {}", self.headline, self.author)
    }
}

// Use trait to accept any type that implements a trait in a function

/* fn notify(item: &impl Summary) {
 *      println!("Breaking News! {}", item.summarize());
 * }
 */

/* or using generics */
fn notify<T: Summary>(item: &T) {
    println!("Breaking news: {}", item.summarize());
}

fn main() {
    let article = NewsArticle {
        headline: String::from("Rust 2.0 release"),
        author: String::from("Daybreak"),
        content: String::from("Rust trait example using traits in function parameters "),
    };
    println!("Article Summary: {}", article.summarize());

    notify(&article);
}
