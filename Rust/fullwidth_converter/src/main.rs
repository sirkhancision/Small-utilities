use std::{env, process};
use unicode_hfwidth::to_fullwidth;
use unidecode::unidecode;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        // convert the string to ascii-only,
        // so as to be perfectly convertable to fullwidth chars
        let text = unidecode(&args[1]);

        for char in text.chars() {
            if char.is_ascii_alphabetic() || char.is_ascii_digit() {
                if let Some(c) = to_fullwidth(char) {
                    print!("{}", c);
                }
            } else {
                print!("{}", char);
            }
        }
        print!("\n");
    } else {
        eprintln!("Argumentos insuficientes passados ao programa");
        process::exit(1);
    }
}
