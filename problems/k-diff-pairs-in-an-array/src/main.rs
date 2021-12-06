fn main() {
    assert_eq!(Solution::find_pairs(vec![3, 1, 4, 1, 5], 2), 2);
    assert_eq!(Solution::find_pairs(vec![1, 2, 3, 4, 5], 1), 4);
    assert_eq!(Solution::find_pairs(vec![1, 3, 1, 5, 4], 0), 1);
    assert_eq!(
        Solution::find_pairs(vec![1, 2, 4, 4, 3, 3, 0, 9, 2, 3], 3),
        2
    );
    assert_eq!(Solution::find_pairs(vec![-1, -2, -3], 1), 2);
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_pairs(nums: Vec<i32>, k: i32) -> i32 {
        let mut ans = 0;
        let mut map = HashMap::new();
        for i in 0..nums.len() {
            for j in i + 1..nums.len() {
                let key = if nums[i] > nums[j] {
                    (nums[i], nums[j])
                } else {
                    (nums[j], nums[i])
                };
                if !map.contains_key(&key) {
                    if key.0 - key.1 == k {
                        ans += 1;
                    }
                    map.insert(key, 1);
                }
            }
        }
        return ans;
    }
}
