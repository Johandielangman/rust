// Topic: Basic arithmetic
//
// Program requirements:
// * Displays the result of the sum of two numbers
//
// Notes:
// * Use a function to display the result
// * Use the "{:?}" token in the println macro to display the result

// * Use a function to add two numbers together
fn add(a: i32, b: i32) -> i32 {
    let result = a + b;
    println!("The sum of {:?} and {:?} is {:?}.", a, b, result);
    return result;
}

fn display_result(result: i32) {
    println!("The result is {:?}.", result);
}

fn main() {
    let a = 5;
    let b = 10;
    let result = add(a, b);
    display_result(result);
}
