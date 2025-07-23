fn main (){
    let total: i16 = 50;
    let mut sum: i16 = 0;

    for number in 1..total{
        if number % 2 != 0 {
            sum += number;
        }
    }

    println!("the sum of all even from 1 to {total} is {sum}");

}
