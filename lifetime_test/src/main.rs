use std::fmt::Display;

struct ImportantExcerpt<'a> {
    part: &'a str,
}
fn main() {
    let str1 = "abec";
    let str2 = String::from("value");
    println!("the largest string is {}", largest(str1, str2.as_str()));

    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");

    let i = ImportantExcerpt{part: first_sentence};

}

fn largest<'a>(str1: &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

 

fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str
where
    T: Display, 
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
