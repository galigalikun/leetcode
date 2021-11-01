fn main() {
    assert_eq!(Solution::reverse_pairs(vec![1, 3, 2, 3, 1]), 2);
    assert_eq!(Solution::reverse_pairs(vec![2, 4, 3, 5, 1]), 3);
    assert_eq!(Solution::reverse_pairs(vec![-5, -5]), 1);
}

pub struct Solution {}
impl Solution {
    pub fn reverse_pairs(nums: Vec<i32>) -> i32 {
        let mut result = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                if nums[i] > 2 * nums[j] {
                    result += 1;
                }
            }
        }
        return result;
    }
}
