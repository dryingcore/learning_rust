fn main() {
    let some_product = Some("laptop");
    let mut product_vec = vec!["cellphone", "battery", "charger"];

    match some_product {
        Some(product) => product_vec.push(product),
        _ => {}
    }

    if let Some(product) = some_product {
        product_vec.push(product);
    }
}

// fn main() {
//     let resultado: Result<i32, &str> = Ok(42);
//
//     if let Ok(valor) = resultado {
//         println!("Sucesso! O valor é {}", valor);
//     }
// }
