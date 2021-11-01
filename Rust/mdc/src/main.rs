use mdc::Mdc;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() == 2 && args.iter().any(|h| h == "-h" || h == "--help") {
        println!(
            "Calculadora de MDC (Máximo Divisor Comum)\n\
 Formato: x y (Exemplo: 10 5)\n
Argumentos possíveis: \"-h\"/\"--help\" - Exibe esse texto de ajuda."
        );
    } else if args.len() == 3 {
        let input = Mdc::new(&args).unwrap_or_else(|err| {
            eprintln!("Execução abortada: {}", err);
            process::exit(1);
        });

        println!(
            "MDC: {}",
            Mdc::calculate(input).unwrap_or_else(|err| {
                eprintln!("Execução abortada: {}", err);
                process::exit(1);
            })
        );
    } else {
        eprintln!("Argumentos insuficientes passados ao programa");
        process::exit(1);
    }
}
