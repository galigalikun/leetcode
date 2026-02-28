fn main() {
    assert_eq!(
        Solution::min_bitwise_array(vec![2, 3, 5, 7]),
        vec![-1, 1, 4, 3]
    );
    assert_eq!(
        Solution::min_bitwise_array(vec![11, 13, 31]),
        vec![9, 12, 15]
    );
}

struct Solution;
impl Solution {
    pub fn min_bitwise_array(nums: Vec<i32>) -> Vec<i32> {
        let mut ans = vec![0; nums.len()];
        for i in 0..nums.len() {
            ans[i] = nums[i] & nums[(i + 1) % nums.len()];
        }
        ans
    }
}
