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

    io::stdin().read_line(&mut n).expect("Error reading n");
    let n: u128 = n.trim().parse().expect("n isn't a valid integer");

    if n > MAX {
        println!("The result of position {} exceeds 128 bits", n);
    } else {
        // decide on the type of suffix to use after
        // the sequence position number
        let suffix = if n < 4 && n != 0 {
            if n == 1 {
                "st"
            } else if n == 2 {
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
            // increment variable
            let i = 2;

            for i in i..=n {
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
        )
    }
}
