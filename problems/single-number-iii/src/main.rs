fn main() {
    assert_eq!(Solution::single_number(vec![1, 2, 1, 3, 2, 5]), vec![3, 5]);
    assert_eq!(Solution::single_number(vec![-1, 0]), vec![-1, 0]);
    assert_eq!(Solution::single_number(vec![0, 1]), vec![0, 1]);
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        let mut map = HashMap::new();
        for n in nums {
            if let Some(m) = map.get_mut(&n) {
                if m == &1 {
                    result.retain(|&x| x != n);
                }
                *m += 1;
            } else {
                map.insert(n, 1);
                result.push(n);
            }
        }
        return result;
    }
}
