
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,

}
fn main() {
    let user1 = User {
        active: true,
        username : String::from("youxuan"),
        email: String::from("1eesfsdf@mail.com"),
        sign_in_count: 1,
    };

    let user2 = User{
        active: false,
        ..user1
    };

    println!("the struct user1 is {}", user1.username);
    println!("the struct is {}, {}",user2.active, user2.username)
}
