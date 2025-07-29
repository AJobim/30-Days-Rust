//4. Create an enum called TrafficLight with variants Red, Yellow, and Green.
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

/*5. Implement a function get_light_duration that returns the duration for each light:
    Red: 30 seconds
    Yellow: 5 seconds
    Green: 20 seconds
*/
impl TrafficLight {
    fn get_light_duration(&self) -> i32 {
        match self {
            TrafficLight::Red    => 30,
            TrafficLight::Yellow => 5,
            TrafficLight::Green  => 20,
        }
    }
}

fn main(){
    let light = TrafficLight::Red;
    let light_duration = light.get_light_duration();

    //6. Use a match statement to print the duration of each traffic light.
    match light {
        TrafficLight::Red    => println!("Red light for {}s", light_duration),
        TrafficLight::Yellow => println!("Yellow light for {}", light_duration),
        TrafficLight::Green  => println!("Green light for {}", light_duration),
    }
    
}