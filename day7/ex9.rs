//9. Define a program that mimics an online order with enum OrderStatus 
//   containing Pending, Shipped, and Delivered. Write a function that takes 
//   OrderStatus as input and prints out the current order status.
enum OrderStatus {
    Pending,
    Shipped,
    Delivered,
}

fn status (order: OrderStatus) {
    match order {
        OrderStatus::Pending   => println!("Your order is pending."),
        OrderStatus::Shipped   => println!("Your order went out for shipping, expect it in a day or two."),
        OrderStatus::Delivered => println!("Your order has been delivered, thank you for your patience"),
    }
}

fn main () {
    let my_order = OrderStatus::Shipped;
    status(my_order);
}