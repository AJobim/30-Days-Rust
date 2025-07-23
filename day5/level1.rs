fn changes_fruit(word: &mut &str) {
   *word = "banana"; /* "*" is used for dereferencing, which *
                      * allow us to change the actual data.  */
}

fn main(){
    let sentence = String::from("little test");
    
    //BORROWING
    let borrows = &sentence; /* The usage of & makes a reference, *
                              * thus only borrowing the string:   */
    println!("{}", borrows.len());

    //MOVING              //the sentence's ownership has been moved
    let moves = sentence; /* if the value moves into a new owner,  *
                           * borrower loses it's string reference. */
    println!("moved! now moves has the sentence {moves}");
    
    //PASSING THROUGH FUNCTIONS
    let mut best_fruit = "apple";

    changes_fruit(&mut best_fruit);
    println!("{best_fruit}");
}
