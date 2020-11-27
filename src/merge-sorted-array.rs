fn main() {
    let mut n1 = vec![1, 2, 3, 0, 0, 0];
    Solution::merge(&mut n1, 3, &mut vec![2, 5, 6], 3);
    assert_eq!(n1, vec![1, 2, 2, 3, 5, 6]);
}

pub struct Solution {}
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        nums1.truncate(m as usize);
        nums2.truncate(n as usize);
        nums1.append(nums2);
        nums1.sort();
    }
}
