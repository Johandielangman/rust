// Topic: Functions
//
// Program requirements:
// * Displays your first and last name

// * Use a function to display your first name
fn display_fist_name(name: &str) {
    // * Use the println macro to display messages to the terminal
    println!("My first name is {name}");
}

// * Use a function to display your last name
fn display_last_name(last_name: &str) {
    // * Use the println macro to display messages to the terminal
    println!("My last name is {last_name}");
}

fn main() {
    let first_name = "Johan";
    let last_name = "Hanekom";

    // Run the functions
    display_fist_name(first_name);
    display_last_name(last_name);
}
