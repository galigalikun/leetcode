fn main() {
    assert_eq!(Solution::longest_subsequence(vec![1, 2, 3, 4], 1), 4);
    assert_eq!(Solution::longest_subsequence(vec![1, 3, 5, 7], 1), 1);
    assert_eq!(
        Solution::longest_subsequence(vec![1, 5, 7, 8, 5, 3, 4, 2, 1], -2),
        4
    );
}

struct Solution;
impl Solution {
    pub fn longest_subsequence(arr: Vec<i32>, difference: i32) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        let mut max = 0;
        for i in arr {
            let count = *map.entry(i - difference).or_insert(0) + 1;
            max = std::cmp::max(max, count);
        }
        return max;
    }
}
