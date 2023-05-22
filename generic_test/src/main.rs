fn main() {
    let number_list = vec![10, 50, 2, 4, 2, 100, 4];
    let lastest_value = lastest(&number_list);
    println!("the number list lastest value: {}", lastest_value);

    let char_list = vec!['a', 'c', 'e', 'b', 'c'];
    let lastest_value = lastest(&char_list);
    println!("the char list lastest value: {}", lastest_value);
}

fn lastest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {

    let mut lastest = &list[0];

    for item in list {
        if item > lastest {
            lastest = item
        }
    }
    return lastest;
}

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl <T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

struct Point2<T,U> {
    x: T,
    y: U,
}

impl <T,U> Point2<T,U>{
    fn mixup <X, Y>(self, p: Point2<X, Y>) -> Point2<T, Y> {
        Point2{x: self.x, y: p.y}
    }
}

#[test]
fn PointTest() {
    let point = Point{x: 1, y: 2};
    let point = Point{x: 1.1, y: 2.1};
    println!("the point is {:?}", point);
}