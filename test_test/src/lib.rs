pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
        assert!(result==4);
    }

    #[test]
    #[should_panic]
    fn greate_than_100() {
        Guess::new(200);
    }


    #[test]
    fn it_works22() -> Result<(), String> {
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    // 测试私有函数
    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}


pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(v: i32) -> Guess {
        if v < 1 || v > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", v);

        }
        Guess{value:v}
    }
}

