fn partition(arr: &mut [i32], lo: i32, hi: i32) -> i32 {
    let pivot = arr[hi as usize];

    let mut idx = lo - 1;

    for i in lo..hi {
        if arr[i as usize] <= pivot {
            idx += 1;
            arr.swap(idx as usize, i as usize);
        }
    }

    idx += 1;
    arr.swap(idx as usize, hi as usize);

    idx
}

fn qs(arr: &mut Vec<i32>, lo: i32, hi: i32) {
    if lo >= hi {
        return;
    }
    let p = partition(arr, lo, hi);
    qs(arr, lo, p - 1);
    qs(arr, p + 1, hi);
}

fn main() {
    let mut arr = vec![10, 7, 8, 9, 1, 5];
    let length = arr.len() as i32;

    qs(&mut arr, 0, length - 1);
}
