// Topic: Working with expressions
//
// Requirements:
// * Print "its big" if a variable is > 100
// * Print "its small" if a variable is <= 100
//
// Notes:
// * Use a boolean variable set to the result of
//   an if..else expression to store whether the value
//   is > 100 or <= 100
// * Use a function to print the messages
// * Use a match expression to determine which message
//   to print

fn evaluate_is_big (big: bool) {
    if big {
        println!("It's big");
    } else {
        println!("It's small");
    }
}

fn main() {
    let my_val = 100;
    let is_big = my_val > 100;
    evaluate_is_big(is_big);
}
