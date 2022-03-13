fn main() {
    assert_eq!(
        Solution::word_break(
            "leetcode".to_string(),
            vec!["leet".to_string(), "code".to_string()]
        ),
        true
    );

    assert_eq!(
        Solution::word_break(
            "applepenapple".to_string(),
            vec!["apple".to_string(), "pen".to_string()]
        ),
        true
    );

    assert_eq!(
        Solution::word_break(
            "catsandog".to_string(),
            vec![
                "cats".to_string(),
                "dog".to_string(),
                "sand".to_string(),
                "and".to_string(),
                "cat".to_string()
            ]
        ),
        false
    );

    assert_eq!(
        Solution::word_break(
            "abcd".to_string(),
            vec![
                "a".to_string(),
                "abc".to_string(),
                "b".to_string(),
                "cd".to_string()
            ]
        ),
        true
    );
}

struct Solution {}
// https://qiita.com/KueharX/items/ac025ce072d98751e688
impl Solution {
    pub fn word_break(s: String, word_dict: Vec<String>) -> bool {
        if s.len() > 0 && word_dict.len() == 0 {
            return false;
        }

        let mut dp = vec![true];
        for i in 1..=s.len() {
            for w in &word_dict {
                if i >= w.len() && s[i - w.len()..i] == *w && dp[i - w.len()] {
                    dp.push(true);
                    break;
                }
            }
            if dp.len() <= i {
                dp.push(false);
            }
        }
        return *dp.last().unwrap();
    }
}
