//Write a program that reads a string input and uses the match statement 
//to respond with different outputs based on the input 
//(e.g., "hello" => "Hi there!", "bye" => "Goodbye!", etc.).

use std::io;

fn main(){
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Invalid input");

    let text: &str = input.trim();

    match text {
        "hello" => println!("hiii!!!!"),
        "bye" => println!("bye-bye!!"),
        "deltarune" => println!("TOMORROW!"),
        _ => println!("idk what to say"),
    }


}
