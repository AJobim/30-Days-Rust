fn main() {
    let current_year = 2025;
    let my_age = 19;

    let mut my_height = 172;

    let my_name = String::from("Aggie");
    
    let is_student = true;
    let already_had_birthday = false;
    
    let birth_year = { 
        if already_had_birthday == true {
            current_year - my_age
        } else {
            current_year - (my_age + 1)
        }
    };

    my_height = 176;

    println!("My age: {}", my_age);
    println!("My name: {}", my_name);
    println!("Is a student: {}", is_student);
    println!("Birth Year: {}", birth_year);
    println!("My height: {}", my_height);
}
