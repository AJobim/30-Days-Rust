//Define a struct for a rectangle and implement methods to calculate area and perimeter.

struct Rectangle {
    lenght: i32,
    width: i32,
}

impl Rectangle{
    fn area(&self) -> i32 {
        self.lenght * self.width
    }

    fn perimeter(&self) -> i32 {
        2 * self.lenght + 2 * self.width
    }
}

fn main(){
    let object = Rectangle {
        lenght: 5,
        width: 10,
    };

    println!("Object lenght = {}", object.lenght);
    println!("Object width = {}", object.width);
    println!("Object area = {}", object.area());
    println!("Object perimeter = {}", object.perimeter());

}