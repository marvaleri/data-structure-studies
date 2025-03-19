use std::sync::{Arc, Mutex};
use std::thread;

// Função que cria um vetor dentro de um Mutex, compartilhado entre threads
fn main() {
    // Arc permite múltiplas referências entre  threads
    // Mutex protege o acesso concorrente ao vetor
    let shared_data = Arc::new(Mutex::new(vec![1, 2, 3]));

    let mut handles = vec![];

    for i in 0..5 {
        let data_ref = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            // Bloqueia o Mutex para acesso exclusivo
            let mut data = data_ref.lock().unwrap();
            data.push(i);
            println!("Thread {} inseriu o valor {}", i, i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Após todas as threads terminarem, podemos ver o resultado
    println!("Vetor final: {:?}", shared_data.lock().unwrap());
}
