fn main() {
    assert_eq!(Solution::find_lhs(vec![1, 3, 2, 2, 5, 2, 3, 7]), 5);
    assert_eq!(Solution::find_lhs(vec![1, 2, 3, 4]), 2);
    assert_eq!(Solution::find_lhs(vec![1, 1, 1, 1]), 0);
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_lhs(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for n in nums {
            if let Some(m) = map.get_mut(&n) {
                *m += 1;
            } else {
                map.insert(n, 1);
            }
        }
        let mut vec = map.iter().collect::<Vec<_>>();
        vec.sort();
        let mut prev = vec[0];
        let mut ans = 0;
        for i in 1..vec.len() {
            if (prev.0 - vec[i].0).abs() == 1 {
                ans = std::cmp::max(ans, prev.1 + vec[i].1);
            }
            prev = vec[i];
        }

        return ans;
    }
}
