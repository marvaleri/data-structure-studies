// Exemplo de tabela hash com função de dispersão 

// Definindo uma estrutura para a tabela hash
// Cada "Balde" é um vetor de pares (String i32)
#[derive(Debug)]
struct HashTable {
    buckets: Vec<Vec<(String, i32)>>,
    size: usize
}

impl HashTable {
    // Cria uma nova tabela hash com um número especificado de baldes
    fn new(size: usize) -> Self {
        let mut buckets = Vec::with_capacity(size);
        // Inicializa cada balde com um vetor vazio
        for _ in 0..size {
            buckets.push(Vec::new());
        }
        HashTable { buckets, size }
    }

    // Função de dispersão simples:
    // Soma os valores dos bytes da chave e calcula o módulo pelo tamanho da tabela
    // Essa função transforma a chave em um índice de 0 até size-1
    fn simple_hash(&self, key: &String) -> usize {
        let sum: usize = key.bytes().map(|b| b as usize).sum();
        sum % self.size
    }

    // Insere um par (chave, valor) na tabela hash
    fn insert(&mut self, key: String, value: i32) {
        let index = self.simple_hash(&key);
        // Verifica se a chave já existe no balde; se existir, atualiza o valor
        for entry in self.buckets[index].iter_mut() {
            if entry.0 == key {
                entry.1 = value;
                return;
            }
        }
        // Se não existir, insere o novo par no balde
        self.buckets[index].push((key, value));
    }

    // Recuper o valor associado a uma chave, se existir
    fn get(&self, key: &String) -> Option<i32> {
        let index = self.simple_hash(key);
        for entry in &self.buckets[index] {
            if &entry.0 == key {
                return Some(entry.1);
            }
        }
        None
    }
}

fn main() {
    // Cria uma tabela hash com 10 baldes
    let mut table = HashTable::new(10);

    // Insere alguns pares (chave, valor)
    table.insert("chave1".to_string(), 100);
    table.insert("chave2".to_string(), 200);
    table.insert("outra_chave".to_string(), 300);
    table.insert("chave_prof".to_string(), 654);


    // Recupera os valores associados às chaves
    println!("Valor para 'chave1': {:?}", table.get(&"chave1".to_string()));
    println!("Valor para 'cahve2': {:?}", table.get(&"chave2".to_string()));
    println!("Valor para 'outro_chave': {:?}", table.get(&"outra_chave".to_string()));
    println!("Valor para 'chave_prof' {:?}", table.get(&"chave_prof".to_string()));
    println!("Valor para 'nao_existe' {:?}", table.get(&"nao_existe".to_string()));
}
