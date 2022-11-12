fn main() {
    assert_eq!(
        Solution::advantage_count(vec![2, 7, 11, 15], vec![1, 10, 4, 11]),
        vec![2, 11, 7, 15]
    );
    assert_eq!(
        Solution::advantage_count(vec![12, 24, 8, 32], vec![13, 25, 32, 11]),
        vec![24, 32, 8, 12]
    );
}

struct Solution;
impl Solution {
    pub fn advantage_count(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![];
        for i in 0..nums1.len() {
            if nums1[i] > nums2[i] {
                ans.push(nums1[i]);
            }
        }
        return ans;
    }
}
