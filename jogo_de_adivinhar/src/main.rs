// Importando a biblioteca io(input/output) que vem com a biblioteca padrão std através da palavra use.
use std::io;

// Todo programa rust inicia pela função main
fn main() {

    // Macro que imprime no output padrão o que for passado como argumento.
    println!("Adivinhe o número!");

    println!("Por favor, digite um número: ");

    // Inicializando uma variável mutável chute com uma instância do objeto String
    let mut chute = String::new();

    io::stdin()
        .read_line(&mut chute)
        .expect("Falha ao ler linha");
    
    println!("Você chutou: {}", chute);
}