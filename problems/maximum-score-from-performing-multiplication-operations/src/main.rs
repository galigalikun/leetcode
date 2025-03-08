fn main() {
    assert_eq!(Solution::maximum_score(vec![1, 2, 3], vec![3, 2, 1]), 14);
    assert_eq!(Solution::maximum_score(vec![-5,-3,-3,-2,7,1], vec![-10,-5,3,4,6]), 102);
}

struct Solution;
impl Solution {
    pub fn maximum_score(nums: Vec<i32>, multipliers: Vec<i32>) -> i32 {
        let n = nums.len();
        let m = multipliers.len();
        let mut multipliers = multipliers;
        for i in 1..m {
            for j in 0..=i {
                let left = if j > 0 { multipliers[j - 1] * nums[j - 1] } else { 0 };
                let right = if j < i { multipliers[j] * nums[n - i + j] } else { 0 };
                multipliers[j] = left.max(right);
                if j > 0 {
                    multipliers[j] += multipliers[j - 1];
                }
            }
        }
        multipliers[m - 1] += multipliers[m - 2];
        multipliers.into_iter().max().unwrap_or(0)
    }
}
