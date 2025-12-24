fn main() {
    assert_eq!(Solution::max_distinct_elements(vec![1,2,2,3,3,4], 2), 6);
    assert_eq!(Solution::max_distinct_elements(vec![4,4,4,4], 1), 3);
}

struct Solution;
impl Solution {
    pub fn max_distinct_elements(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashSet;
        let mut num_set: HashSet<i32> = HashSet::new();
        for &num in &nums {
            num_set.insert(num);
        }
        let distinct_count = num_set.len() as i32;
        if distinct_count >= k {
            return distinct_count;
        }
        return 0;
    }
}
