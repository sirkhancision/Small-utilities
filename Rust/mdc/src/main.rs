use std::{env, mem::swap, process};

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

struct Mdc {
    first_var: u128,
    second_var: u128,
}

impl Mdc {
    fn new(input: &Vec<String>) -> Result<Self, &str> {
        let x: u128 = match input[1].as_str().parse() {
            Err(_e) => return Err("x não é inteiro"),
            Ok(n) => n,
        };
        let y: u128 = match input[2].as_str().parse() {
            Err(_e) => return Err("y não é inteiro"),
            Ok(n) => n,
        };

        Ok(Mdc {
            first_var: x,
            second_var: y,
        })
    }

    fn calculate(mut input: Self) -> Result<u128, &'static str> {
        if input.first_var == 0 || input.second_var == 0 {
            return Err("divisão por zero não é permitida");
        } else if input.second_var > input.first_var {
            swap(&mut input.first_var, &mut input.second_var);
        }

        // Euclidean algorithm
        while input.first_var % input.second_var != 0 {
            swap(&mut input.first_var, &mut input.second_var);
            input.second_var %= input.first_var;
        }

        Ok(input.second_var)
    }
}
