fn search(arr: Vec<i32>, target: i32) -> bool {
    let mut low = 0i32;
    let mut high = arr.len() as i32 - 1;

    while low <= high {
        let mid = low + (high - low) / 2;

        if arr[mid as usize] == target {
            return true;
        } else if arr[mid as usize] > target {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }

    false
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5];

    assert!(search(arr.clone(), 3));
    assert!(!search(arr.clone(), 6));
}
