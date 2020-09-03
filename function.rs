#![allow(non_snake_case)]
fn main() {
    let mut nome = String::new();
    println!("Digite seu nome :");
    std::io::stdin().read_line(&mut nome).unwrap();
    verificaNome(nome);
}

fn verificaNome(nome: String) {
    if nome == "Victor" {
        print!("E o Victor\n");
    } else {
        print!("Nao e o Victor\nVoce digitou isso: {}",nome);
    }
}