fn main() {
    assert_eq!(
        Solution::find_duplicates(vec![4, 3, 2, 7, 8, 2, 3, 1]),
        vec![2, 3]
    );
    assert_eq!(Solution::find_duplicates(vec![1, 1, 2]), vec![1]);
    assert_eq!(Solution::find_duplicates(vec![1]), vec![]);
}

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut test = HashMap::new();
        let mut result = vec![];
        for n in nums {
            if let Some(x) = test.get_mut(&n) {
                if *x == 1 {
                    result.push(n);
                }
                *x += 1;
            } else {
                test.insert(n, 1);
            }
        }
        return result;
    }
}
