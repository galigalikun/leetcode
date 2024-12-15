fn main() {
    assert_eq!(Solution::max_operations(vec![1, 2, 3, 4], 5), 2);
    assert_eq!(Solution::max_operations(vec![3, 1, 3, 4, 3], 6), 1);
}

struct Solution;
impl Solution {
    pub fn max_operations(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;
        let mut map = HashMap::new();
        for num in nums {
            *map.entry(num).or_insert(0) += 1;
        }
        let mut count = 0;
        for (num, freq) in &map {
            if num * 2 == k {
                count += freq / 2;
            } else {
                count += std::cmp::min(freq, map.get(&(k - num)).unwrap_or(&0));
            }
        }
        count / 2
    }
}
