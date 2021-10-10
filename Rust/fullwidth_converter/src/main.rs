use std::io;
use unicode_hfwidth::to_fullwidth;
use unidecode::unidecode;

fn main() {
    println!("Insira o texto a passar para caracteres fullwidth:");

    let mut input = String::new();
    if let Err(e) = io::stdin().read_line(&mut input) {
        println!("Erro: {}", e)
    }
    
    // convert the trimmed string to ascii-only,
    // so as to be perfectly convertable to fullwidth chars
    let input = unidecode(input.trim());

    for element in input.chars() {
        if element.is_ascii_alphabetic() || element.is_ascii_digit() {
            if let Some(c) = to_fullwidth(element) {
                print!("{}", c);
            }
        } else {
            print!("{}", element);
        }
    }
    print!("\n")
}
