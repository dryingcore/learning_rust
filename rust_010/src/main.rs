use std::collections::HashMap;

#[derive(Debug)]
struct Person {
    name: String,
    age: i32
}

fn persons_by_name(persons: Vec<Person>) -> HashMap<String, Person> {
    persons.into_iter().map(|p| (p.name.clone(), p)).collect()
}

fn main() {
    let person_one = Person {
        name: "One".to_string(),
        age: 1
    };

    let person_two = Person {
        name: "Two".to_string(),
        age: 2
    };

    let person_three = Person {
        name: "Three".to_string(),
        age: 3
    };

    let persons = vec![person_one, person_two, person_three];
    let persons_hash = persons_by_name(persons);

    for (name, details) in &persons_hash {
        println!("Person {} has the details of {:?}", name, details)
    }
}
