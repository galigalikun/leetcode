fn main() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    assert_eq!(Solution::can_jump(vec![0]), true);
    assert_eq!(Solution::can_jump(vec![1, 2]), true);
    assert_eq!(Solution::can_jump(vec![2, 0, 0]), true);
}

struct Solution {}
// https://lenchen.medium.com/leetcode-55-jump-game-d95b482642
impl Solution {
    pub fn can_jump(nums: Vec<i32>) -> bool {
        let n = nums.len();
        if n <= 1 {
            return true;
        }

        let mut last_point = n - 1;
        for i in (0..n).rev() {
            if i + nums[i] as usize >= last_point {
                last_point = i;
            }
        }

        if last_point == 0 {
            return true;
        }
        return false;
    }
}
