use std::collections::HashMap;

const MAX_POINTS: u32 = 100_000;


fn main() {
    let mut x = 5;
    println!("The value of x is :{}", x);
    x = 10;
    println!("The value of x is :{}", x);
    println!("The value of Max is:{}", MAX_POINTS);

    println!("the division is:{}", 1.0 / 2.0);

    let tup = (1, 2.1, 5);
    println!("the tup t1 = {}, t2= {}, t3 = {}", tup.0, tup.1, tup.2);
    let (t1, t2, t3) = tup;
    println!("the tup t1 = {},t2 = {}, t3 = {}", t1, t2, t3);

    let y = {
        x = 10;
        x + 1
    };
    let number = if y > 10 { 1 } else { 0 };
    println!("The Value is {}", number);

    let arr = [0, 1, 2, 324, 345, 345];
    println!("{:?}", arr);
    for i in 0..arr.len() {
        println!("{}", i)
    }

    for i in (0..arr.len() - 1).rev() {
        println!("{}", i)
    }

    let mut i = 0;
    let x = loop {
        i = i+1;
        if i == 10 {
            break i*20
        }
    };
    println!("The value i is {x}");

    for ele in (1..5).rev() {
        println!("The element is {ele}");
    }
    let s2 = String::from("hello word!");
    let s3 = s2;

    // println!("the s is {}", s2)

    let mut map = HashMap::new();
    
    map.insert(123, false);
    map.insert(456, true);

    if let Some(T) = map.get(&123) {

    }
    println!("{}", map.get(&123).unwrap());
    

}

use std::collections::HashSet;
pub fn contains_duplicate(nums: Vec<i32>) -> bool {

    let mut set: HashSet<i32> = nums.to_owned().into_iter().collect();
    return set.len() != nums.len();
}

pub fn contains_duplicate2(nums: Vec<i32>) -> bool {
    let mut set = HashSet::new();
    for v in nums {
        if !set.insert(v) {
            return true;
        }
    }
    false
}