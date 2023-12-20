// Topic: Strings
//
// Requirements:
// * Print out the name and favorite colors of people aged 10 and under
//
// Notes:
// * Use a struct for a persons age, name, and favorite color
// * The color and name should be stored as a String
// * Create and store at least 3 people in a vector
// * Iterate through the vector using a for..in loop
// * Use an if expression to determine which person's info should be printed
// * The name and colors should be printed using a function

struct Student {
    name: String,
    age: i32,
    fav_color: String,
    pronoun: String,
}

impl Student {
    fn print(&self) {
        println!("{:?} is {:?} years old and likes {:?} dragons", self.pronoun, self.age, self.fav_color);
    }
}

fn print_name(name: &str){
    println!("This is {:?}", name);
}

fn main() {
    let class = vec![
        Student {
            name: "Johan".to_owned(),
            age: 25,
            fav_color: "Red".to_owned(),
            pronoun: "He".to_owned(),
        },
        Student {
            name: "Annika".to_owned(),
            age: 23,
            fav_color: "Pink".to_owned(),
            pronoun: "She".to_owned(),
        },
        Student {
            name: "Bismarck".to_owned(),
            age: 9,
            fav_color: "Pink".to_owned(),
            pronoun: "He".to_owned(),
        }
    ];

    for student in class {
        if student.age > 10 {
            print_name(&student.name);
            student.print();
        }
    }
}
