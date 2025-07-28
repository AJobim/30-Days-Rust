//Write a program that uses a tuple struct to store RGB color values.

struct Rgb(u8, u8, u8);

fn main() {
    let black = Rgb(0, 0, 0);
    let white = Rgb(255, 255, 255);

    println!("Black: {}, {}, {}", black.0, black.1, black.2);
    println!("White: {}, {}, {}", white.0, white.1, white.2);
}