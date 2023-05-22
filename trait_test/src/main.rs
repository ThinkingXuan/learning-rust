use std::fmt::Display;

use trait_test::{Summary, Tweet, NewsArticle};

pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
pub fn notify2<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

pub fn notify3(item: &(impl Summary + Display)) {
    println!("Breaking news! {}", item);
}

pub fn notify4<T: Summary + Display>(item: &T) {
    println!("Breaking news! {}", item);

}

fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn returns_summarizable2(switch: bool) -> impl Summary {
    
    NewsArticle {
        headline: String::from(
            "Penguins win the Stanley Cup Championship!",
        ),
        location: String::from("Pittsburgh, PA, USA"),
        author: String::from("Iceburgh"),
        content: String::from(
            "The Pittsburgh Penguins once again are the best \
                hockey team in the NHL.",
        ),
    }
   
}


fn main() {
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };
    println!("1 new tweet: {}", tweet.summarize());

    notify(&tweet);
    notify2(&tweet);
    
    notify3(&tweet);
    notify4(&tweet);


}
