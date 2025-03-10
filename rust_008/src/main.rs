fn main() {
    let mut s1: String = String::from("Hello");
    let s2: &str = "World";

    let mut s3 = filter(&mut s1);
    println!("{}", s3);
    x(&mut s1);

    s3 = filter(&mut s1);
    println!("{}", s3);
}

fn filter(s: &mut String) -> String {
    return s.chars().filter(|c| !"aeiouAEIOU".contains(*c)).collect();
}

fn x(y: &mut String) {
    y.push_str(", World!");
    println!("{}", y)
}

