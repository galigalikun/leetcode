fn main() {
    assert_eq!(Solution::find_error_nums(vec![1, 2, 2, 4]), vec![2, 3]);
    assert_eq!(Solution::find_error_nums(vec![1, 1]), vec![1, 2]);
    assert_eq!(Solution::find_error_nums(vec![2, 2]), vec![2, 1]);
    assert_eq!(Solution::find_error_nums(vec![1, 3, 3]), vec![3, 2]);
    assert_eq!(Solution::find_error_nums(vec![3, 2, 2]), vec![2, 1]);
    assert_eq!(
        Solution::find_error_nums(vec![3, 2, 3, 4, 6, 5]),
        vec![3, 1]
    );
    assert_eq!(
        Solution::find_error_nums(vec![1, 5, 3, 2, 2, 7, 6, 4, 8, 9]),
        [2, 10]
    );
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_error_nums(nums: Vec<i32>) -> Vec<i32> {
        let mut nn = nums;
        nn.sort();
        let mut num = 0;
        let mut prev: Option<i32> = None;
        let mut map = HashMap::new();
        for n in nn.clone() {
            if prev == Some(n) {
                num = n;
            }
            map.insert(n as usize, 1);
            prev = Some(n);
        }
        for i in 1..=nn.len() {
            if !map.contains_key(&i) {
                return vec![num, i as i32];
            }
        }
        return vec![];
    }
}
