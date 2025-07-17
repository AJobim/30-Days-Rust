//1. Create a program that calculates the factorial of a given number using a while loop

use std::io;

fn main(){
    
    let mut n = 2;
    let mut factorial = 1;
    let mut input = String::new();
    
    println!("Calculate the factorial of a number! Type the number:");

    io::stdin()
        .read_line(&mut input) //Reads input
        .expect("Failed to read input"); //Error message

    let number: i32 = input
        .trim()  //Deletes newline
        .parse() //Convert to given type (i32)
        .expect("Not an int."); //Error message

    while n <= number {
        factorial *= n;
        n += 1;
    }
    println!("The factorial of {number} is {factorial}");
}
