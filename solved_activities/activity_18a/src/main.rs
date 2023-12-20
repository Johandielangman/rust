// Topic: Result
//
// Requirements:
// * Create an structure named `Adult` that represents a person aged 21 or older:
//   * The structure must contain the person's name and age
//   * Implement Debug print functionality using `derive`
// * Implement a `new` function for the `Adult` structure that returns a Result:
//   * The Ok variant should contain the initialized structure, but only
//     if the person is aged 21 or older
//   * The Err variant should contain a String (or &str) that explains why
//     the structure could not be created
// * Instantiate two `Adult` structures:
//   * One should be aged under 21
//   * One should be 21 or over
// * Use `match` to print out a message for each `Adult`:
//   * For the Ok variant, print any message you want
//   * For the Err variant, print out the error message

#[derive(Debug)]
struct Adult {
    age: u8,
    name: String,
}

impl Adult {
    fn new(age: u8, name: &str) -> Result<Self, &str> {
        if age >= 21 {
            Ok(Self {
                age,
                name: name.to_string(),
            })
        } else {
            Err("{Not an adult")
        }
    }
}

fn main() {
    let adult1 = Adult::new(20, "John");
    let adult2 = Adult::new(21, "Jane");

    match adult1 {
        Ok(adult) => println!("{:?} is an adult ({:?})", adult.name, adult.age),
        Err(err) => println!("{}", err),
    }

    match adult2 {
        Ok(adult) => println!("{:?}", adult),
        Err(err) => println!("{}", err),
    }
}
