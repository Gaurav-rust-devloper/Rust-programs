fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot_index = low + (high - low) / 2;
    let pivot_value = arr[pivot_index];
    arr.swap(pivot_index, high);
    let mut i = low;
    for j in low..high {
        if arr[j] < pivot_value {
            arr.swap(i, j);
            i += 1;
        }
    }
    arr.swap(i, high);
    i 
}

fn quicksort(arr: &mut [i32], low: usize, high: usize) {
    if low < high {
        let pivot_index = partition(arr, low, high);
        quicksort(arr, low, pivot_index);
        quicksort(arr, pivot_index + 1, high);
    }
}

fn kth_smallest_element(arr: &mut [i32], k: usize) -> i32 {
    quicksort(arr, 0, arr.len() - 1);
    arr[k - 1]
}

fn main() {
    let mut arr = [7, 10, 4, 3, 20, 15];
    let k = 3;
    let kth_smallest = kth_smallest_element(&mut arr, k);
    println!("The {}th smallest element is: {}", k, kth_smallest);
}
