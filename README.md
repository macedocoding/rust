# Aprendendo Rust

As anotações a seguir foram feitas a partir do livro *The Rust Programming Language* disponível no conjunto de arquivos baixados pela ferramenta de instalação ***rustup*** e acessível como arquivo HTML através do comando `rustup doc`.

## Alguns comandos

Utilizando a ferramenta de gestão de sistemas e pacotes Cargo, é muito mais fácil trabalharmos com projetos Rust, pois ela nos permite organizar (códigos-fonte e dependências), checar e compilar de uma maneira fácil e objetiva em quaisquer sistemas operacionais.

`cargo new projeto` - Nos permite criar um diretório *projeto*, que conterá nossos arquivos de projeto contendo, inicialmente, um arquivo de configuração ***Cargo.toml***, que gerenciará dependências e outras informações acerca do código.

Uma vez dentro do diretório gerado com o comando anterior, podemos editar o código-fonte contido em ***src/main.rs*** e, em seguida, compilá-lo com o compilador ***rustc***, atrvés do comando `cargo build`. Isto gerará um binário executável ***target/debug/projeto*** que poderá ser executado manualmente através de um terminal.

Uma alternativa que compila e executa o programa em seguida pode ser feita com o comando `cargo run`.

`cargo check` - Este comando checa se o código não possui erros e pode ser compilado. É mais rápido que o comando anterior, pois não gera o binário executável, por isso é frequentemente utilizado durante a escrita de códigos.

`cargo build --release` - Adicionando-se o parâmetro `--release`, gera-se um executável com otimizações, tornando-o mais rápido em sua execução, em detrimento de um tempo de compilação maior. É utilizado, geralmente, quando o programa desenvolvido está pronto para ser distribuído, ou seja, passou pelas fases de desenvolvimento. O executável gerado por este comando fica disponível no diretório ***target/release*** com o mesmo nome do diretório Cargo.