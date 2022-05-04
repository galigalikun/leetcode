fn main() {
    assert_eq!(Solution::max_sum_of_three_subarrays(vec![1,2,1,2,6,7,5,1], 2), vec![0,3,5]);
    assert_eq!(Solution::max_sum_of_three_subarrays(vec![1,2,1,2,1,2,1,2,1], 2), vec![0,2,4]);
}

struct Solution{}
impl Solution {
    pub fn max_sum_of_three_subarrays(nums: Vec<i32>, k: i32) -> Vec<i32> {
        return vec![];
    }
}
