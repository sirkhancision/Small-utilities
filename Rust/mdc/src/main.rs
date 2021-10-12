use std::{io, mem::swap};

fn main() {
    let mut input = String::new();

    println!("Digite dois números para calcular o MDC:");

    io::stdin()
        .read_line(&mut input)
        .expect("erro ao ler texto da entrada padrão");
    let input = input.trim();

    if !input.is_empty() {
        println!("MDC: {}", mdc(slice_input(input)));
    }
}

fn mdc(mut input: (u128, u128)) -> u128 {
    if input.1 > input.0 {
        swap(&mut input.0, &mut input.1);
    } else if input.1 == 0 && input.0 == 0 {
        panic!("entrada inválida");
    }

    // Euclidean algorithm
    while input.0 % input.1 != 0 {
        swap(&mut input.0, &mut input.1);
        input.1 %= input.0;
    }

    input.1
}

// Slices the input string and outputs a u128 tuple, with
// the two numbers for calculating the GDC
fn slice_input(input_string: &str) -> (u128, u128) {
    for (i, char) in input_string.chars().enumerate() {
        if char.is_whitespace() {
            let x = input_string[..i].parse().expect("x não é inteiro");
            let y = input_string[i + 1..].parse().expect("y não é inteiro");

            return (x, y);
        }
    }

    panic!("não há espaço separando primeiro número do segundo");
}
