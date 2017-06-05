#[derive(Debug)]
struct Rectangle {
    length: u32,
    width: u32,
}

impl Rectangle {
    fn square(size: u32) -> Rectangle {
        Rectangle { length: size, width: size }
    }

    fn area(&self) -> u32 {
        self.length * self.width
    }

    fn can_hold(&self, rect: &Rectangle) -> bool {
        self.length > rect.length && self.width > rect.width
    }
}

fn main() {
    let rect1 = Rectangle { length: 50, width: 30 };

    println!("Rect is {:#?}", rect1);
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect1 = Rectangle { length: 50, width: 30 };
    let rect2 = Rectangle { length: 40, width: 10 };
    let rect3 = Rectangle { length: 45, width: 60 };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));

    let rect = Rectangle::square(10);
    println!("A square rectangle: {} x {}", rect.width, rect.length);
}
