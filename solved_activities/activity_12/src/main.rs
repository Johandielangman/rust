// Topic: Implementing functionality with the impl keyword
//
// Requirements:
// * Print the characteristics of a shipping box
// * Must include dimensions, weight, and color
//
// Notes:
// * Use a struct to encapsulate the box characteristics
// * Use an enum for the box color
// * Implement functionality on the box struct to create a new box
// * Implement functionality on the box struct to print the characteristics

enum BoxColor {
    Red,
    Blue,
    Green,
    Yellow,
    Orange,
    Purple,
    Black,
    White,
    Brown,
    Grey,
}

impl BoxColor{
    fn print(&self){
        match self {
            BoxColor::Red => println!("Color: Red"),
            BoxColor::Blue => println!("Color: Blue"),
            BoxColor::Green => println!("Color: Green"),
            BoxColor::Yellow => println!("Color: Yellow"),
            BoxColor::Orange => println!("Color: Orange"),
            BoxColor::Purple => println!("Color: Purple"),
            BoxColor::Black => println!("Color: Black"),
            BoxColor::White => println!("Color: White"),
            BoxColor::Brown => println!("Color: Brown"),
            BoxColor::Grey => println!("Color: Grey"),
        }
    }
}

struct Dimensions {
    len: f64,
    with: f64,
    height: f64,
}

impl Dimensions {
    fn print(&self) {
        println!("Dimensions: {}x{}x{}", self.len, self.with, self.height);
    }
}

struct Box {
    dimensions: Dimensions,
    weight: f64,
    color: BoxColor,
}

impl Box {
    fn new_box(weight: f64, color: BoxColor, dimensions: Dimensions) -> Self {
        Self {
            dimensions,
            weight,
            color,
        }
    }

    fn give_characteristics(&self) {
        self.dimensions.print();
        println!("Weight: {}", self.weight);
        self.color.print();
    }
}


fn main() {
    let small_box = Dimensions {
        len: 10.0,
        with: 10.0,
        height: 10.0,
    };

    let my_box = Box::new_box(
        10.0,
        BoxColor::Red,
        small_box,
    );
    my_box.give_characteristics();
}
