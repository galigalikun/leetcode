fn main() {
    assert_eq!(Solution::pivot_index(vec![1, 7, 3, 6, 5, 6]), 3);
    assert_eq!(Solution::pivot_index(vec![1, 2, 3]), -1);
    assert_eq!(Solution::pivot_index(vec![-1, -1, 0, 1, 1, 0]), 5);
}

struct Solution {}
impl Solution {
    pub fn pivot_index(nums: Vec<i32>) -> i32 {
        for i in 0..nums.len() {
            let left_sum = nums[0..i].iter().sum::<i32>();
            let right_sum = nums[i + 1..].iter().sum::<i32>();
            if left_sum == right_sum {
                return i as i32;
            }
        }
        return -1;
    }
}
