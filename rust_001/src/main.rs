use rust_001::Student;
use std::env;

fn main() {

    // let new_student = Student {
    //     name: "Gabriel".to_string(),
    //     age: 17
    // };

    let args: Vec<String> = env::args().collect();
    let new_student = Student::new(args[1].to_string()).unwrap_or_default();
    println!("{:?}", new_student)
}
