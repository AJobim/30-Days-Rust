//1. Implement a program that tracks a list of tasks using a vector. 
//   Each task should have a description and a boolean indicating if it’s completed.
#[derive(Debug)]    
struct Task {
    description: String,
    finished: bool,
}

fn task_list() {
    let mut tasks: Vec <Task> = Vec::new();
    
    tasks.push(Task {
        description: String::from("water plants"),
        finished: false,
    });
    tasks.push(Task {
        description: String::from("wash the dishes"),
        finished: true,
    });
    
    println!("1. {:?}", tasks);
}

//2. Create a hash map to count the occurrences of words in a given sentence.
//   Print each word with its count.
use std::collections::HashMap;

fn word_ocurrence() {
    let mut wordboard: HashMap<&str, i32> = HashMap::new();
    let phrase: String = String::from("before was was was, was was is").replace(",", "");

    for word in phrase.trim().split_whitespace(){
        if !wordboard.contains_key(word){
            wordboard.insert(word, 1);

        } else {
            wordboard.entry(word).and_modify(|value| *value += 1);
        }
    }
    println!("2. {:?}", wordboard);
    
}

//3. Use a VecDeque to implement a queue for processing tasks. 
//   Demonstrate adding and removing tasks from the queue.
use std::collections::VecDeque;

fn queue() {
    let mut queue: VecDeque<String> = VecDeque::new();

    queue.push_back(String::from("Clean bedroom"));
    queue.push_back(String::from("Take the dog out"));
    queue.push_front(String::from("Wash clothes"));

    println!("3. Queue before removing from the back: {:?}", queue);
    queue.pop_back();

    println!("   Queue now: {:?}", queue);
}
//4.Write a program using a BTreeMap
use std::collections::BTreeMap;

fn family() {
    let mut family = BTreeMap::new();

    family.insert("Alexander", 86);
    family.insert("Alice", 58);
    family.insert("Andrew", 27);

    println!("4. Family tree!");

    for (name, age) in &family {
        println!("   {} is {} years old", name, age);
    }
}

fn main() {
    task_list();
    word_ocurrence();
    queue();
    family();
}   