fn main() {
    assert_eq!(
        Solution::is_prefix_string(
            "iloveleetcode".to_string(),
            vec![
                "i".to_string(),
                "love".to_string(),
                "leetcode".to_string(),
                "apples".to_string()
            ]
        ),
        true
    );
    assert_eq!(
        Solution::is_prefix_string(
            "iloveleetcode".to_string(),
            vec![
                "apples".to_string(),
                "i".to_string(),
                "love".to_string(),
                "leetcode".to_string()
            ]
        ),
        false
    );
}

struct Solution;
impl Solution {
    pub fn is_prefix_string(s: String, words: Vec<String>) -> bool {
        let mut prefix = String::new();
        for word in words {
            prefix.push_str(&word);
            if prefix == s {
                return true;
            }
            if prefix.len() > s.len() {
                return false;
            }
        }
        false
    }
}
