//Create a Rust program that defines an enum representing a payment method: CreditCard, DebitCard, Cash, and PayPal. 
//Write a function to print the payment method used.

enum PaymentMethod {
    CreditCard(String),
    DebitCard(String),
    Cash,
    PayPal,
    Pix,
}

fn payed_with(which: PaymentMethod){
    match which {
        PaymentMethod::CreditCard(number)   => println!("Payed with credit card! The number is {}", number),
        PaymentMethod::DebitCard(number)    => println!("Payed with debit card! The number is {}", number),
        PaymentMethod::Cash                 => println!("Payed with cash! $$"),
        PaymentMethod::PayPal               => println!("Payed with PayPal!"),
        PaymentMethod::Pix                  => println!("Payed with Pix!"),
    }
}
fn main() {
    let payment = PaymentMethod::CreditCard(String::from("1154-5235-7437-1892"));
    let second_payment = PaymentMethod::Pix;
    
    payed_with(payment);
    payed_with(second_payment);
}