//Write a function that returns an instance of a unit-like struct.

struct Marker;

fn marks() -> Marker {
    Marker
}

fn main() {
    let valor = marks();
    println!("The value was marked")
}