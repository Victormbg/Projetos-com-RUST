use std::io;
#![feature(type_name_of_val)]
use std::any::type_name_of_val;


struct Dados {
    num1: u32,
    num2: u32
}

//Dados{num1:50,num2:30}

fn main() {
    // mut = mutavel - pode ser mudar o valor da var
    let mut num1;
    println!("Digite o primeiro valor :");
    io::stdin().read_line(&mut num1).expect("Falha ao ler entrada");
    
    let mut num2;
    println!("Digite o segundo valor :");
    io::stdin().read_line(&mut num2).expect("Falha ao ler entrada");
    
    println!("{}", type_name_of_val(&num1));

    //let res = soma(num1,num2);

    //println!("teste: {}", res);
}
/*
fn soma(num1,num2) {
    return num1+num2;
}
*/