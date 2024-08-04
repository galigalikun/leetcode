fn main() {
    assert_eq!(Solution::get_length_of_optimal_compression("aaabcccd".to_string(), 2), 4);
    assert_eq!(Solution::get_length_of_optimal_compression("aabbaa".to_string(), 2), 2);
    assert_eq!(Solution::get_length_of_optimal_compression("aaaaaaaaaaa".to_string(), 0), 3);

}

struct Solution;
impl Solution {
    pub fn get_length_of_optimal_compression(s: String, k: i32) -> i32 {
        let s = s.as_bytes();
        let n = s.len();
        let k = k as usize;
        let mut dp = vec![vec![vec![n + 1; n + 1]; n + 1]; k + 1];
        dp[0][0][0] = 0;
        for i in 1..=n {
            for j in 0..=i {
                for l in 0..=k {
                    if l > j {
                        continue;
                    }
                    let mut cnt = 0;
                    let mut del = 0;
                    for m in (0..j).rev() {
                        if s[i - 1] == s[m] {
                            cnt += 1;
                        } else {
                            del += 1;
                        }
                        if l >= del {
                            dp[l][i][cnt] = dp[l - del][m][cnt].min(dp[l][i][cnt]);
                        }
                    }
                    if l > 0 {
                        dp[l][i][1] = dp[l - 1][i][1].min(dp[l][i][1]);
                    }
                }
            }
        }
        let mut ans = n as i32;
        for i in 0..=k {
            for j in 0..=n {
                for l in 0..=n {
                    if dp[i][j][l] <= n {
                        ans = ans.min(dp[i][j][l] as i32 + (if l > 1 { 1 } else { 0 }));
                    }
                }
            }
        }
        ans
    }
}
