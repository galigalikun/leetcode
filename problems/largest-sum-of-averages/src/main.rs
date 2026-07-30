fn main() {
    assert_eq!(Solution::largest_sum_of_averages(vec![9, 1, 2, 3, 9], 3), 20.0);
    assert_eq!(
        Solution::largest_sum_of_averages(vec![1, 2, 3, 4, 5, 6, 7], 4),
        20.5
    );
}

struct Solution {}
impl Solution {
    pub fn largest_sum_of_averages(nums: Vec<i32>, k: i32) -> f64 {
        let n = nums.len();
        let groups = (k as usize).min(n);

        let mut prefix = vec![0.0; n + 1];
        for i in 0..n {
            prefix[i + 1] = prefix[i] + nums[i] as f64;
        }

        let mut dp = vec![vec![0.0; n + 1]; groups + 1];
        for i in 0..n {
            dp[1][i] = (prefix[n] - prefix[i]) / (n - i) as f64;
        }

        for g in 2..=groups {
            for i in 0..=n - g {
                let mut best: f64 = 0.0;
                for split in i + 1..=n - g + 1 {
                    let first_avg = (prefix[split] - prefix[i]) / (split - i) as f64;
                    let candidate = first_avg + dp[g - 1][split];
                    best = best.max(candidate);
                }
                dp[g][i] = best;
            }
        }

        dp[groups][0]
    }
}
