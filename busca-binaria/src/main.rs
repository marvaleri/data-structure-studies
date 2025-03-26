fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = arr.len(); // high é exclusivo

    while low < high {
        let mid = low + (high - low) / 2;
        if arr[mid] == target {
            return Some(mid);
        } else if arr[mid] < target {
            low = mid + 1;
        } else {
            high = mid;
        }
    }
    None
}

fn main() {
    let arr = [1, 3, 5, 7, 9, 11, 13];
    let target = 7;
    match binary_search(&arr, target) {
        Some(index) => println!("Elemento {} encontrado no índice {}.", target, index),
        None => println!("Elemento {} não encontrado.", target),
    }
}
