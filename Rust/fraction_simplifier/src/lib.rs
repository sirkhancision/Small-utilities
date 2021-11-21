#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fraction_simplification() {
        let fraction_1 = Fraction::new("255/50").unwrap();
        let fraction_2 = Fraction::new("51/10").unwrap();
        let simplified = Fraction::simplify(fraction_1);

        assert_eq!(
            (fraction_2.numerator, fraction_2.denominator),
            (simplified.numerator, simplified.denominator)
        );
    }
}

pub struct Fraction {
    numerator: Option<i128>,
    denominator: Option<i128>,
}

impl Fraction {
    pub fn new(fracao: &str) -> Result<Fraction, &str> {
        for (i, char) in fracao.chars().enumerate() {
            if char == '/' {
                let numerator_result = match fracao[..i].parse() {
                    Err(_e) => return Err("numerador não é inteiro"),
                    Ok(n) => n,
                };

                let denominator_result = match fracao[i + 1..].parse() {
                    Err(_e) => return Err("denominador não é inteiro"),
                    Ok(0) => return Err("divisão por zero não permitida"),
                    Ok(n) => n,
                };

                return Ok(Fraction {
                    numerator: Some(numerator_result),
                    denominator: Some(denominator_result),
                });
            }
        }
        Err("não há barra separando numerador e denominador")
    }

    // Simplifies the fraction down to its smaller factors
    pub fn simplify(self) -> Fraction {
        let lower = if &self.numerator < &self.denominator {
            self.numerator.unwrap()
        } else {
            self.denominator.unwrap()
        };
        let mut fraction = self;

        for i in 2..=lower {
            // Checks if they're divisible by i, from 2 to the greatest value of the fraction
            while fraction.numerator.unwrap() % i == 0 && fraction.denominator.unwrap() % i == 0 {
                fraction.numerator.replace(fraction.numerator.unwrap() / i);
                fraction
                    .denominator
                    .replace(fraction.denominator.unwrap() / i);
            }
        }

        fraction
    }

    pub fn print_fraction(self) {
        println!(
            "Fração simplificada: {}/{}",
            self.numerator.unwrap(),
            self.denominator.unwrap()
        );
    }

    pub fn print_fraction_quiet(self) {
        println!("{}/{}", self.numerator.unwrap(), self.denominator.unwrap());
    }
}
