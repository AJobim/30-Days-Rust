//Create a program that defines and uses a struct for a book with 
//fields like title, author, pages, and publisher.

struct Books {
    title: String,
    author: String,
    pages: i32,
    saga: String,
}

fn main(){

    let book = Books {
        title: String::from("The Last Wish"),
        author: String::from("Andrzej Sapkowski"),
        pages: 288,
        saga: String::from("The Witcher"),
    };

    println!("I'm reading {} from {} saga, wrote by {}! It has {} pages.", book.title, book.saga, book.author, book.pages);
}