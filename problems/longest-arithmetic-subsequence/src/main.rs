fn main() {
    assert_eq!(Solution::longest_arith_seq_length(vec![3, 6, 9, 12]), 4);
    assert_eq!(Solution::longest_arith_seq_length(vec![9, 4, 7, 2, 10]), 3);
    assert_eq!(
        Solution::longest_arith_seq_length(vec![20, 1, 15, 3, 10, 5, 8]),
        4
    );
}

struct Solution;
impl Solution {
    pub fn longest_arith_seq_length(nums: Vec<i32>) -> i32 {
        let mut dp = vec![vec![0; 20001]; nums.len()];
        let mut max = 0;
        for i in 0..nums.len() {
            for j in 0..i {
                let diff = (nums[i] - nums[j] + 10000) as usize;
                dp[i][diff] = std::cmp::max(dp[i][diff], dp[j][diff] + 1);
                max = std::cmp::max(max, dp[i][diff]);
            }
        }
        return max + 1;
    }
}
