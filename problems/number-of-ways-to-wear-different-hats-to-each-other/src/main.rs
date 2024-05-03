fn main() {
    assert_eq!(
        Solution::number_ways(vec![vec![3, 4], vec![4, 5], vec![5]]),
        1
    );
    // assert_eq!(Solution::number_ways(vec![vec![3, 5, 1], vec![3, 5]]), 4);
    // assert_eq!(
    //     Solution::number_ways(vec![
    //         vec![1, 2, 3, 4],
    //         vec![1, 2, 3, 4],
    //         vec![1, 2, 3, 4],
    //         vec![1, 2, 3, 4]
    //     ]),
    //     24
    // );
}

struct Solution;
impl Solution {
    pub fn number_ways(hats: Vec<Vec<i32>>) -> i32 {
        let mut dp = vec![0; 1 << 10];
        dp[0] = 1;
        let n = hats.len();
        let mut hat = vec![0; 41];
        for i in 0..n {
            for j in 0..hats[i].len() {
                hat[hats[i][j] as usize] |= 1 << i;
            }
        }
        let mod_num = 1000000007;
        for i in 1..41 {
            for j in (0..(1 << n)).rev() {
                for k in 1..11 {
                    if (hat[i] & j) != 0 {
                        dp[j] = (dp[j] + dp[j ^ (1 << (k - 1))]) % mod_num;
                    }
                }
            }
        }
        return dp[(1 << n) - 1];
    }
}
