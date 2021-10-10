use rand;
use std::io;

fn main() {
    println!("Escreva o texto para randomcase:");

    let mut sentence = String::new();
    if let Err(e) = io::stdin().read_line(&mut sentence) {
        println!("Erro: {}", e)
    }

    if ! sentence.trim().is_empty() {
        for element in sentence.chars() {
            if element.is_alphabetic() && rand::random() {
                if element.is_lowercase() {
                    print!("{}", element.to_uppercase());
                } else if element.is_uppercase() {
                    print!("{}", element.to_lowercase());
                }
            } else {
                print!("{}", element);
            }
        }
    } else {
        println!("String vazia na entrada, saindo.")
    }
}
