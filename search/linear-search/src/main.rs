fn search(arr: Vec<i32>, target: i32) -> bool {
    for i in 0..arr.len() {
        if arr[i] == target {
            return true;
        }
    }

    false
}

fn main() {
    let arr = vec![1, 2, 3, 4, 5];

    assert!(search(arr.clone(), 3));
    assert!(!search(arr.clone(), 6));
}
