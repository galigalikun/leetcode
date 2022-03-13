fn main() {
    assert_eq!(
        Solution::find_max_form(
            vec![
                "10".to_string(),
                "0001".to_string(),
                "111001".to_string(),
                "1".to_string(),
                "0".to_string()
            ],
            5,
            3
        ),
        4
    );
    assert_eq!(
        Solution::find_max_form(
            vec!["10".to_string(), "0".to_string(), "1".to_string()],
            1,
            1
        ),
        2
    );
}

struct Solution {}
// https://dev.to/seanpgallivan/solution-ones-and-zeroes-2emf
impl Solution {
    pub fn find_max_form(strs: Vec<String>, m: i32, n: i32) -> i32 {
        let mut dp = vec![vec![0; n as usize + 1]; m as usize + 1];
        for s in strs {
            let mut zeros = 0;
            let mut ones = 0;
            for c in s.chars() {
                if c == '0' {
                    zeros += 1;
                } else {
                    ones += 1;
                }
            }
            for j in (zeros..=m as usize).rev() {
                for k in (ones..=n as usize).rev() {
                    dp[j][k] = std::cmp::max(dp[j][k], dp[j - zeros][k - ones] + 1);
                }
            }
        }
        return dp[m as usize][n as usize];
    }
}
