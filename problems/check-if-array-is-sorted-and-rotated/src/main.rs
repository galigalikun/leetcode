fn main() {
    assert_eq!(Solution::check(vec![3, 4, 5, 1, 2]), true);
    assert_eq!(Solution::check(vec![2, 1, 3, 4]), false);
    assert_eq!(Solution::check(vec![1, 2, 3]), true);
}

struct Solution;
impl Solution {
    pub fn check(nums: Vec<i32>) -> bool {
        for i in 0..nums.len() {
            let mut nums = nums.clone();
            nums.rotate_left(i);
            if nums.windows(nums.len()).any(|w| {
                let mut w = w.to_vec();
                w.sort();
                w == nums
            }) {
                return true;
            }
        }
        false
    }
}
