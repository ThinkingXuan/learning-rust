use std::collections::HashMap;
use std::io::{ErrorKind, self, Read};
use std::fs::File;


fn main() {
    // panic!("crush and burn");
    let v = vec![1, 2, 3];
    // v[99];
    let greeting_file_result = File::open("hello.rs");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.rs") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the File:{:?}", e),
            },

            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        
        },
    };

    let greeting_file = File::open("hello.rs").unwrap_or_else(|error|{
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.rs").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });


    // let greeting_file = File::open("hello.txt").unwrap();
    // let greeting_file = File::open("hello.txt")
    //     .expect("hello.txt should be included in this project");

    test1();

}

fn test1() {
}

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file2() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    return Ok(username)
}

fn read_username_from_file3() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    return Ok(username);
}


// 统计最后str的第一行中的最后一个字符串
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()

}
pub struct Solution{}
impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        for _ in 0.. k as usize % nums.len() {
            let n = nums.pop().unwrap();
            nums.insert(0, n);
        }
    }
}