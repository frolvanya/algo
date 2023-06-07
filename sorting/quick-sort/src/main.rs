use rand::prelude::*;

fn partition(arr: &mut [i32], low: usize, high: usize) -> usize {
    let pivot = low;
    let mut i = low + 1;

    for j in low + 1..high + 1 {
        if arr[j] < arr[pivot] {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i - 1, pivot);

    i - 1
}

fn partition_r(arr: &mut [i32], low: usize, high: usize) -> usize {
    let r = rand::thread_rng().gen_range(low..high);
    arr.swap(low, r);
    partition(arr, low, high)
}

fn quick_sort(arr: &mut Vec<i32>, low: usize, high: usize) {
    if low < high {
        let p = partition_r(arr, low, high);

        quick_sort(arr, low, p);
        quick_sort(arr, p + 1, high);
    }
}

fn main() {
    let mut arr = vec![5, 1, 1, 2, 0, 0];
    let length = arr.len();

    quick_sort(&mut arr, 0, length - 1);
    println!("{:?}", arr);
}
