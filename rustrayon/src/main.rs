use rayon::prelude::*;
use std::time::Instant;

fn main() {
    // Cria um vetor com números de 1 até 10.000.000
    let numbers: Vec<i32> = (1..=10_000_000).collect();

    // Mede o tempo para calcular a soma dos quadros
    let start = Instant::now();
    let sum_squares: u128 = numbers
        .par_iter()
        .map(|&x| (x as u128) * (x as u128))
        .sum();
    let elapsed = start.elapsed();
    println!("Soma dos quadrados: {}", sum_squares);
    println!("Tempo de cálculo dos quadrados: {:.2?}", elapsed);

    // Mede o tempo para filtrar os números pares
    let start_even = Instant::now();
    let even_numbers: Vec<i32> = numbers
        .par_iter()
        .filter(|&&x| x % 2 == 0)
        .cloned()
        .collect();
    let elapsed_even = start_even.elapsed();
    println!("Quantidade de números pares: {}", even_numbers.len());
    println!("Tempo de filtragem dos pares: {:.2?}", elapsed_even);
}
