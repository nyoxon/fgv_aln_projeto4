# fgv_aln_projeto4
repositório temporário referente a um projeto da disciplina Álgebra Linear Númerica da FGV-EMAP no 1º semestre de 2025

## Requisitos

- [Rust](https://www.rust-lang.org/tools/install)
- [Gnuplot devidamente instalado no sistema](http://www.gnuplot.info/download.html)

## Estrutura do projeto

<pre>
.
├── Cargo.toml             # Configuração do projeto Rust
├── README.md              # Instruções

├── src                    # Código fonte principal
│   └── main.rs
│   └── plots.rs
│   └── matrix.rs

├── histograma.png         # Saída de gráficos gerados pelo programa
├── relatorio.pdf          # Relatório do projeto
</pre>

## Rodando o projeto

Primeiro, certifique-se de mudar a função main do arquivo `src/main.rs` para receber o output do item desejado.
Por exemplo, se quero ver os plots referentes ao item c, a função main deve ficar:

`fn main {
    c();
}`

Agora, estando dentro do diretório do repositório, rode o programa com:

`cargo run`

E claro, fique a vontade para mudar quais parâmetros dentro dos códigos para resultados diferentes.
