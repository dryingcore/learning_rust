use std::io;

enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

fn main() {
    loop {
        println!("Simple CLI Calculator");
        println!("----------------------");
        println!("1 - Adição");
        println!("2 - Subtração");
        println!("3 - Multiplicação");
        println!("4 - Divisão");
        println!("5 - Sair");
        println!("Selecione uma opção: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Erro ao ler entrada");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Entrada inválida! Tente novamente.");
                continue;
            }
        };

        if choice == 5 {
            println!("Saindo...");
            break;
        }

        println!("Digite o primeiro número: ");
        let a = read_number();
        println!("Digite o segundo número: ");
        let b = read_number();

        let result = match choice {
            1 => add_num(a, b),
            2 => sub_num(a, b),
            3 => mul_num(a, b),
            4 => {
                if b == 0 {
                    println!("Erro: divisão por zero!");
                    continue;
                }
                div_num(a, b)
            }
            _ => {
                println!("Opção inválida!");
                continue;
            }
        };

        println!("Resultado: {}", result);
    }
}

fn read_number() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Erro ao ler número");
    input.trim().parse().expect("Entrada inválida")
}

fn add_num(a: i32, b: i32) -> i32 {
    a + b
}

fn sub_num(a: i32, b: i32) -> i32 {
    a - b
}

fn div_num(a: i32, b: i32) -> i32 {
    a / b
}

fn mul_num(a: i32, b: i32) -> i32 {
    a * b
}
