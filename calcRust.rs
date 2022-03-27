use std::io;

struct Dados {
    num1: u32,
    num2: u32
}

//Dados{num1:50,num2:30}

fn main() {
    // mut = mutavel - pode ser mudar o valor da var
    let mut input_text = String::new();
    io::stdin()
        .read_line(&mut input_text)
        .expect("failed to read from stdin");

    let trimmed = input_text.trim();
    match trimmed.parse::<u32>() {
        Ok(i) => println!("your integer input: {}", i),
        Err(..) => println!("this was not an integer: {}", trimmed),
    };
    
}
/*
fn soma(num1,num2) {
    return num1+num2;
}
*/