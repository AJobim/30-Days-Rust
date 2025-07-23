//Create a small game where the program generates a random number 
//between 1 and 10 and the user has to guess it. 
//Use a loop to keep asking until the user gets it right.

use std::io;
use std::io::Write;

fn main(){
    let rng: i8 = rand::random_range(1..10);

    loop{
        let mut input = String::new();

        print!("input: ");

        io::stdout().flush().expect("Failed to flush");

        io::stdin()
            .read_line(&mut input)
            .expect("Invalid input");

        let choice: i8 = input
                        .trim()
                        .parse()
                        .expect("CAN'T PUT TS INSIDE AN INT");
        
        if choice != rng {
            println!("Wrong!! Try again:");
        } else {
            println!("You did it! The number was {rng}");
            break;
        }
    }


}
