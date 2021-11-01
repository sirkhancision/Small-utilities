use std::mem;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mdc_euclidean() {
        let mdc = Mdc { first_var: 255 , second_var: 50, };

        assert_eq!(Mdc::calculate(mdc), Ok(5));
    }
}

pub struct Mdc {
    first_var: u128,
    second_var: u128,
}

impl Mdc {
    pub fn new(input: &Vec<String>) -> Result<Self, &str> {
        let x: u128 = match input[1].as_str().parse() {
            Err(_e) => return Err("x não é inteiro"),
            Ok(n) => n,
        };
        let y: u128 = match input[2].as_str().parse() {
            Err(_e) => return Err("y não é inteiro"),
            Ok(n) => n,
        };

        Ok(Mdc {
            first_var: x,
            second_var: y,
        })
    }

    pub fn calculate(mut input: Self) -> Result<u128, &'static str> {
        if input.first_var == 0 || input.second_var == 0 {
            return Err("divisão por zero não é permitida");
        } else if input.second_var > input.first_var {
            mem::swap(&mut input.first_var, &mut input.second_var);
        }

        // Euclidean algorithm
        while input.first_var % input.second_var != 0 {
            mem::swap(&mut input.first_var, &mut input.second_var);
            input.second_var %= input.first_var;
        }

        Ok(input.second_var)
    }
}
