use std::io;

fn main() {
    let mut input = String::new();
    println!("Conversão de F° para C° [1] ou de C° para F° [2]?");

    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler input");
    let input: f64 = input.trim().parse().expect("input não é um float válido");

    if input == 1.0 || input == 2.0 {
        println!("Digite a temperatura:");
        println!("{}", conversion(input));
    } else {
        println!("Opção inválida");
    }
}

fn conversion(option: f64) -> String {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Erro ao ler input");
    let input: f64 = input.trim().parse().expect("input não é um float válido");

    let output;
    let suffix;

    if option == 1.0 {
        output = 5.0 / 9.0 * (input - 32.0);
        suffix = "C°";
    } else {
        output = (9.0 / 5.0 * input) + 32.0;
        suffix = "F°";
    }

    format!("{:.1} {}", output, suffix)
}
