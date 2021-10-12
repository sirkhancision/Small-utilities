use std::io;
use unicode_hfwidth::to_fullwidth;
use unidecode::unidecode;

fn main() {
    println!("Insira o texto a passar para caracteres fullwidth:");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("erro ao ler texto da entrada padrão");

    // convert the trimmed string to ascii-only,
    // so as to be perfectly convertable to fullwidth chars
    input = unidecode(&input);
    let input = input.trim();

    if !input.is_empty() {
        for char in input.chars() {
            if char.is_ascii_alphabetic() || char.is_ascii_digit() {
                if let Some(c) = to_fullwidth(char) {
                    print!("{}", c);
                }
            } else {
                print!("{}", char);
            }
        }
    }
    print!("\n");
}
