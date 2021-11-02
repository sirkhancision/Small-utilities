use fraction_simplifier::Fraction;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 && args.iter().any(|h| h == "-h" || h == "--help") {
        println!(
            "Simplificador de frações\n\
Formato de uso: x/y (Exemplo: 10/5)\n
Argumentos possíveis: \"-h\"/\"--help\" - Exibe esse texto de ajuda."
        );
    } else if args.len() == 2 {
        let frac = Fraction::new(&args[1])
            .unwrap_or_else(|err| {
                eprintln!("Execução abortada: {}", err);
                process::exit(1);
            })
            .simplify();

        Fraction::print_fraction(frac);
    } else {
        eprintln!("Argumentos insuficientes passados ao programa");
        process::exit(1);
    }
}
