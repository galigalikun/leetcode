fn main() {
    assert_eq!(
        Solution::num_of_strings(
            vec![
                "a".to_string(),
                "abc".to_string(),
                "bc".to_string(),
                "d".to_string()
            ],
            "abc".to_string()
        ),
        3
    );
    assert_eq!(
        Solution::num_of_strings(
            vec!["a".to_string(), "b".to_string(), "c".to_string()],
            "aaaaabbbbb".to_string()
        ),
        2
    );
    assert_eq!(
        Solution::num_of_strings(
            vec!["a".to_string(), "a".to_string(), "a".to_string()],
            "ab".to_string()
        ),
        3
    );
}

struct Solution;
impl Solution {
    pub fn num_of_strings(patterns: Vec<String>, word: String) -> i32 {
        return patterns.iter().filter(|p| word.contains(*p)).count() as i32;
    }
}
