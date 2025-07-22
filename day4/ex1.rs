//Celsius to fahrenheit calculator

fn c_to_f(c: f32) -> f32 {
    (c * 1.8) + 32.0
}

fn f_to_c (f: f32) -> f32 {
    (f / 1.8) - 32.0
}

fn main (){

    let celsius = 0.0;
    let fahrenheit = 75.0;
   
    println!("a temp é {}", c_to_f(celsius));
    println!("a temp é {}", f_to_c(fahrenheit));

}
