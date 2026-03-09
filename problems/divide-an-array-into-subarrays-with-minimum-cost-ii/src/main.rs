fn main() {
    assert_eq!(Solution::minimum_cost(vec![1, 3, 2, 6, 4, 2], 3, 3), 5);
    assert_eq!(Solution::minimum_cost(vec![10, 1, 2, 2, 2, 1], 4, 3), 15);
    assert_eq!(Solution::minimum_cost(vec![10, 8, 18, 9], 3, 1), 36);
}

struct Solution;
impl Solution {
    pub fn minimum_cost(nums: Vec<i32>, k: i32, dist: i32) -> i64 {
        let mut nums = nums;
        nums.sort_unstable();
        let mut ans = 0;
        for i in (0..nums.len()).step_by(k as usize) {
            ans += nums[i] as i64;
        }
        ans * dist as i64
    }
}
