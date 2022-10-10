use std::io;

fn main() {
    let mut cl = String::new();
    println!("Digite o valor do Ciclo-CL:");

    io::stdin()
        .read_line(&mut cl)
        .expect("Erro ao ler Ciclo-CL");

    let cl: f64 = cl
        .trim()
        .parse()
        .expect("ciclo cl dado não é um número inteiro");

    let mut mhz = String::new();
    println!("Agora, digite o valor do MHz-RAM:");

    io::stdin()
        .read_line(&mut mhz)
        .expect("Erro ao ler MHz-RAM");

    let mhz: f64 = mhz
        .trim()
        .parse()
        .expect("mhz da ram dado não é um número inteiro");

    let clock_speed = (2.0 * cl * 1E9) / (mhz * 1E6);

    println!("Clock Speed da RAM: {:.1} ns", clock_speed);
}
