use std::io;

fn main() {
    let mut input = String::new();

    println!("Digite a fração a ser simplificada:");

    io::stdin()
        .read_line(&mut input)
        .expect("erro ao ler fração da entrada padrão");
    let input = input.trim();

    if !input.is_empty() {
        let frac = Fraction::new(input).simplify();

        println!(
            "Fração simplificada: {}/{}",
            frac.numerator.unwrap(),
            frac.denominator.unwrap()
        );
    }
}

struct Fraction {
    numerator: Option<i128>,
    denominator: Option<i128>,
}

impl Fraction {
    fn new(fracao: &str) -> Fraction {
        let fracao = Fraction::slice_fraction(fracao);

        if let (Some(_), Some(0)) = (fracao.numerator, fracao.denominator) {
            panic!("divisão por zero não permitida")
        } else {
            fracao
        }
    }

    // Slices the input string and outputs a i128 tuple, with
    // the numerator and denominator of the fraction
    fn slice_fraction(fraction_string: &str) -> Fraction {
        for (i, char) in fraction_string.chars().enumerate() {
            if char == '/' {
                let numerator_result = fraction_string[..i]
                    .parse()
                    .expect("numerador não é inteiro");

                let denominator_result = fraction_string[i + 1..]
                    .parse()
                    .expect("denominador não é inteiro");

                return Fraction {
                    numerator: Some(numerator_result),
                    denominator: Some(denominator_result),
                };
            }
        }
        panic!("não há barra separando numerador e denominador");
    }

    // Simplifies the fraction down to its smaller factors
    fn simplify(self) -> Fraction {
        let mut fraction = self;

        for i in 2..10 {
            // Checks if they're divisible by i, from 2 to 9
            while fraction.numerator.unwrap() % i == 0 &&
                fraction.denominator.unwrap() % i == 0 {
                fraction
                    .numerator
                    .replace(fraction.numerator.unwrap() / i);
                fraction
                    .denominator
                    .replace(fraction.denominator.unwrap() / i);
            }
        }

        fraction
    }
}
