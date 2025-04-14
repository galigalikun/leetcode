fn main() {
    assert_eq!(
        Solution::num_different_integers("a123bc34d8ef34".to_string()),
        3
    );
    assert_eq!(
        Solution::num_different_integers("leet1234code234".to_string()),
        2
    );
    assert_eq!(Solution::num_different_integers("a1b01c001".to_string()), 1);
    assert_eq!(
        Solution::num_different_integers("167278959591294".to_string()),
        1
    );
    assert_eq!(Solution::num_different_integers("0a0".to_string()), 1);
}

struct Solution;
impl Solution {
    pub fn num_different_integers(word: String) -> i32 {
        return word
            .split(|c: char| !c.is_ascii_digit())
            .filter(|s| !s.is_empty())
            .map(|s| {
                let trimmed = s.trim_start_matches('0');
                if trimmed.is_empty() { "0" } else { trimmed }
            })
            .collect::<std::collections::HashSet<_>>()
            .len() as i32;
    }
}
