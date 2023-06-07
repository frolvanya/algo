fn counting_sort(arr: &mut Vec<i32>) {
    let mut min = i32::MAX;
    let mut max = i32::MIN;

    for element in arr.clone() {
        if element < min {
            min = element;
        }

        if element > max {
            max = element;
        }
    }

    let mut count_array = vec![0; max as usize - min as usize + 1];
    let mut output_array = vec![0; arr.len()];

    for element in arr.clone() {
        count_array[(element - min) as usize] += 1;
    }

    for i in 1..count_array.len() {
        count_array[i] += count_array[i - 1]
    }

    for i in (0..arr.len()).rev() {
        output_array[count_array[(arr[i] - min) as usize] - 1] = arr[i];
        count_array[(arr[i] - min) as usize] -= 1;
    }

    arr[..output_array.len()].copy_from_slice(&output_array[..]);
}

fn main() {
    let mut arr = vec![3, 6, 2, 1, 7, 10, 9, 5, 8, 4];

    counting_sort(&mut arr);

    println!("{arr:?}");
}
