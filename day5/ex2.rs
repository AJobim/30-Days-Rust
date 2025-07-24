//Implement a function to clone a vector without transferring ownership.
fn clones(arr: [u32; 3]) -> [u32; 3] {
    let arr2 = arr.clone();
    arr2
}

fn main(){
    let vector1: [u32; 3] = [0, 10, 20];
    let vector2 = clones(vector1);
    println!("{:?}, {:?}", vector1, vector2);
}

