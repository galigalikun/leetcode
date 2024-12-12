fn main() {
    assert_eq!(Solution::min_moves(vec![1, 2, 4, 3], 4), 1);
    assert_eq!(Solution::min_moves(vec![1, 2, 2, 1], 2), 2);
    assert_eq!(Solution::min_moves(vec![1, 2, 1, 2], 2), 0);
}

struct Solution;
impl Solution {
    pub fn min_moves(nums: Vec<i32>, limit: i32) -> i32 {
        let mut diff = vec![0 as i32; 2 * limit as usize + 2];
        let n = nums.len();
        for i in 0..n / 2 {
            let a = nums[i];
            let b = nums[n - i - 1];
            diff[2] += 2;
            diff[2 * limit as usize + 1] -= 2;
            diff[1 + std::cmp::min(a, b) as usize] -= 1;
            diff[limit as usize + std::cmp::max(a, b) as usize + 1] += 1;
            diff[(a + b) as usize] -= 1;
            diff[(a + b) as usize + 1] += 1;
        }
        let mut sum = 0;
        let mut res = n as i32;
        for i in 2..2 * limit + 1 {
            sum += diff[i as usize];
            res = std::cmp::min(res, sum);
        }
        return res as i32;
    }
}
