#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rectangle = Rectangle {
        width: 60,
        height: 30,
    };

    let area = rectangle.area();

    println!("Area: {area}");

    println!("Rectangle: {rectangle:#?}");

    let rectangle2 = Rectangle {
        width: 30,
        height: 10,
    };

    let rectangle3 = Rectangle {
        width: 80,
        height: 10,
    };

    println!("{}", rectangle.can_hold(&rectangle2));
    println!("{}", rectangle.can_hold(&rectangle3));

    println!("square: {:?}", Rectangle::square(40));
}
