fn main() {
    let n1 = &mut vec![1, 1, 1, 2, 2, 3];
    assert_eq!(Solution::remove_duplicates(n1), 5);
    assert_eq!(n1, &mut vec![1, 1, 2, 2, 3]);

    let n2 = &mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    assert_eq!(Solution::remove_duplicates(n2), 7);
    assert_eq!(n2, &mut vec![0, 0, 1, 1, 2, 3, 3]);
}

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        let mut duplicate = HashMap::new();
        let mut key = 0;
        for i in 0..nums.len() {
            if let Some(n) = duplicate.get_mut(&nums[i - key]) {
                *n += 1;
                if *n >= 3 {
                    nums.remove(i - key);
                    key += 1;
                }
            } else {
                duplicate.insert(nums[i - key], 1);
            }
        }

        return nums.len() as i32;
    }
}
