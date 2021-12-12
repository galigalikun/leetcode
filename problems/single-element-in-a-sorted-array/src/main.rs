fn main() {
    assert_eq!(
        Solution::single_non_duplicate(vec![1, 1, 2, 3, 3, 4, 4, 8, 8]),
        2
    );
    assert_eq!(
        Solution::single_non_duplicate(vec![3, 3, 7, 7, 10, 11, 11]),
        10
    );
    assert_eq!(Solution::single_non_duplicate(vec![1]), 1);
}

struct Solution {}
impl Solution {
    pub fn single_non_duplicate(nums: Vec<i32>) -> i32 {
        let mut prev = nums[0];
        for i in 1..nums.len() {
            if prev != nums[i] && i % 2 == 1 {
                return prev;
            }

            prev = nums[i];
        }
        return prev;
    }
}
