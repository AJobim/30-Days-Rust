// Create a function that accepts a string and returns how many vowels are in it

fn vowels(sentence: &str){
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut count = 0;

    for letter in sentence.chars() {
        for vowel in vowels {
            if letter == vowel { 
                count += 1; 
            }
        }
    }
    println!("there are {} vowels in {}", count, sentence);
}

//input functions I created for easing handling io;
mod input;

fn main(){
   
    //user_input: creates string, asks for input and outputs String
    let sentence = input::user_input("write your sentence: ");

    vowels(sentence.trim());
}
