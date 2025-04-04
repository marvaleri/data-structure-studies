use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};

fn main() {
    // Exemplo1: Hashmap - Estrutura de dados associativa (dicionário)
    let mut scores = HashMap::new();
    scores.insert(String::from("Alice"), 90);
    scores.insert(String::from("Bob"), 75);
    scores.insert(String::from("Carol"), 85);
    println!("HashMap (scores): {:?}", scores);

    // Itera sobre as chaves e valores
    for (name, scores) in &scores {
        println!("{}: {}", name, scores);
    }

    // Exemplo 2: HashSet - Conjunto de valores únicos
    let mut unique_numbers = HashSet::new();
    unique_numbers.insert(1);
    unique_numbers.insert(2);
    unique_numbers.insert(3);
    unique_numbers.insert(2); // Tentativa de inserir duplicata
    println!("HashSet (números únicos): {:?}", unique_numbers);

    // Exemplo 3: VecDeque - Fila de deque (double-ended queue)
    let mut deque = VecDeque::new();
    deque.push_back(10);
    deque.push_back(20);
    deque.push_front(5);
    println!("VecDeque: {:?}", deque);
    // Remove elemento do início
    if let Some(front) = deque.pop_front() {
        println!("Elemento removido do início: {}", front);
    }

    // Exemplo 4: BinaryHeap - Heap binário (fila de prioridade)
    let mut heap = BinaryHeap::new();
    heap.push(8);
    heap.push(3);
    heap.push(5);
    println!("BinaryHeap (max-heap): {:?}", heap);
    // O elemento no topo é o maior
    if let Some(max) = heap.peek() {
        println!("Elemento máximo: {}", max);
    }
}
