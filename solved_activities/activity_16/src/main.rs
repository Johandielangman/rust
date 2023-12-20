// Topic: Option
//
// Requirements:
// * Print out the details of a student's locker assignment
// * Lockers use numbers and are optional for students
//
// Notes:
// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

// * Use a struct containing the student's name and locker assignment
// * The locker assignment should use an Option<i32>

/// A student with a name and locker assignment
struct Student {
    name: String,
    locker: Option<i32>,
}

/// Print out the details of a student's locker assignment
fn print_locker(student: &Student) {
    match student.locker {
        Some(locker) => println!("{} has locker {}", student.name, locker),
        None => println!("{} does not have a locker", student.name),
    }
}

fn main() {
    let student1 = Student {
        name: String::from("Johan"),
        locker: Some(1),
    };

    let student2 = Student {
        name: String::from("Annika"),
        locker: None,
    };

    print_locker(&student1);
    print_locker(&student2);

}
