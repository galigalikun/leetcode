fn main() {
    assert_eq!(
        Solution::longest_str_chain(vec![
            "a".to_string(),
            "b".to_string(),
            "ba".to_string(),
            "bca".to_string(),
            "bda".to_string(),
            "bdca".to_string()
        ]),
        4
    );
    assert_eq!(
        Solution::longest_str_chain(vec![
            "xbc".to_string(),
            "pcxbcf".to_string(),
            "xb".to_string(),
            "cxbc".to_string(),
            "pcxbc".to_string()
        ]),
        5
    );
    assert_eq!(
        Solution::longest_str_chain(vec!["abcd".to_string(), "dbqca".to_string()]),
        1
    );
}

struct Solution;
impl Solution {
    fn is_predecessor(a: &String, b: &String) -> bool {
        if a.len() + 1 != b.len() {
            return false;
        }
        let mut i = 0;
        let mut j = 0;
        let mut diff = 0;
        while i < a.len() && j < b.len() {
            if a.chars().nth(i).unwrap() == b.chars().nth(j).unwrap() {
                i += 1;
                j += 1;
            } else {
                diff += 1;
                j += 1;
            }
        }
        return diff <= 1;
    }
    pub fn longest_str_chain(words: Vec<String>) -> i32 {
        let mut words = words;
        words.sort_by(|a, b| a.len().cmp(&b.len()));
        let mut dp = vec![1; words.len()];
        for i in 0..words.len() {
            for j in 0..i {
                if Solution::is_predecessor(&words[j], &words[i]) {
                    dp[i] = dp[i].max(dp[j] + 1);
                }
            }
        }
        return *dp.iter().max().unwrap();
    }
}
