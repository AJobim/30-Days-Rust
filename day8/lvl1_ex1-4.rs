use std::collections::HashMap;

fn main() {
    //1. Create a vector of integers, push values into it, and print the vector.
    let mut values: Vec <i32> = Vec::new();

    values.push(23);
    values.push(62);
    values.push(413);
    println!("1. {:?}", values);   

    //2. Write a program to create a string and append characters to it, displaying the final string.
    let mut phrase: String = String::new();

    phrase.push_str("a");
    phrase.push_str("b");
    phrase.push_str("c");
    println!("2. {}", phrase);

    //3. Implement a hash map to store names and ages of three people. Print each person's name and age.
    let mut people = HashMap::new();

    people.insert(String::from("Alice"), 16);
    people.insert(String::from("Cheshire Cat"), 21);
    people.insert(String::from("Mad Hatter"), 39);
    
    println!("3. People: {:?}", people);

    //4. Create a vector of strings, representing fruit names, and sort them in alphabetical order.
    let mut fruits: Vec <String> = Vec::new();

    fruits.push(String::from("Banana"));
    fruits.push(String::from("Orange"));
    fruits.push(String::from("Mango"));
    fruits.push(String::from("Apple"));

    fruits.sort();
    println!("4. {:?}", fruits);
}