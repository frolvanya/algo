fn bubble_sort(arr: &mut Vec<i32>) {
    for i in 0..arr.len() {
        for j in 0..arr.len() - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j + 1, j);
            }
        }
    }
}

fn main() {
    let mut arr = vec![5, 4, 3, 2, 1];
    bubble_sort(&mut arr);

    assert_eq!(arr, vec![1, 2, 3, 4, 5])
}
