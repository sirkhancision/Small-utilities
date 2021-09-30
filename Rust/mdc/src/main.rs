use std::io;
use std::str;
use std::mem;

fn main() {
    let result: u128;
    let mut input = String::new();

    println!("Digite dois números para calcular o MDC:");

    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler entrada.");
    let input = slice_input(&input);

    if let (0, 0) = input {
        panic!("Entrada inválida.")
    } else {
        result = mdc(input);
        println!("MDC: {}", result)
    }
}

fn mdc(mut input: (u128, u128)) -> u128 {
    if input.1 > input.0 {
        mem::swap(&mut input.0, &mut input.1);
    }

    // Euclidean algorithm
    while input.0 % input.1 != 0 {
        mem::swap(&mut input.0, &mut input.1);
        input.1 %= input.0;
    }

    return input.1
}

// Slices the input string and outputs a u128 tuple, with
// the two numbers for calculating the GDC
fn slice_input(input_string: &str) -> (u128, u128) {
    let string_bytes = input_string.trim().as_bytes();
    let mut input: (u128, u128) = (0, 0);

    for (i, &element) in string_bytes.iter().enumerate() {
        if element == b' ' {
            let x: u128 = str::from_utf8(&string_bytes[..i])
                .expect("Erro ao converter x para string.")
                .parse()
                .expect("x não é inteiro.");
            let y: u128 = str::from_utf8(&string_bytes[i + 1..])
                .expect("Erro ao converter y para string.")
                .parse()
                .expect("y não é inteiro.");

            input = (x, y);

            break;
        }
    }

    return input
}