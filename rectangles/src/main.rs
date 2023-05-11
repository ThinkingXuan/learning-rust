use std::fmt::Debug;

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 实现结构体方法
impl Rectangle {

    // 关联函数，一般用户结构体的初始化， 使用 Rectangle::square(11)调用
    fn square(size :u32) ->Self {
        Self {
            width: size,
            height : size,
        }
    }

    fn get_area(&self) ->u32 {
        return self.width * self.height;
    }
}

fn main() {
    let width = 20;
    let height = 20;

    println!(
        "The area of the rectangle is {} square pixels.",
        area(width, height)
    );

    let rectl = (30, 50);

    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rectl)
    );

    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("rectangle is {:?}", rect1);
    println!("rectangle is {:#?}", rect1);


    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect1)
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.get_area()
    );

    println!(
        "The area of the rectangle is {:?} square pixels.",
        Rectangle::square(10)
    );
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

