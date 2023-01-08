fn main() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1,-2,3,-2]), 3);
    assert_eq!(Solution::max_subarray_sum_circular(vec![5,-3,5]), 10);
    assert_eq!(Solution::max_subarray_sum_circular(vec![-3,-2,-3]), -2);
}

struct Solution;
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut ans = std::i32::MIN;
        for i in 0..nums.len() {
            ans = std::cmp::max(ans, nums[i]);
            for n in i+1..nums.len() {
                ans = std::cmp::max(ans, nums[i] + nums[(i + 1) % n]);
            }
        }
        return ans;
    }
}
