//8. Create an enum Status with variants: Active, Inactive, and Suspended. 
//Use the if let control flow to handle the enum.

#[derive(Debug)]
enum Status {
    Active,
    Inactive,
    Suspended,
}

fn main(){
    if let aux = Status::Active {
        println!("{:?}", aux);
    }
}

