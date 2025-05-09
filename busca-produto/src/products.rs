use std::collections::HashMap;

#[derive(Debug)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub category: String,
}

pub fn create_product_table() -> HashMap<u32, Product> {
    let mut table = HashMap::new();

    table.insert(
        1001,
        Product {
            id: 1001,
            name: "Xbox 360".to_string(),
            category: "Eletrônicos".to_string(),
        },
    );

    table.insert(
        1002,
        Product {
            id: 1002,
            name: "Pé-de-cabra".to_string(),
            category: "Ferramentas".to_string(),
        },
    );

    table.insert(
        1003,
        Product {
            id: 1003,
            name: "Luz 360 lumenz".to_string(),
            category: "Eletrônicos".to_string(),
        },
    );

    table.insert(
        1004,
        Product {
            id: 1004,
            name: "Tênis Nike".to_string(),
            category: "Vestuário".to_string(),
        },
    );

    table.insert(
        1005,
        Product {
            id: 1005,
            name: "Machado".to_string(),
            category: "Ferramenta".to_string(),
        },
    );

    table.insert(
        1006,
        Product {
            id: 1006,
            name: "Boné Adidas".to_string(),
            category: "Vestuário".to_string(),
        },
    );

    table.insert(
        1007,
        Product {
            id: 1007,
            name: "PlayStation 5".to_string(),
            category: "Eletrônicos".to_string(),
        },
    );

    table.insert(
        1008,
        Product {
            id: 1008,
            name: "Jaqueta de couro".to_string(),
            category: "Vestuário".to_string(),
        },
    );

    table
}