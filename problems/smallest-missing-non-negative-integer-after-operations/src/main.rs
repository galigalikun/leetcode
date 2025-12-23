fn main() {
    assert_eq!(
        Solution::find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 5),
        4
    );
    assert_eq!(
        Solution::find_smallest_integer(vec![1, -10, 7, 13, 6, 8], 7),
        2
    );
    assert_eq!(
        Solution::find_smallest_integer(vec![3, 0, 3, 2, 4, 2, 1, 1, 0, 4], 5),
        10
    );
}

struct Solution;
impl Solution {
    pub fn find_smallest_integer(nums: Vec<i32>, value: i32) -> i32 {
        use std::collections::HashSet;
        let mut remainders = HashSet::new();
        for &num in &nums {
            let rem = ((num % value) + value) % value;
            remainders.insert(rem);
        }
        for i in 0..value {
            if !remainders.contains(&i) {
                return i;
            }
        }
        0
    }
}
