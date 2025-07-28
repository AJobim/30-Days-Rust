fn counts(phrase: &str){
    let alphabet: [char, 26] = ["a","b","c","d","e","f","g","h","i","j","k","l","m","n","o","p","q","r","s","t","u","v","w","x","y","z"];
    let mut letter_counter = [26; 0];
    let mut index: u32 = 0;

    for char in phrase.chars() {
        index = 0;

        for letter in alphabet {
            if char == letter {
                letter_counter[index] += 1;               
            }
            println!("{letter} = {letter_counter[index}")
            index += 1;
        }
    }

}

fn main(){
    let phrase: &str = "bom dia";

    counts(&phrase);
}