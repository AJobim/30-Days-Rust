//Create a program to demonstrate the use of struct update syntax.

struct User {
    name: String,
    password: String,
    credits: i32,
    adult: bool,
}

fn main(){
    let user_model = User {
        name: String::from("MODEL"),
        password: String::from("12345"),
        credits: 0,
        adult: true,        
    };

    let _user1 = User {
        name: String::from("Feldspar"),
        password: String::from("harmonicaLover123"),

        //this will fill the remaining camps with data in userModel
        ..user_model //credits = 0, adult = 0
    };

}