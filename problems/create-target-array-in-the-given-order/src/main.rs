use std::vec;

fn main() {
    assert_eq!(
        Solution::create_target_array(vec![0, 1, 2, 3, 4], vec![0, 1, 2, 2, 1]),
        vec![0, 4, 1, 3, 2]
    );
    assert_eq!(
        Solution::create_target_array(vec![1, 2, 3, 4, 0], vec![0, 1, 2, 3, 0]),
        vec![0, 1, 2, 3, 4]
    );
    assert_eq!(Solution::create_target_array(vec![1], vec![0]), vec![1]);
}

struct Solution;
impl Solution {
    pub fn create_target_array(nums: Vec<i32>, index: Vec<i32>) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..nums.len() {
            result.insert(index[i] as usize, nums[i]);
        }
        return result;
    }
}
