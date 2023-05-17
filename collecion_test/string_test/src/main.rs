fn main() {
    let mut s = String::new();
    println!("this is {}", s);

    let data = "initial contents";

    let mut s = data.to_string();

    let mut s = String::from("foo");
    s.push_str("bar");

    println!("s is {}", s);

    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{s1}-{s2}-{s3}");
    println!("s is {}", s);

    for c in "Зд".chars() {
        println!("{c}")
    }

    

}
