use std::io;

struct Person {
    name: String,
    age: String,
}

fn main(){
    let mut choice = String::new();
    let mut people: Vec<crate::Person> = Vec::new();
    while choice.trim() != "quit" {
        choice.clear();
        println!("Type 'add' to add Person to data");
        println!("Type 'show' to see all people");
        println!("Type 'delete' to delete all people");
        println!("Type 'age' to search all people of that age");
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line");

        if(choice.trim() == "show")
        {
            for Person { name, age } in &people{
                println!("Name:{} Age:{}", name, age); //prints each person in people
            }
        }
        if(choice.trim() == "delete")
        {
            people.clear(); //clears whole list
        }
        if(choice.trim() == "age"){
            let mut ageSearch = String::new();
            println!("What age would you like to search for?");
            io::stdin()
                .read_line(&mut ageSearch)
                .expect("Failed to read line");
            for Person { name, age } in &people{
                if(age == ageSearch.trim()){
                    println!("Name:{}", name);
                }
            }
        }
        if(choice.trim() == "add") // adds person to people after asking for name and age
        {
            let mut name = String::new();
            let mut age = String::new();
            println!("What is your name?");
            io::stdin()
                .read_line(&mut name)
                .expect("Failed to read line");
            println!("What is your age?");
            io::stdin()
                .read_line(&mut age)
                .expect("Failed to read line");
            let new = Person{
                name: String::from(name.trim()),
                age: String::from(age.trim()),
            };
            people.push(new);
        }
    }
}