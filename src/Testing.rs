mod Project3;

use std::io;

struct Person {
    name: String,
    //age: u32,
}

fn main() {
    let mut input = String::new();
    let mut people: Vec<Person> = Vec::new();
    while input.trim() != "quit" {
        println!("Enter your name:");
        input.clear();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        let user = Person {
            name: String::from(input.trim()),
        };
        people.push(user);
    }
    for i in 0..people.len() {
        println!("{}", people[i].name);
    }

}

