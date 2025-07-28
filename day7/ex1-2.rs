//1. Create an enum called Weather with variants: Sunny, Rainy, Cloudy, and Windy.

enum Weather {
    Sunny,
    Rainy,
    Cloudy,
    Windy,
}

fn main () {
    let current_weather = Weather::Rainy;

//2. Use a match statement to print out a message for each weather condition.
    match current_weather {
        Weather::Sunny  => println!("I'm feeling glad, I got sunshine in a bag"),
        Weather::Rainy  => println!("Now that it's raining more than ever, you can stand under my umbrella"),
        Weather::Cloudy => println!("Cloudy, but with a zero chance of meatballs"),
        Weather::Windy  => println!("Take a minute to listen to the wind of change"),
    }
}