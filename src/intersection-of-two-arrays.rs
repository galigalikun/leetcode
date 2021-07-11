fn main() {
    assert_eq!(
        Solution::intersection(vec![1, 2, 2, 1], vec![2, 2]),
        vec![2]
    );

    assert_eq!(
        Solution::intersection(vec![4, 9, 5], vec![9, 4, 9, 8, 4]),
        vec![9, 4]
    );
}

pub struct Solution {}
use std::collections::HashSet;
impl Solution {
    pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut result = HashSet::new();
        for n in nums1 {
            if nums2.iter().find(|&&x| x == n) != None {
                result.insert(n);
            }
        }
        return result.into_iter().collect::<Vec<_>>();
    }
}
