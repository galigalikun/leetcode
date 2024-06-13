fn main() {
    assert_eq!(Solution::running_sum(vec![1, 2, 3, 4]), vec![1, 3, 6, 10]);
    assert_eq!(
        Solution::running_sum(vec![1, 1, 1, 1, 1]),
        vec![1, 2, 3, 4, 5]
    );
    assert_eq!(
        Solution::running_sum(vec![3, 1, 2, 10, 1]),
        vec![3, 4, 6, 16, 17]
    );
}

struct Solution;
impl Solution {
    pub fn running_sum(nums: Vec<i32>) -> Vec<i32> {
        if nums.len() > 0 {
            let mut result = vec![nums[0]];
            for i in 1..nums.len() {
                result.push(result[i - 1] + nums[i]);
            }
            return result;
        }
        return vec![];
    }
}
