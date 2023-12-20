// Topic: Flow control using if..else
//
// Program requirements:
// * Displays a message based on the value of a boolean variable
// * When the variable is set to true, display "hello"
// * When the variable is set to false, display "goodbye"
//
// Notes:
// * Use a variable set to either true or false
// * Use an if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

// Topic: Flow control using if..else if..else
//
// Program requirements:
// * Display ">5", "<5", or "=5" based on the value of a variable
//   is > 5, < 5, or == 5, respectively
//
// Notes:
// * Use a variable set to any integer value
// * Use an if..else if..else block to determine which message to display
// * Use the println macro to display messages to the terminal

fn main() {
    let is_hello = true;

    if is_hello {
        println!("hello");
    } else {
        println!("Goodbye");
    }

    let x = 4;
    if x > 5 {
        println!(">5");
    } else if x < 5 {
        println!("<5");
    } else {
        println!("=5");
    }
}
