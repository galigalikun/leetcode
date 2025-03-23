fn main() {
    assert_eq!(Solution::min_elements(vec![1, -1, 1], 3, -4), 2);
    assert_eq!(Solution::min_elements(vec![1, -10, 9, 1], 100, 0), 1);
}

struct Solution;
impl Solution {
    pub fn min_elements(nums: Vec<i32>, limit: i32, goal: i32) -> i32 {
        let sum: i64 = nums.iter().map(|&x| x as i64).sum();
        let diff = goal as i64 - sum;
        if diff == 0 {
            return 0;
        }
        let diff = diff.abs();
        if diff % limit as i64 == 0 {
            return (diff / limit as i64) as i32;
        } else {
            return (diff / limit as i64 + 1) as i32;
        }
    }
}
