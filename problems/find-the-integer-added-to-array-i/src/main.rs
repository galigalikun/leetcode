fn main() {
    assert_eq!(Solution::added_integer(vec![10], vec![5]), -5);
    assert_eq!(
        Solution::added_integer(vec![1, 1, 1, 1], vec![1, 1, 1, 1]),
        0
    );
    assert_eq!(Solution::added_integer(vec![2, 6, 4], vec![9, 7, 5]), 3);
}

struct Solution;
impl Solution {
    pub fn added_integer(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        return -nums1.iter().sum::<i32>() + nums2.iter().sum::<i32>();
    }
}
