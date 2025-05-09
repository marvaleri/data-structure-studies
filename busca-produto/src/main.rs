mod products;

use products::{Product, create_product_table};
use unicode_normalization::UnicodeNormalization;
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Instant;


pub fn preprocess(text: &str) -> String {
    text.nfkd() // Normaliza acentos (ex: "á" -> "a" + "~")
        .filter(|c| c.is_ascii() && c.is_alphanumeric() || *c == ' ')
        .collect::<String>()
        .to_lowercase()
}

fn main() {
    let product_table = create_product_table();

    let mut cache: HashMap<String, Vec<u32>> = HashMap::new();
    let start = Instant::now();


    loop {
        print!("\nDigite um termo de busca (ou 'sair' para encerrar): ");
        io::stdout().flush().unwrap();

        let mut search_term = String::new();
        io::stdin().read_line(&mut search_term).expect("Erro na leitura");
        let search_term = search_term.trim();
        
        if search_term.eq_ignore_ascii_case("sair") {
            break;
        }

        let normalized_term = preprocess(search_term);

        if let Some(cached_ids) = cache.get(&normalized_term) {
            println!("[CACHE] Resultados encontrados:");
            let duration = start.elapsed();
            println!("Tempo de resposta: {:.2?}", duration);
            for id in cached_ids {
                if let Some(product) = product_table.get(id) {
                    println!("-> {:?}", product);
                }
            }
        } else {
            let result: Vec<&Product> = product_table
                .values()
                .filter(|product| preprocess(&product.name).contains(&normalized_term))
                .collect();

            if result.is_empty() {
                println!("Nenhum produto encontrado para esse termo.");
            } else {
                println!("Resultado da busca:");
                let duration = start.elapsed();
                println!("Tempo de resposta: {:.2?}", duration);
                for p in &result {
                    println!("-> {:?}", p);
                }

                let ids: Vec<u32> = result.iter().map(|p| p.id).collect();
                cache.insert(normalized_term, ids);
            }
        }
    }
    
    println!("\nCatálogo completo:");
    for (id, product) in &product_table {
        println!("ID {}: {} - {}", id, product.name, product.category);
    }
    
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_preprocess_remove_acentuacao() {
        let input = "Boné Adidas";
        let expected = "bone adidas";
        assert_eq!(preprocess(input), expected);
    }

    #[test]
    fn test_preprocess_mantem_alfanumerico() {
        let input = "Xbox 360";
        let expected = "xbox 360";
        assert_eq!(preprocess(input), expected);
    }

    #[test]
    fn test_busca_encontra_produto_por_nome() {
        let mut product_table: HashMap<u32, Product> = HashMap::new();
        product_table.insert(1, Product {
        id: 1,
        name: "Luz 360 lumenz".to_string(),
        category: "Eletrônicos".to_string(),
    });

    let term = "Luz 360 lumenz";
    let normalized = preprocess(term);
    let results: Vec<&Product> = product_table
        .values()
        .filter(|p| preprocess(&p.name).contains(&normalized))
        .collect();

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, 1);
    }

    #[test]
    fn test_busca_cache_funciona() {
        let mut cache: HashMap<String, Vec<u32>> = HashMap::new();
        let key = "machado".to_string();
        let ids = vec![1000];
        cache.insert(key.clone(), ids.clone());

        assert!(cache.contains_key(&key));
        assert_eq!(cache.get(&key), Some(&ids));
    }

    #[test]

    fn test_busca_retorna_vazia_ou_nao_encontrada() {
        let mut product_table: HashMap<u32, Product> = HashMap::new();

        product_table.insert(1, Product {
            id: 1,
            name: "PlayStation 5".to_string(),
            category: "Eletrônicos".to_string(),
    });

    let term = "XBox One";
    let normalized = preprocess(term);
    let results: Vec<&Product> = product_table
        .values()
        .filter(|p| preprocess(&p.name).contains(&normalized))
        .collect();

        assert!(results.is_empty());
    }
}