/* DAY 8 CHALLENGE
Create a Rust program that manages a simple inventory system using collections. 
Implement the following functionalities:
    Store items in a vector.
    Use a hash map to track the quantities of each item.
    Allow adding, removing, and updating item quantities.
 */
use std::collections::HashMap;
use std::io;

fn add_item(items: &mut Vec<String>, quant: &mut HashMap<String, i32>) {
    let mut name = String::new();
    let mut qty = String::new();

    println!("Enter item name:");
    io::stdin().read_line(&mut name).expect("Failed to read name");
    let name: String = name.trim().to_string();

    println!("Enter item quantity:");
    io::stdin().read_line(&mut qty).expect("Failed to read quantity");
    let qty: i32 = qty.trim().parse().unwrap(); //Parses into i32
    
    //Creates new item if it didn't exist before
    if !items.contains(&name) {
        items.push(name.clone());
        quant.insert(name, qty);
    } 
    //Adds item to existing 
    else if let Some(qnt) = quant.get_mut(&name) {
        *qnt += qty;
    }

    for item in items {
        let quant = quant.get(item).unwrap();
        println!("{}: {}", item, quant);
    }

    println!("Added!");
}

fn show_items(items: &Vec <String>, quant: &HashMap<String, i32>) {
    println!("");
    println!("Your inventory:");

    for item in items {
        let quant = quant.get(item).unwrap();
        println!("{}: {}x", item, quant);
    }
}

fn use_item(items: &mut Vec <String>, quant: &mut HashMap<String, i32>) {
    let mut name = String::new();
    let mut qty = String::new();
    let mut specific_item = String::new();

    //Item input
    println!("Enter item name:");
    
    io::stdin().read_line(&mut name).expect("Failed to read name");
    let name: String = name.trim().to_string();

    if items.contains(&name) {
        specific_item = name;
        println!("Item found -> {}: {}x", &specific_item, quant.get(&specific_item).unwrap());
    } else {
        println!("No result for '{}'", name);
        return;
    }

    //Ammount input
    println!("How many would you like to remove?");

    io::stdin()
        .read_line(&mut qty)
        .expect("Failed to read name");
    let qty: i32 = qty.trim().parse().unwrap();
    
    //Removes X amount
    if let Some(ammount) = quant.get_mut(&specific_item){
        *ammount -= qty;
        println!("Removed {}x items from {}", ammount, specific_item);
        
        if *ammount <= 0 {
            items.retain(|item| item != &specific_item); //Removes the item from the vector
            println!("Zero items left, the item was removed!"); 
        }
    }

}

fn main() {
    let mut items: Vec<String> = Vec::new();
    let mut input: String = String::new();
    let mut quantities: HashMap<String, i32> = HashMap::new();

    println!("INVENTORY MANAGER");
    loop {
        println!("");
        println!("0 - End program");
        println!("1 - Add Items");
        println!("2 - Remove Items");
        println!("3 - Display Items");
        println!("Please, type your choice: ");

        io::stdin()
            .read_line(&mut input)
            .expect("Error reading input");

        let choice: i32 = input.trim().parse().unwrap();
        input.clear();

        match choice {
            0 => break,
            1 => add_item(&mut items, &mut quantities),
            2 => use_item(&mut items, &mut quantities),
            3 => show_items(&items, &quantities),
            _ => break,
        }
    }
    println!("finished!");
}