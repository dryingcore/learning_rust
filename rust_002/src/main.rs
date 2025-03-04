const VOGAIS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];

fn main() {
    let palavra = "exemplo";
    let quantidade = how_many_vogals(palavra);
    println!("A palavra '{}' tem {} vogais.", palavra, quantidade);

    let is_a_palindromo = is_palindromo("carros");
    if is_a_palindromo {
        println!("Is a palindromo")
    }

    else {
        println!("Isn't a palindromo")
    }
}

fn how_many_vogals(word: &str) -> usize {
    word.chars().filter(|c| VOGAIS.contains(c)).count()
}

fn is_palindromo(word: &str) -> bool {
    word.chars().eq(word.chars().rev())
}