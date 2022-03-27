use std::io;

fn main() {
    // mut = mutavel - pode ser mudar o valor da var
    let mut valor1 = String::new();
    println!("Digite o primeiro valor :");
    io::stdin().read_line(&mut valor1).expect("Falha ao ler o valor inserido");
    let mut num1: i32 = valor1.trim().parse().unwrap();
    
    let mut valor2 = String::new();
    println!("Digite o segundo valor :");
    io::stdin().read_line(&mut valor2).expect("Falha ao ler o valor inserido");
    let mut num2: i32 = valor2.trim().parse().unwrap();
    
    soma(num1, num2)
    
}

fn soma(x: i32, y: i32) {
    println!("{} + {} = {}", x, y, x + y);
}
