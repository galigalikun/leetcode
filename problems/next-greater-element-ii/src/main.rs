fn main() {
    assert_eq!(
        Solution::next_greater_elements(vec![1, 2, 1]),
        vec![2, -1, 2]
    );
    assert_eq!(
        Solution::next_greater_elements(vec![1, 2, 3, 4, 3]),
        vec![2, 3, 4, -1, 4]
    );
    assert_eq!(
        Solution::next_greater_elements(vec![5, 4, 3, 2, 1]),
        vec![-1, 5, 5, 5, 5]
    );
}

struct Solution {}
impl Solution {
    pub fn next_greater_elements(nums: Vec<i32>) -> Vec<i32> {
        let max = nums.iter().fold(0, |m, v| *v.max(&m));
        let mut result = vec![-1; nums.len()];
        for i in 0..nums.len() {
            result[i] = if nums[i] == max { -1 } else { nums[i] + 1 };
        }
        return result;
    }
}
