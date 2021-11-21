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

    #[test]
    fn shortening_of_10_multiples() {
        let fraction_1 = ("335000000000", "550000000");
        let fraction_2 = ("33500", "55");
        let shortened =
            Fraction::shorten_mul_10(fraction_1.0, fraction_1.1, fraction_1.1.len() as i32);

        assert_eq!(fraction_2, shortened);
    }
}

pub struct Fraction {
    numerator: Option<i128>,
    denominator: Option<i128>,
}

impl Fraction {
    // Parses the numerator and denominator as strings, and does operations to check if they're
    // valid integers and to make the simplification proccess faster
    fn parse_fraction(fraction_string: (String, String)) -> Result<(i128, i128), &'static str> {
        let longer: i32 = if fraction_string.0.len() > fraction_string.1.len() {
            fraction_string.0.len() as i32
        } else {
            fraction_string.1.len() as i32
        };

        let fraction_string =
            Fraction::shorten_mul_10(fraction_string.0.trim(), fraction_string.1.trim(), longer);

        let numerator_result = match fraction_string.0.parse() {
            Err(_e) => return Err("numerador não é inteiro"),
            Ok(n) => n,
        };

        let denominator_result = match fraction_string.1.parse() {
            Ok(0) => return Err("divisão por zero não permitida"),
            Ok(n) => n,
            Err(_e) => return Err("denominador não é inteiro"),
        };

        Ok((numerator_result, denominator_result))
    }

    // Shortens numbers if they're both divisible by 10
    fn shorten_mul_10<'a>(
        numerator: &'a str,
        denominator: &'a str,
        longer: i32,
    ) -> (&'a str, &'a str) {
        let mut numerator = numerator.chars();
        let mut denominator = denominator.chars();

        if !(numerator.as_str().is_empty() || denominator.as_str().is_empty()) {
            if denominator.as_str() != "0" {
                for _i in 0..=longer {
                    if numerator.to_owned().last().unwrap() == '0'
                        && denominator.to_owned().last().unwrap() == '0'
                    {
                        numerator.next_back();
                        denominator.next_back();
                    }
                }
            }
        }
        (numerator.as_str(), denominator.as_str())
    }

    // Creates a new fraction out of a string slice
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

    // Prints the simplified fraction normally
    pub fn print_fraction(self) {
        println!(
            "Fração simplificada: {}/{}",
            self.numerator.unwrap(),
            self.denominator.unwrap()
        );
    }

    // Prints only the resulting simplified fraction, without additional text
    pub fn print_fraction_quiet(self) {
        println!("{}/{}", self.numerator.unwrap(), self.denominator.unwrap());
    }
}
