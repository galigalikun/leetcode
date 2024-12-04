fn main() {
    assert_eq!(
        Solution::max_repeating("ababc".to_string(), "ab".to_string()),
        2
    );
    assert_eq!(
        Solution::max_repeating("ababc".to_string(), "ba".to_string()),
        1
    );
    assert_eq!(
        Solution::max_repeating("ababc".to_string(), "ac".to_string()),
        0
    );
    assert_eq!(
        Solution::max_repeating(
            "aaabaaaabaaabaaaabaaaabaaaabaaaaba".to_string(),
            "aaaba".to_string()
        ),
        5
    );
}

struct Solution;
impl Solution {
    pub fn max_repeating(sequence: String, word: String) -> i32 {
        let mut k = 0;
        let mut repeated_word = String::new();

        loop {
            repeated_word.push_str(&word.clone());
            if !sequence.contains(&repeated_word) {
                break;
            }
            k += 1;
        }

        k
    }
}
