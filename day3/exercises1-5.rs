use std::io;

fn main() {
    
//1. Check if number is odd or even
    let number: i8 = 7;

    if number % 2 == 0 {
        println!("The number {number} is even");
    } else {
        println!("The number {number} is odd");
    }

//2. Prints number from 1 to 10
    let mut loop_number: i8 = 0;

    while loop_number <= 10 {
        println!("the number is {}", loop_number);
        loop_number += 1;
    }

//3. Iterate over an array of fav colors and print using for loop
    let fav_colors: [&str; 3] = ["purple", "lavender", "red"];

    for color in fav_colors {
        println!("I love {color}!")
    }

//4. Create a simple calculator using match to perform 
//   add, sub, mult and div based on user inputs
     
    let number1 = 10;
    let number2 =  3;
    
                      //String type
    let mut operation = String::new();
    
    println!("==================Simple calculator ===================");
    println!("Choose operation (add | sub | mult | div):");
    
    io::stdin().read_line(&mut operation);

    //Shadowing of  String to &str and remove newline
    let operation = operation.trim();

    //Match works in &str
    match operation {
        "add"  => println!("{number1} + {number2} = {}", (number1 + number2)),
        "sub"  => println!("{number1} - {number2} = {}", (number1 - number2)),
        "mult" => println!("{number1} * {number2} = {}", (number1 * number2)),
        "div"  => println!("{number1} / {number2} = {}", (number1 as f64 / number2 as f64)),
            _  => println!("Error, not a valid operation"),
    }
//5. Continuously takes user input until "exit" is typed using a loop

    let mut command = String::new();
    
    
    loop{
        io::stdin().read_line(&mut command);
        
        let command = command.trim();

        if command == "exit"{
            break;
        }
    }

    println!("The loop was broken!");
    

}
