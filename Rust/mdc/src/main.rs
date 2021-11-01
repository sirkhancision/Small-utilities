use std::{io, mem::swap, process};

fn main() {
    let mut input = String::new();

    println!("Digite dois números para calcular o MDC:");

    if let Err(e) = io::stdin().read_line(&mut input) {
        eprintln!(
            "Execução abortada: erro ao ler texto da entrada padrão\n{}",
            e
        );
        process::exit(1);
    }
    let input = input.trim();

    if !input.is_empty() {
        println!(
            "MDC: {}",
            mdc(slice_input(input).unwrap_or_else(|err| {
                eprintln!("Execução abortada: {}", err);
                process::exit(1);
            }))
            .unwrap_or_else(|err| {
                eprintln!("Execução abortada: {}", err);
                process::exit(1);
            })
        );
    }
}

fn mdc(mut input: (u128, u128)) -> Result<u128, &'static str> {
    if input.0 == 0 || input.1 == 0 {
        return Err("divisão por zero não é permitida");
    } else if input.1 > input.0 {
        swap(&mut input.0, &mut input.1);
    }

    // Euclidean algorithm
    while input.0 % input.1 != 0 {
        swap(&mut input.0, &mut input.1);
        input.1 %= input.0;
    }

    Ok(input.1)
}

// Slices the input string and outputs a u128 tuple, with
// the two numbers for calculating the GDC
fn slice_input(input_string: &str) -> Result<(u128, u128), &str> {
    for (i, char) in input_string.chars().enumerate() {
        if char.is_whitespace() {
            let x = input_string[..i].parse().expect("x não é inteiro");
            let y = input_string[i + 1..].parse().expect("y não é inteiro");

            return Ok((x, y));
        }
    }

    Err("não há espaço separando o primeiro número do segundo")
}
