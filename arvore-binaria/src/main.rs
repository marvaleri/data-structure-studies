use std::cmp;

#[derive(Debug)]
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    height: i32, // Altura do nó (usado apenas no AVL)
}

impl Node {
    // Cria um novo nó
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
            height: 1, // Altura inicial é 1
        }
    }

    // Calcula o fator de balanceamento de um nó
    fn balance_factor(&self) -> i32 {
        height(&self.left) - height(&self.right)
    }

    // Atualiza a altura de um nó
    fn update_height(&mut self) {
        self.height = cmp::max(height(&self.left), height(&self.right)) + 1
    }

    // Rotação à direita
    fn rotate_right(mut self) -> Box<Node> {
        let mut left_child = self.left.take().unwrap();
        self.left = left_child.right.take();
        self.update_height();
        left_child.right = Some(Box::new(self));
        left_child.update_height();
        left_child
    }

    // Rotação à esquerda
    fn rotate_left(mut self) -> Box<Node> {
        let mut right_child = self.right.take().unwrap();
        self.right = right_child.left.take();
        self.update_height();
        right_child.left = Some(Box::new(self));
        right_child.update_height();
        right_child
    }

    // Balanceia a árvore AVL
    fn balance(mut self) -> Box<Node> {
        self.update_height();
        let balance_factor = self.balance_factor();

        // Caso esquerda-esquerda
        if balance_factor > 1 {
            if let Some(left) = self.left {
                if left.balance_factor() < 0 {
                    self.left = Some(left.rotate_left());
                }
            }
            return self.rotate_right();
        }

        // Caso direita-direita
        if balance_factor < -1 {
            if let Some(right) = self.right {
                if right.balance_factor() > 0 {
                    self.right = Some(right.rotate_right());
                }
            }
            return self.rotate_left();
        }

        Box::new(self)
    }

    // Insere um valor na árvore (BST ou AVL)
    fn insert(mut self, value: i32) -> Box<Node> {
        if value < self.value {
            self.left = match self.left {
                Some(left) => Some(left.insert(value)),
                None => Some(Box::new(Node::new(value))),
            };
        } else if value > self.value {
            self.right = match self.right {
                Some(right) => Some(right.insert(value)),
                None => Some(Box::new(Node::new(value))),
            };
        }

        // Para AVL, balanceia a árvore após a inserção
        self.balance()
    }

    // Busca um valor na árvore
    fn search(&self, value: i32) -> bool {
        if value == self.value {
            return true;
        } else if value < self.value {
            if let Some(left) = &self.left {
                return left.search(value);
            }
        } else {
            if let Some(right) = &self.right {
                return right.search(value);
            }
        }
        false
    }
}

// Função que calcula a altura de um nó
fn height(node: &Option<Box<Node>>) -> i32 {
    match node {
        Some(n) => n.height,
        None => 0,
    }
}

// Definição da árvore
#[derive(Debug)]
struct Tree {
    root: Option<Box<Node>>,
}

impl Tree {
    // Cria uma nova árvore vazia
    fn new() -> Self {
        Tree { root: None }
    }

    // Insere um valor na árvore
    fn insert(&mut self, value: i32) {
        match self.root.take() {
            Some(root) => self.root = Some(root.insert(value)),
            None => self.root = Some(Box::new(Node::new(value))),
        }
    }

    // Busca um valor na árvore
    fn search(&self, value: i32) -> bool {
        match &self.root {
            Some(root) => root.search(value),
            None => false,
        }
    }
}

fn main() {
    let mut tree = Tree::new();

    // Inserção de valores
    tree.insert(10);
    tree.insert(20);
    tree.insert(30);
    tree.insert(40);
    tree.insert(50);
    tree.insert(25);

    // Busca de valores
    println!("Busca por 20: {}", tree.search(20)); // true
    println!("Busca por 15: {}", tree.search(15)); // false

    // Exibe a árvore
    println!("{:#?}", tree);
}
