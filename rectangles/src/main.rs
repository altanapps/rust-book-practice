struct Rectangle {
    width: u32,
    height: u32,
}

// This is the method
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn get_width(&self) -> u32 {
        self.width
    }

    fn get_height(&self) -> u32 {
        self.height
    }
}

fn main() {
    // This is the Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // This is called automatic dereferencing and referencing
    println!(
        "The width of the rectangle is {} pixels.",
        rect1.get_width()
    );

    println!(
        "The height of the rectangle is {} pixels.",
        rect1.get_height()
    );

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );
}

// fn area(rectangle: &Rectangle) -> u32 {
//     rectangle.width * rectangle.height
// }

// fn area(dimension: (u32, u32)) -> u32 {
//     dimension.0 * dimension.1
// }

// fn area(width: u32, height: u32) -> u32 {
//     width * height
// }
