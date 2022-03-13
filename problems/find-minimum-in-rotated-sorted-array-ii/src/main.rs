fn main() {
    assert_eq!(Solution::find_min(vec![1, 3, 5]), 1);
    assert_eq!(Solution::find_min(vec![2, 2, 2, 0, 1]), 0);
}

struct Solution {}
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if let Some(m) = nums.into_iter().min() {
            return m;
        }
        return 0;
    }
}
