use unicode_normalization::UnicodeNormalization;
use std::collections::HashMap;
use std::io::{self, Write};

#[derive(Debug)]
struct Product {
    id: u32,
    name: String,
    category: String,
}

fn preprocess(text: &str) -> String {
    text.nfkd() // Normaliza acentos (ex: "á" -> "a" + "~")
        .filter(|c| c.is_ascii() && c.is_alphanumeric() || *c == ' ')
        .collect::<String>()
        .to_lowercase()
}

fn main() {
    // Cria a tabela hash (HashMap) com índice do produto como chave
    let mut product_table: HashMap<u32, Product> = HashMap::new();

    // Adiciona produtos
    product_table.insert(
        1001,
        Product {
            id: 1001,
            name: "Xbox 360".to_string(),
            category: "Eletrônicos".to_string(),
        },
    );

    product_table.insert(
        1002,
        Product {
            id: 1002,
            name: "Pé-de-cabra".to_string(),
            category: "Ferramentas".to_string(),
        },
    );

    product_table.insert(
        1003,
        Product {
            id: 1003,
            name: "Luz 360 lumenz".to_string(),
            category: "Eletrônicos".to_string(),
        },
    );

    product_table.insert(
        1004,
        Product {
            id: 1004,
            name: "Tênis Nike".to_string(),
            category: "Vestuário".to_string(),
        },
    );

    // "-----Barra de pesquisa-----"
    loop {
        print!("\nDigite um termo de busca (ou 'sair' para encerrar): ");
        io::stdout().flush().unwrap(); // força exibir o prompt antes de ler

        let mut search_term = String::new();
        io::stdin().read_line(&mut search_term).expect("Erro na leitura");
        let search_term = search_term.trim();

        if search_term.eq_ignore_ascii_case("sair") {
            break;
        }

        let search_term = preprocess(search_term);

        let result: Vec<&Product> = product_table
            .values()
            .filter(|product| preprocess(&product.name).contains(&search_term))
            .collect();

        if result.is_empty() {
            println!("Nenhum produto encontrado para esse termo.");
        } else {
            for p in result {
                println!("Produto encontrado: {:?}", p);
            }
        }
    }


    // Itera sobre todos os produtos
    println!("\nCatálogo completo:");
    for (id, product) in &product_table {
        println!("ID {}: {} - {}", id, product.name, product.category);
    }

    // Cache de buscas
    // let mut search_cache: HashMap<String, Vec<u32>> = HashMap::new();

    // // Simulação de várias buscas
    // let search_inputs = ;

    // for raw_term in search_inputs {
    //     let search_term = preprocess(raw_term);
    //     println!("\nBusca por: \"{}\"", raw_term);

    //      // Verifica se está no cache
    //      if let Some(cached_ids) = search_cache.get(&search_term) {
    //         println!("Resultado do cache:");
    //         for id in cached_ids {
    //             if let Some(product) = product_table.get(id) {
    //                 println!("{:?}", product);
    //             }
    //         }
    //     } else {
    //         // Busca nos produtos
    //         let result: Vec<&Product> = product_table
    //             .values()
    //             .filter(|product| preprocess(&product.name).contains(&search_term))
    //             .collect();

    //          // Mostra os resultados
    //          if result.is_empty() {
    //             println!("Nenhum produto encontrado.");
    //         } else {
    //             for p in &result {
    //                 println!("{:?}", p);
    //             }

    //             // Armazena os IDs no cache
    //             let ids: Vec<u32> = result.iter().map(|p| p.id).collect();
    //             search_cache.insert(search_term.clone(), ids);
    //         }
    //     }
    // }
}