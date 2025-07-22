//Give a value and print its mult table

use std::io::{self, Write};

fn table(number: u32) {
    for val in 1..11 {
        println!("{number} * {val} = {}", (number * val));
    }
}

fn main(){
    let mut input = String::new();
    
    print!("Choose a number: ");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Error: Not a valid input value");

    let number = input
                .trim()
                .parse::<u32>()
                .unwrap();
    
    table(number);
}
