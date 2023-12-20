// Topic: Working with an enum
//
// Program requirements:
// * Prints the name of a color to the terminal
//
// Notes:
// * Use an enum with color names as variants
// * Use a function to print the color name
// * The function must use the enum as a parameter
// * Use a match expression to determine which color
//   name to print

enum ColorNames {
    Red,
    Orange,
    Yellow,
    Green,
    Blue,
    Indigo,
    Violet,
}

fn print_color_name(color: ColorNames) {
    match color {
        ColorNames::Red => println!("Red"),
        ColorNames::Orange => println!("Orange"),
        ColorNames::Yellow => println!("Yellow"),
        ColorNames::Green => println!("Green"),
        ColorNames::Blue => println!("Blue"),
        ColorNames::Indigo => println!("Indigo"),
        ColorNames::Violet => println!("Violet"),
    }
}


fn main() {
    let color = ColorNames::Red;
    print_color_name(color);
}
