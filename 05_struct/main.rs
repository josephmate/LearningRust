
#[derive(Debug)]
struct Square {
    dimension: u32,
}

impl Square {
    fn area(&self) -> u32 {
        self.dimension * self.dimension
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

fn main() {
    let rect = Rectangle { width: 30, height: 50 };
    let square = Square { dimension: 30 };

    println!(
        "The area of the {:?} is {} square pixels.",
        rect,
        rect.area()
    );
    println!(
        "The area of the {:?} is {} square pixels.",
        square,
        square.area()
    );
}
