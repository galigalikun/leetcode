fn main() {
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 5), 2);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 2), 1);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 7), 4);
    assert_eq!(Solution::search_insert(vec![1, 3, 5, 6], 0), 0);
    assert_eq!(Solution::search_insert(vec![1], 0), 0);
}

pub struct Solution {}
impl Solution {
    pub fn search_insert(nums: Vec<i32>, target: i32) -> i32 {
        for i in 0..nums.len() {
            if nums[i] >= target {
                return i as i32;
            }
        }

        return nums.len() as i32;
    }
}
