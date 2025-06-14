fn main() {
    assert_eq!(Solution::minimum_xor_sum(vec![1, 2], vec![2, 3]), 2);
    assert_eq!(Solution::minimum_xor_sum(vec![1, 0, 3], vec![5, 3, 4]), 8);
}

struct Solution;
impl Solution {
    pub fn minimum_xor_sum(nums1: Vec<i32>, nums2: Vec<i32>) -> i32 {
        let n = nums1.len();
        let mut dp = vec![vec![i32::MAX; 1 << n]; n + 1];
        dp[0][0] = 0;

        for i in 0..n {
            for mask in 0..(1 << n) {
                if dp[i][mask] == i32::MAX {
                    continue;
                }
                for j in 0..n {
                    if (mask & (1 << j)) == 0 {
                        let new_mask = mask | (1 << j);
                        dp[i + 1][new_mask] =
                            dp[i + 1][new_mask].min(dp[i][mask] + (nums1[i] ^ nums2[j]));
                    }
                }
            }
        }

        dp[n][(1 << n) - 1]
    }
}
