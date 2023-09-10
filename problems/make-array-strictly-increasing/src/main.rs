fn main() {
    assert_eq!(
        Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 3, 2, 4]),
        1
    );
    assert_eq!(
        Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![4, 3, 1]),
        2
    );
    assert_eq!(
        Solution::make_array_increasing(vec![1, 5, 3, 6, 7], vec![1, 6, 3, 3]),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn make_array_increasing(arr1: Vec<i32>, arr2: Vec<i32>) -> i32 {
        let mut arr2 = arr2;
        arr2.sort();
        let mut dp = vec![vec![0; arr2.len() + 1]; arr1.len() + 1];
        for i in 0..arr2.len() + 1 {
            dp[0][i] = 1;
        }
        for i in 1..arr1.len() + 1 {
            for j in 1..arr2.len() + 1 {
                if arr1[i - 1] > arr2[j - 1] {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                }
                if i > 1 && arr1[i - 1] > arr1[i - 2] {
                    dp[i][j] = std::cmp::min(dp[i][j], dp[i - 1][j]);
                }
            }
        }
        let mut ans = std::i32::MAX;
        for i in 1..arr2.len() + 1 {
            ans = std::cmp::min(ans, dp[arr1.len()][i]);
        }
        if ans != std::i32::MAX {
            return ans;
        }
        return -1;
    }
}
