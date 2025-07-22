// Arquivo de métodos comumente reutilizados
// em códigos durante o desafio de 30 dias de rust.
use std::io::{self, Write};

//User input without text
pub fn user_input_textless() -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");
    
    input
}

//User input with print text
pub fn user_input(frase: &str) -> String {
    let mut input = String::new();
        
    print!("{}", frase);
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut input)
        .expect("Error reading input");

    input.trim()
}

pub fn to_int(input: String) -> u32 {
    let output: u32 = input
                    .trim()
                    .parse()
                    .unwrap();
    output
}
