fn main() {
    assert_eq!(
        Solution::get_concatenation(vec![1, 2, 1]),
        vec![1, 2, 1, 1, 2, 1]
    );
    assert_eq!(
        Solution::get_concatenation(vec![1, 3, 2, 1]),
        vec![1, 3, 2, 1, 1, 3, 2, 1]
    );
}

struct Solution;
impl Solution {
    pub fn get_concatenation(nums: Vec<i32>) -> Vec<i32> {
        return nums.iter().cloned().chain(nums.iter().cloned()).collect();
    }
}
