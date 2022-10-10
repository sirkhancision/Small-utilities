use std::io;

fn main() {
    // maximum position before overflow
    const MAX: u128 = 186;
    // to generate the the nth number
    let mut n = String::new();
    // to store n-1 and n-2 values from the sequence
    let mut n_minus = (0, 1);
    // the fibonacci number to be printed
    let mut fib_number = 0;

    println!("Type the position of the sequence to be printed (max: 186):");

    io::stdin().read_line(&mut n).expect("error reading n");
    let n = n.trim();

    if !n.is_empty() {
        let n_digit = &n.chars().last().unwrap();

        let n: u128 = n.parse().expect("n isn't a valid integer");
        let n_digit: u128 = n_digit.to_string().parse().unwrap();

        if n > MAX {
            println!("The result of position {} exceeds 128 bits", n);
        } else {
            // decide on the type of suffix to use after
            // the sequence position number
            let suffix = if n_digit < 4 && n_digit != 0 {
                if n_digit == 1 {
                    "st"
                } else if n_digit == 2 {
                    "nd"
                } else {
                    "rd"
                }
            } else {
                "th"
            };

            if n < 2 {
                fib_number = n;
            } else {
                for i in 2..=n {
                    fib_number = n_minus.0 + n_minus.1;

                    if i % 2 == 0 {
                        n_minus.0 += n_minus.1;
                    } else {
                        n_minus.1 += n_minus.0;
                    }
                }
            }

            println!(
                "Fibonnaci number at {}{} position: {}",
                n, suffix, fib_number
            );
        }
    }
}
