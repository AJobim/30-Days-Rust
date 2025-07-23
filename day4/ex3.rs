//Find the nth Fibonnaci number

use std::io::{self, Write};

fn fibonacci(nth: u32) -> u32 {
    
    if nth <=1 {
        return nth; 
    }

    return fibonacci(nth - 1) + fibonacci(nth - 2);
}

fn main () {

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
    
    println!("O {}° número de fibonacci é: {}", number, fibonacci(number));

}
