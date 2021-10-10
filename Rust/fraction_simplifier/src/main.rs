use std::{io, str::from_utf8};

struct Fraction {
    numerator: Option<i128>,
    denominator: Option<i128>,
}

impl Fraction {
    // Simplifies the fraction down to its smaller factors
    fn simplify(&mut self) {
        if let Fraction {
            numerator: Some(x),
            denominator: Some(y),
        } = self
        {
            let i = 2;

            for i in i..10 {
                // Checks if they're divisible by i, from 2 to 9
                while *x % i == 0 && *y % i == 0 {
                    *x /= i;
                    *y /= i;
                }
            }
        } else {
            println!("Não é possível simplificar fração nula.")
        }
    }

    // Slices the input string and outputs a i128 tuple, with
    // the numerator and denominator of the fraction
    fn slice_fraction(fraction_string: String) -> Fraction {
        let string_bytes = fraction_string.trim().as_bytes();
        let mut fraction = Fraction {
            numerator: None,
            denominator: None,
        };

        for (i, &element) in string_bytes.iter().enumerate() {
            if element == b'/' {
                let numerator_result: i128 = from_utf8(&string_bytes[..i])
                    .expect("Erro ao converter numerador para string")
                    .parse()
                    .expect("Numerador não é inteiro");

                let denominator_result: i128 = from_utf8(&string_bytes[i + 1..])
                    .expect("Erro ao converter denominador para string")
                    .parse()
                    .expect("Denominador não é inteiro");

                fraction.numerator = Some(numerator_result);
                fraction.denominator = Some(denominator_result);

                break;
            }
        }

        fraction
    }
}

fn main() {
    let mut input = String::new();

    println!("Digite a fração a ser simplificada:");

    if let Err(e) = io::stdin().read_line(&mut input) {
        println!("Erro: {}", e)
    }

    let mut frac = Fraction::slice_fraction(input);

    if frac.denominator == Some(0) {
        println!("Insira uma fração válida: divisão por zero não permitida.")
    } else {
        frac.simplify();
        match frac {
            Fraction {
                numerator: Some(x),
                denominator: Some(y),
            } => println!("Fração simplificada: {}/{}", x, y),
            _ => println!("Erro ao imprimir fração simplificada."),
        }
    }
}
