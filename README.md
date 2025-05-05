# Data Structure and Implementation with Rust

Este repositório contém a implementação de diversas estruturas de dados utilizando a linguagem **Rust**, como parte da disciplina _Data Structure and Implementation_.

## 📚 Objetivo

O objetivo desta disciplina é proporcionar uma compreensão sólida das principais estruturas de dados e como elas podem ser implementadas de maneira eficiente e segura em Rust, aproveitando suas características como segurança de memória, concorrência e performance.

## 🦀 Por que Rust?

Rust é uma linguagem de sistemas moderna que oferece:
- Segurança de memória sem coletor de lixo (garbage collector)
- Alta performance
- Ferramentas poderosas de concorrência
- Tipagem estática e inferência sofisticada
- Excelente gerenciamento de erros

Essas características a tornam ideal para estudar implementações de estruturas de dados com alto desempenho e robustez.

## 📦 Crates Utilizadas

As seguintes crates foram utilizadas ao longo dos projetos para auxiliar na serialização, entrada/saída e paralelização:

- [`serde`](https://crates.io/crates/serde): Usada para serialização e desserialização de estruturas de dados em formatos como JSON, facilitando a persistência e troca de dados.
- [`rayon`](https://crates.io/crates/rayon): Utilizada para processamento paralelo, permitindo acelerar algoritmos com paralelização de forma segura e eficiente.
