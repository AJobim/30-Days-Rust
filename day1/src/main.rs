fn main() {
    let word: &str = "value";
    let number: i8 = 21;
    let boolean: bool;

    let mut x = 5;
    println!("X was {}!", x);

    x = 10;
    println!("X equals {} now, not 5!", x);

   boolean = number > x;

   if boolean {
       println!("the {} {} is bigger than {}!", word, number, x);
    } 
   else {
       println!("it's not :<");
    } 
}
