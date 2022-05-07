fn main() {
    assert_eq!(Solution::min_stickers(vec!["with".to_string(),"example".to_string(),"science".to_string()], "thehat".to_string()), 3);
    assert_eq!(Solution::min_stickers(vec!["notice".to_string(),"possible".to_string()], "basicbasic".to_string()), -1);
}

struct Solution{}
impl Solution {
    pub fn min_stickers(stickers: Vec<String>, target: String) -> i32 {
        let mut dp = vec![vec![0; target.len() + 1]; stickers.len() + 1];
        for i in 1..=stickers.len() {
            for j in 1..=target.len() {
                if stickers[i - 1].chars().nth(j - 1) == target.chars().nth(j - 1) {
                    dp[i][j] = dp[i - 1][j - 1] + 1;
                } else {
                    dp[i][j] = std::cmp::max(dp[i - 1][j], dp[i][j - 1]);
                }
            }
        }
        println!("debug: {:?}", dp);
        if dp[stickers.len()][target.len()] == target.len() {
            return stickers.len() as i32;
        }
        return -1;
    }
}
