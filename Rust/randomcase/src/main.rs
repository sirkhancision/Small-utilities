use rand::random;
use std::io;

fn main() {
    println!("Escreva o texto para randomcase:");

    let mut sentence = String::new();
    io::stdin()
        .read_line(&mut sentence)
        .expect("erro ao ler texto da entrada padrão");
    let sentence = sentence.trim();

    if !sentence.is_empty() {
        for char in sentence.chars() {
            if char.is_alphabetic() && random() {
                if char.is_lowercase() {
                    print!("{}", char.to_uppercase());
                } else if char.is_uppercase() {
                    print!("{}", char.to_lowercase());
                }
            } else {
                print!("{}", char);
            }
        }
        print!("\n");
    }
}
