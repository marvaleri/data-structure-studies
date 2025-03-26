fn partition(arr: &mut [i32]) -> usize {
    let len = arr.len();
    // Escolhe o último elemento como pivô
    let pivot = arr[len - 1];
    let mut i = 0;

    // Move todos os elementos menores que o pivô para o início do array
    for j in 0..(len - 1) {
        if arr[j] < pivot {
            arr.swap(i, j);
            i += 1;
        }
    }
    // Coloca o pivô na posição correta
    arr.swap(i, len - 1);
    i
}

fn quick_sort(arr: &mut [i32]) {
    // Se o vetor tiver 0 ou 1 elementos, está ordenado
    if arr.len() <= 1 {
        return;
    }
    // Particiona o array e obtém o índice do pivô
    let pivot_index = partition(arr);

    // Divide o slice em duas partes: antes e depois do pivô
    let (left, right) = arr.split_at_mut(pivot_index);

    // Ordena recursivamente a sublista da esquerda
    quick_sort(left);

    // Ordena recursivamente a sublista da direita (excluindo o pivô)
    quick_sort(&mut right[1..]);
}

fn main() {
    let mut array = [10, 7, 8, 9, 1, 5];
    println!("Array original: {:?}", array);

    quick_sort(&mut array);
    println!("Quick Sort: {:?}", array);
}
