fn main() {
    assert_eq!(Solution::single_number(vec![2, 2, 1]), 1);
    assert_eq!(Solution::single_number(vec![4, 1, 2, 1, 2]), 4);
    assert_eq!(Solution::single_number(vec![1]), 1);
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> i32 {
        let mut map = HashMap::new();
        for n in nums {
            if let Some(x) = map.get_mut(&n) {
                *x += 1;
            } else {
                map.insert(n, 1);
            }
        }
        for (k, v) in map {
            if v == 1 {
                return k;
            }
        }

        return 0;
    }
}
