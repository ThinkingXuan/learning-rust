use std::collections::HashMap;
fn main() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);


    let team = String::from("Blue");

    let score = scores.get(&team).copied().unwrap_or(0);

    println!("{}", score);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }

    let text = "hello world wonderful world";

    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}

fn main() {
    let arr = [1, 2, 3, 2, 4, 1, 3, 5];
    let mut counts = HashMap::new();

    for &num in &arr {
        *counts.entry(num).or_insert(0) += 1;
    }

    for (num, count) in counts {
        println!("{} occurs {} times", num, count);
    }
}

pub fn single_number(nums: Vec<i32>) -> i32 {
    let mut mp: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        if let Some(&count) = mp.get(&nums[i]) {
            mp.insert(nums[i], count+1);
        }else {
             mp.insert(nums[i], 1);
        }
    }

    for (k, v) in mp {
        if v == 1 {
            return k;
        }
    }

    
    return -1;
}
