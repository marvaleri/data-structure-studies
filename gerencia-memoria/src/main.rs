struct MyResource {
    data: Vec<i32>,
}

impl MyResource {
    // Cria um novo recurso e aloca memória para um vetor
    fn new() -> MyResource {
        println!("MyResource criado.");
        MyResource { data: vec![1, 2, 3] }
    }

    // Método que utiliza os dados armazenados
    fn do_something(&self) {
        println!("MyResource está usando os dados: {:?}", self.data);
    }
}

// Implementa o trait Drop para liberar recursos automaticamente
impl Drop for MyResource {
    fn drop(&mut self) {
        println!("MyResource sendo descartado. Liberando memória...");
    }
}

fn main() {
    {
        // Cria uma instância de MyResource
        let resource = MyResource::new();
        resource.do_something();
        // Ao final deste bloco, 'resource' sai de escopo e o método 'drop' é chamado automaticamente
    }
    println!("O recurso saiu de escopo e sua memória foi liberada.");
}
