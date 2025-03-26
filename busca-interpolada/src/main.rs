fn interpolation_search(arr: &[i32], target: i32) -> Option<usize> {
    if arr.is_empty() {
        return None;
    }

    let mut low = 0;
    let mut high = arr.len() - 1;

    while low <= high && target >= arr[low] && target <= arr[high] {
        // Evita divisão por zero se todos os elementos no intervalo forem iguais
        if arr[high] == arr[low] {
            return if arr[low] == target { Some(low) } else { None };
        }

        // Cálculo da posição baseado na interpolação
        let pos = low + (((target - arr[low]) as usize * (high - low))
                        / ((arr[high] - arr[low]) as usize));

        // Verifica se a posilção está dentro dos limites
        if pos >= arr.len() {
            break;
        }

        if arr[pos] == target {
            return Some(pos);
        } else if arr[pos] < target {
            low = pos + 1;
        } else {
            // Prevenir underflow
            if pos == 0 { break; }
            high = pos - 1;
        }
    }
    None
}

fn main() {
    let arr = [10, 20, 30, 40, 50, 60, 70, 80, 90];
    let target = 70;
    match interpolation_search(&arr, target) {
        Some(index) => println!("Elemento {} encontrado no índice {}.", target, index),
        None => println!("Elemento {} não encontrado.", target),
    }
}
