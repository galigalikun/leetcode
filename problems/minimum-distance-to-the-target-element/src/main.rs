fn main() {
    assert_eq!(Solution::get_min_distance(vec![1, 2, 3, 4, 5], 5, 3), 1);
    assert_eq!(Solution::get_min_distance(vec![1], 1, 0), 0);
    assert_eq!(
        Solution::get_min_distance(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 1, 0),
        0
    );
}

struct Solution;
impl Solution {
    pub fn get_min_distance(nums: Vec<i32>, target: i32, start: i32) -> i32 {
        return nums
            .iter()
            .enumerate()
            .filter(|&(_, &num)| num == target)
            .map(|(i, _)| (i as i32 - start).abs())
            .min()
            .unwrap_or(-1);
    }
}
