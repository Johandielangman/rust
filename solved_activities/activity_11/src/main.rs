// Topic: Ownership
//
// Requirements:
// * Print out the quantity and id number of a grocery item
//
// Notes:
// * Use a struct for the grocery item
// * Use two i32 fields for the quantity and id number
// * Create a function to display the quantity, with the struct as a parameter
// * Create a function to display the id number, with the struct as a parameter

struct GroceryItem{
    qty: i32,
    id: i32
}

fn display_qty(item: &GroceryItem){
    println!("We have {:?} items", item.qty);
}

fn display_id(item: &GroceryItem){
    println!("For grocery item with id {:?}", item.id);
}

fn main() {
    let apples = GroceryItem{
        qty: 10,
        id: 1
    };

    display_id(&apples);
    display_qty(&apples);

}
