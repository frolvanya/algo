fn find_duplicate(nums: Vec<usize>) -> i32 {
    let mut slow = 0;
    let mut fast = 0;

    loop {
        slow = nums[slow];
        fast = nums[nums[fast]];

        if slow == fast {
            break;
        }
    }

    slow = 0;
    while slow != fast {
        slow = nums[slow];
        fast = nums[fast];
    }

    slow as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cycle_detection() {
        // Floyd's cycle detection algorithm
        // Note: Algorithm is modified to work on arrays with values in range [1, n]
        assert_eq!(find_duplicate(vec![1, 3, 4, 2, 2]), 2);
        assert_eq!(find_duplicate(vec![3, 1, 3, 4, 2]), 3);
    }
}
