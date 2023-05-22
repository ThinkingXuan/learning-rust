use std::fmt::Display;

pub trait Summary {
    // fn summarize(&self) -> String;

    fn summarize_auther(&self) -> String;
    // 默认实现的方法
    fn summarize(&self) -> String {
        format!("Read more...from {}", self.summarize_auther())
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle  {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    fn summarize_auther(&self) -> String {
        return String::from("value")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

// impl Summary for Tweet {
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// }

impl Summary for Tweet {
    fn summarize_auther(&self) -> String {
        format!("@{}", self.username)
    }
}

impl Display for Tweet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       println!("impl display");
       todo!()
    }
}