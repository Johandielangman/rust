// Topic: Advanced match
//
// Requirements:
// * Print out a list of tickets and their information for an event
// * Tickets can be Backstage, Vip, and Standard
// * Backstage and Vip tickets include the ticket holder's name
// * All tickets include the price
//
// Notes:
// * Use an enum for the tickets with data associated with each variant
// * Create one of each ticket and place into a vector
// * Use a match expression while iterating the vector to print the ticket info

enum Tickets {
    Backstage(f64, String ),
    Standard(f64),
    Vip(f64, String),
}

fn main() {
    let tickets = vec![
        Tickets::Backstage(64.0, "Johan".to_owned()),
        Tickets::Standard(50.0),
        Tickets::Vip(80.0, "Annika".to_owned()),
    ];

    for ticket in tickets {
        match ticket {
            Tickets::Backstage(amount, name) => {
                println!("Backstage ticket for {} is {}", name, amount);
            },
            Tickets::Standard(amount) => {
                println!("Standard ticket is {}", amount);
            },
            Tickets::Vip(amount, name) => {
                println!("Vip ticket for {} is {}", name, amount);
            },
        }
    }

}
