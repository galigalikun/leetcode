fn main() {
    assert_eq!(Solution::max_length(vec!["un".to_string(),"iq".to_string(),"ue".to_string()]), 4);
    assert_eq!(Solution::max_length(vec!["cha".to_string(),"r".to_string(),"act".to_string(),"ers".to_string()]), 6);
    assert_eq!(Solution::max_length(vec!["abcdefghijklmnopqrstuvwxyz".to_string()]), 26);
}

struct Solution;
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        let mut max = 0;
        let mut dp = vec![0; arr.len()];
        for i in 0..arr.len() {
            let mut mask = 0;
            for c in arr[i].chars() {
                let bit = 1 << (c as u8 - 'a' as u8);
                if mask & bit != 0 {
                    mask = 0;
                    break;
                }
                mask |= bit;
            }
            dp[i] = mask;
        }
        println!("debug: {:?}", dp);
        for i in 0..arr.len() {
            if dp[i] == 0 {
                continue;
            }
            for j in i..arr.len() {
                if dp[j] == 0 {
                    continue;
                }
                if dp[i] & dp[j] == 0 {
                    max = max.max(arr[i].len() + arr[j].len());
                }
            }
        }
        return max as i32;
    }
}
