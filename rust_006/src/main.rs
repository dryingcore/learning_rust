fn main() {
    let x = String::from("String");
    println!("{}", x);
    show_x(&x);
    println!("{}", x)
}

fn show_x(x: &String) {
    println!("{}", x)
}