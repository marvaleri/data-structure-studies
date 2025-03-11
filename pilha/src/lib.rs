pub struct Stack<T> {
    elements: Vec<T>,
}

impl<T> Stack<T> {
    // Cria uma pilha (stack)
    pub fn new() -> Stack<T> {
        Stack { elements: Vec::new() }
    }

    pub fn push(&mut self, value: T) {
        self.elements.push(value);
    }

    // Remove e retorna o elemento do topo da pilha
    pub fn pop(&mut self) -> Option<T> {
        self.elements.pop()
    }

    // Retorna uma referencia ao elemento do topo da pilha
    pub fn peek(&self) -> Option<&T> {
        self.elements.last()
    }

    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stack_operations() {
        let mut stack = Stack::new();

        // Testando se a pilha esta inicialmente vazia
        assert!(stack.is_empty());

        // Adicionando elementos a pilha
        stack.push(10);
        stack.push(20);

        // Verificando o elemento do topo da pilha
        assert_eq!(stack.peek(), Some(&20));

        // Removendo elementos da pilha e verificando
        assert_eq!(stack.pop(), Some(20));
        assert_eq!(stack.pop(), Some(10));

        // Testando se a pilha esta vazia novamente
        assert!(stack.is_empty());
    }
}