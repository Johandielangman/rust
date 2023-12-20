// Topic: Decision making with match
//
// Program requirements:
// * Display "it's true" or "it's false" based on the value of a variable
//
// Notes:
// * Use a variable set to either true or false
// * Use a match expression to determine which message to display

// Topic: Decision making with match
//
// Program requirements:
// * Display "one", "two", "three", or "other" based on whether
//   the value of a variable is 1, 2, 3, or some other number,
//   respectively
//
// Notes:
// * Use a variable set to any integer
// * Use a match expression to determine which message to display
// * Use an underscore (_) to match on any value

fn main() {
    let some_bool = false;

    match some_bool {
        true => println!("It's true"),
        false => println!("It's false")
    }

    let some_int: i32 = 2;

    match some_int {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("other")
    }

}
