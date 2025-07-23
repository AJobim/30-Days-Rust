fn main(){
    let mut timer: i8 = 10;

    loop{
        if timer > 0 {
            println!("{timer}!");
        } else {
            break;
        }

        timer-= 1;
    }

    println!("Landoff!!");
}
