//Create a recursive function to find the factorial of a number using borrowed values.

fn find_factorial(val: &mut u32) {
    let mut factorial = 1;
    let mut n = 2;

    while n <= *val {
        factorial *= n;
        n += 1;
    }
    
    *val = factorial;
}

fn main(){
    let mut value: u32 = 3;
    println!("The value is {value}");

    find_factorial(&mut value);

    println!("It's factorial is: {value}");

}