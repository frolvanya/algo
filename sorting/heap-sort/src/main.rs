use rand::prelude::*;

fn heapify(array: &mut [i32], heap_size: usize, index: usize) {
    let mut largest = index;
    let left = 2 * index + 1;
    let right = 2 * index + 2;

    if left < heap_size && array[left] > array[largest] {
        largest = left;
    }

    if right < heap_size && array[right] > array[largest] {
        largest = right;
    }

    if largest != index {
        array.swap(index, largest);
        heapify(array, heap_size, largest);
    }
}

fn heap_sort(arr: &mut [i32]) {
    let n = arr.len();

    for i in (0..n / 2).rev() {
        heapify(arr, n, i);
    }

    for i in (1..n).rev() {
        arr.swap(0, i);
        heapify(arr, i, 0);
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let mut arr = Vec::new();
    for _ in 0..100000000 {
        arr.push(rng.gen());
    }

    heap_sort(&mut arr);
}
