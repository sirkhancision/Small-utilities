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
    fn parse_fraction(mut fraction_string: (String, String)) -> Result<(i128, i128), &'static str> {
        let longer: i32 = if fraction_string.0.len() > fraction_string.1.len() {
            fraction_string
                .0
                .len()
                .to_owned()
                .to_string()
                .parse()
                .unwrap()
        } else {
            fraction_string
                .1
                .len()
                .to_owned()
                .to_string()
                .parse()
                .unwrap()
        };

        {
            let mut numerator = fraction_string.0.trim().chars();
            let mut denominator = fraction_string.1.trim().chars();

            // Shortens the number by removing mutual zeros at the right
            for _i in 0..=(longer as i32) {
                if numerator.to_owned().last().unwrap() == '0'
                    && denominator.to_owned().last().unwrap() == '0'
                {
                    numerator.next_back();
                    denominator.next_back();
                }
            }

            fraction_string = (
                numerator.as_str().to_string(),
                denominator.as_str().to_string(),
            );
        }

        let numerator_result = match fraction_string.0.parse() {
            Err(_e) => return Err("numerador não é inteiro"),
            Ok(n) => n,
        };

        let denominator_result = match fraction_string.1.parse() {
            Err(_e) => return Err("denominador não é inteiro"),
            Ok(0) => return Err("divisão por zero não permitida"),
            Ok(n) => n,
        };

        Ok((numerator_result, denominator_result))
    }

    pub fn new(fracao: &str) -> Result<Fraction, &str> {
        for (i, char) in fracao.chars().enumerate() {
            if char == '/' {
                let fraction = Fraction::parse_fraction((
                    fracao[..i].to_string(),
                    fracao[i + 1..].to_string(),
                ))?;

                return Ok(Fraction {
                    numerator: Some(fraction.0),
                    denominator: Some(fraction.1),
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
