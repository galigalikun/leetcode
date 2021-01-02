fn main() {
    assert_eq!(Solution::can_jump(vec![2, 3, 1, 1, 4]), true);
    assert_eq!(Solution::can_jump(vec![3, 2, 1, 0, 4]), false);
    assert_eq!(Solution::can_jump(vec![0]), true);
    assert_eq!(Solution::can_jump(vec![1, 2]), true);
    assert_eq!(Solution::can_jump(vec![2, 0, 0]), true);
}

pub struct Solution {}
impl Solution {
    fn jump(nums: Vec<i32>, start: usize, step: usize, max: usize) -> bool {
        if (step + 1) >= max {
            return true;
        }
        for i in start..=step {
            let n = nums[i] as usize;
            if (start + n + 1) >= max {
                return true;
            }
            if Solution::jump(nums.clone(), i + 1, i + n, max) {
                return true;
            }
        }
        return false;
    }
    pub fn can_jump(nums: Vec<i32>) -> bool {
        return Solution::jump(nums.clone(), 1, nums[0] as usize, nums.len());
    }
}
