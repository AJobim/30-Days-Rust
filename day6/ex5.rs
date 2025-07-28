//Implement a method on a struct that compares two struct instances for equality.

#[derive(PartialEq)]
struct Model {
    name: String,
    currency: i32,
    is_active: bool,
}

impl Model {
    fn compares(obj1: &Model, obj2: &Model) -> bool {
        obj1 == obj2
    }
}

fn main(){

    let obj1 = Model {
        name: String::from("teste"),
        currency: 0,
        is_active: true,
    };

    let obj2 = Model {
        name: String::from("teste"),
        ..obj1
    };

    let result = Model::compares(&obj1, &obj2); 
    println!("São iguais? {}", result);
}

/*Também poderia ser feito com método de instância
impl Model {
    fn compares(&self, other: &Model) -> bool {
        self == other
    }
}

fn main() {
    [...]
    let result = obj1.compares(&obj2);
}
*/