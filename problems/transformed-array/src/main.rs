fn main() {
    assert_eq!(
        Solution::construct_transformed_array(vec![3, -2, 1, 1]),
        vec![1, 1, 1, 3]
    );
    assert_eq!(
        Solution::construct_transformed_array(vec![-1, 4, -1]),
        vec![-1, -1, 4]
    );
}

struct Solution;
impl Solution {
    pub fn construct_transformed_array(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut result = vec![0; n];
        for i in 0..n {
            let left = if i == 0 { 1 } else { nums[i - 1] };
            let right = if i == n - 1 { 1 } else { nums[i + 1] };
            result[i] = left * right;
        }
        result
    }
}
