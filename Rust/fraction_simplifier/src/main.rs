use std::io;
use std::str;

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
            panic!("Não é possível simplificar fração nula.")
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
                let numerator_result: i128 = str::from_utf8(&string_bytes[..i])
                    .expect("Erro ao converter numerador para string.")
                    .parse()
                    .expect("Numerador não é inteiro.");
                let denominator_result: i128 = str::from_utf8(&string_bytes[i + 1..])
                    .expect("Erro ao converter denominador para string.")
                    .parse()
                    .expect("Denominador não é inteiro.");

                fraction.numerator = Some(numerator_result);
                fraction.denominator = Some(denominator_result);

                break;
            }
        }

        return fraction;
    }
}

fn main() {
    let mut frac1: Fraction;
    let mut input = String::new();

    println!("Digite a fração a ser simplificada:");

    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler fração.");

    frac1 = Fraction::slice_fraction(input);

    if frac1.denominator == Some(0) {
        panic!("Insira uma fração válida: divisão por zero não permitida.")
    } else {
        frac1.simplify();
        println!(
            "Fração simplificada: {}/{}",
            frac1.numerator.unwrap(),
            frac1.denominator.unwrap()
        )
    }
}
