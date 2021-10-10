use std::{io, mem::swap, str::from_utf8};

fn main() {
    let mut input = String::new();

    println!("Digite dois números para calcular o MDC:");

    if let Err(e) = io::stdin().read_line(&mut input) {
        println!("Erro: {}", e)
    }
    let input = slice_input(&input);

    if let (0, 0) = input {
        println!("Entrada inválida.")
    } else {
        let result = mdc(input);
        println!("MDC: {}", result)
    }
}

fn mdc(mut input: (u128, u128)) -> u128 {
    if input.1 > input.0 {
        swap(&mut input.0, &mut input.1);
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
    let string_bytes = input_string.trim().as_bytes();
    let mut input: (u128, u128) = (0, 0);

    for (i, &element) in string_bytes.iter().enumerate() {
        if element == b' ' {
            let x: u128 = from_utf8(&string_bytes[..i])
                .expect("Erro ao converter x para string.")
                .parse()
                .expect("x não é inteiro");
            let y: u128 = from_utf8(&string_bytes[i + 1..])
                .expect("Erro ao converter y para string.")
                .parse()
                .expect("y não é inteiro");

            input = (x, y);
            break;
        }
    }

    input
}
