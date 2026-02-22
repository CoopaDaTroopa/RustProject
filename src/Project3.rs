use std::io;

struct Person {
    name: String,
    age: String,
}

fn main(){
    let mut input = String::new();
    let mut people: Vec<crate::Person> = Vec::new();
    while input.trim() != "quit" {
        input.clear();
        println!("Type 'add' to add Person to data");
        println!("Type 'show' to see all people");
        println!("Type 'delete' to delete all people");
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        if(input.trim() == "show"){

        }
        if(input.trim() == "delete"){

        }
        if(input.trim() == "add"){
            input.clear();
            let mut name = "";
            let mut age = "";
            println!("What is your name?");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            name = input.trim();
            input.clear();
            println!("What is your age?");
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read line");
            age = input.trim();
            let new = Person{
                name: String::from(name),
                age: String::from(age),
            };
            people.push(new);
        }
    }
}
