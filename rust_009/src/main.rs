struct Animal {
    name: String,
    breed: String,
    gender: String,
    age: u64,
}

trait Interact {
    fn say_hi(&self) {}
    fn talk(&self) {}
}

impl Interact for Animal {
    fn say_hi(&self) {
        println!("Hi! I'm {}", self.name)
    }
    fn talk(&self) {
        println!("I cannot talk right now, I'm busy!")
    }
}

fn main() {
    let cat = Animal {
        name: String::from("Street"),
        breed: String::from("Orange"),
        gender: String::from("Male"),
        age: 8
    };

    cat.say_hi();
    cat.talk();
}