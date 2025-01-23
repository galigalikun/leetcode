fn main() {
    assert_eq!(Solution::minimum_time_required(vec![3, 2, 3], 3), 3);
    assert_eq!(Solution::minimum_time_required(vec![1, 2, 4, 7, 8], 2), 11);
}

struct Solution;
impl Solution {
    pub fn minimum_time_required(jobs: Vec<i32>, k: i32) -> i32 {
        let n = jobs.len();
        let k = k as usize;
        let mut dp = vec![vec![0; 1 << n]; k];
        let mut sum = vec![0; 1 << n];
        for i in 0..1 << n {
            for j in 0..n {
                if i & 1 << j != 0 {
                    sum[i] += jobs[j];
                }
            }
        }
        for i in 0..1 << n {
            dp[0][i] = sum[i];
        }
        for i in 1..k {
            for j in 0..1 << n {
                let mut minn = std::i32::MAX;
                for x in 0..1 << n {
                    if x & j == x {
                        minn = minn.min(dp[i - 1][j ^ x].max(sum[x]));
                    }
                }
                dp[i][j] = minn;
            }
        }
        dp[k - 1][(1 << n) - 1]
    }
}
