trait Sound {
    fn animal_sound(&self);
}

struct Animal {
    name: String
}

impl Sound for Animal {
    fn animal_sound(&self) {
        match self.name.as_str() {
            "Cachorro" => println!("Woof Woof!"),
            "Gato" => println!("Wowwn!"),
            "Vaca" => println!("MUUUuuuu!"),
            _ => print!("{} faz um som desconhecido!", self.name)
        }
    }
}

fn main() {
    let dog = Animal { name: String::from("Cachorro") };
    let cat = Animal { name: String::from("Gato") };
    let vaca = Animal { name: String::from("Vaca") };

    let animal_vec = vec![dog, cat, vaca];

    for a in animal_vec {
        a.animal_sound();
    }
}
