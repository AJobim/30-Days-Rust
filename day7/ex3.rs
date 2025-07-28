//Define an enum Device that can hold values Laptop, Tablet, and Phone, each containing a string (e.g., model name). 
//Write a method to display the device information.

enum Device {
    Laptop(String),
    Tablet(String),
    Phone(String),
}

impl Device {
    fn display_info(&self) {
        match self {
            Device::Laptop(text) => println!("Laptop details: {}", text),
            Device::Tablet(text) => println!("Tablet details: {}", text),
            Device::Phone(text) => println!("Phone details: {}", text),
        }
    }
}
fn main(){
    let my_device = Device::Phone(String::from("Iphone 11"));
    
    my_device.display_info();
}