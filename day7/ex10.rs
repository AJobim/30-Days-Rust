//10. Create an enum called Shape with variants Circle(f64) for the radius, Rectangle(f64, f64) 
//for width and height, and Triangle(f64, f64, f64) for the lengths of the three sides.
enum Shape {
    Circle(f64),
    Rectangle(f64, f64), 
    Triangle(f64, f64, f64),
}

/*11. Implement a function calculate_area that takes a Shape and returns the area of the shape:
        For a circle, use the formula πr².
        For a rectangle, use width * height.
        For a triangle, use Heron's formula.*/

impl Shape {
    fn calculate_area(&self) -> f64{
        match self {
            Shape::Circle(radius) => 3.14 * radius.powf(2.0),
            Shape::Rectangle(width, height) => width * height,
            Shape::Triangle(l1, l2, l3) => {
                let p = (l1 + l2 + l3)/2.0;
                (p*(p - l1)*(p - l2)*(p - l3)).sqrt()
            },
        }
    }
}

fn main (){
    let obj = Shape::Triangle(3.0, 4.0, 5.0);
    let area = obj.calculate_area();

    println!("the area is {}", area);
}