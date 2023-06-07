fn merge(arr: &mut [i32], left: usize, middle: usize, right: usize) {
    let n1 = middle - left + 1;
    let n2 = right - middle;

    let mut left_arr: Vec<i32> = Vec::with_capacity(n1);
    let mut right_arr: Vec<i32> = Vec::with_capacity(n2);

    for i in 0..n1 {
        left_arr.push(arr[left + i]);
    }

    for i in 0..n2 {
        right_arr.push(arr[middle + 1 + i]);
    }

    let mut i = 0;
    let mut j = 0;
    let mut k = left;

    while i < n1 && j < n2 {
        if left_arr[i] <= right_arr[j] {
            arr[k] = left_arr[i];
            i += 1;
        } else {
            arr[k] = right_arr[j];
            j += 1;
        }
        k += 1;
    }

    while i < n1 {
        arr[k] = left_arr[i];
        i += 1;
        k += 1;
    }

    while j < n2 {
        arr[k] = right_arr[j];
        j += 1;
        k += 1;
    }
}

fn merge_sort(arr: &mut [i32], left: usize, right: usize) {
    if left < right {
        let middle = left + (right - left) / 2;
        merge_sort(arr, left, middle);
        merge_sort(arr, middle + 1, right);

        merge(arr, left, middle, right);
    }
}

fn main() {
    let mut arr = vec![3, 6, 2, 1, 7, 10, 9, 5, 8, 4];
    let length = arr.len();

    merge_sort(&mut arr, 0, length - 1);
    println!("{arr:?}");
}
