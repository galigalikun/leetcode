fn main() {
    assert_eq!(Solution::num_subarrays_with_sum(vec![1,0,1,0,1], 2), 4);
    assert_eq!(Solution::num_subarrays_with_sum(vec![0,0,0,0,0], 0), 15);
}

struct Solution;
impl Solution {
    pub fn num_subarrays_with_sum(nums: Vec<i32>, goal: i32) -> i32 {
        return 0;
    }
}
