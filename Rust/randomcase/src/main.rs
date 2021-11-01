use rand::random;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args.iter().any(|h| h == "-h" || h == "--help") {
        println!(
            "Conversor de randomcase\n\
Randomiza a ordem de maiusculas e minusculas em um texto\n
Argumentos possíveis:\n\"-h\"/\"--help\" - Exibe esse texto de ajuda."
        );
    } else if args.len() > 1 {
        for char in args[1].chars() {
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
    } else {
        eprintln!("Argumentos insuficientes passados ao programa");
        process::exit(1);
    }
}
