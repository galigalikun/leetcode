fn main() {
    assert_eq!(Solution::predict_the_winner(vec![1, 5, 2]), false);
    assert_eq!(Solution::predict_the_winner(vec![1, 5, 233, 7]), true);
}

struct Solution {}
impl Solution {
    pub fn predict_the_winner(nums: Vec<i32>) -> bool {
        let n = nums.len();
        let mut cum_sum = vec![0; n];
        cum_sum[0] = nums[0];
        for i in 1..n {
            cum_sum[i] = cum_sum[i - 1] + nums[i];
        }
        let mut dp = vec![vec![0; n]; n];

        for i in 0..n {
            for j in 0..n - i {
                let start = j;
                let end = i + j;
                if start == end {
                    dp[j][j + i] = nums[j];
                } else if start + 1 == end {
                    dp[j][j + i] = std::cmp::max(nums[start], nums[end]);
                } else {
                    let choice_1 = nums[start]
                        + (cum_sum[end] - cum_sum[start + 1] + nums[start + 1]
                            - dp[start + 1][end]);
                    let choice_2 = nums[end]
                        + (cum_sum[end - 1] - cum_sum[start] + nums[start] - dp[start][end - 1]);
                    dp[j][j + i] = std::cmp::max(choice_1, choice_2);
                }
            }
        }

        println!("debug {:?}", dp);
        return dp[0][n - 1] >= cum_sum[n - 1] - dp[0][n - 1];
    }
}
