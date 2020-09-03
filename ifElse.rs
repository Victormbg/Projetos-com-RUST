fn main() {
    let mut nome = String::new();
    println!("Digite seu nome :");
    std::io::stdin().read_line(&mut nome).unwrap();
    
    if nome == "Victor" {
        print!("E o Victor\n");
    } else {
        print!("Nao e o Victor\n{}",nome);
    }
}
