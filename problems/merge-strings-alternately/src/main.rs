fn main() {
    assert_eq!(
        Solution::merge_alternately("abc".to_string(), "pqr".to_string()),
        "apbqcr"
    );
    assert_eq!(
        Solution::merge_alternately("ab".to_string(), "pqrs".to_string()),
        "apbqrs"
    );
    assert_eq!(
        Solution::merge_alternately("abcd".to_string(), "pq".to_string()),
        "apbqcd"
    );
}

struct Solution;
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        return word1
            .chars()
            .zip(word2.chars())
            .flat_map(|(a, b)| vec![a, b])
            .chain(
                word1
                    .chars()
                    .skip(word2.len())
                    .chain(word2.chars().skip(word1.len())),
            )
            .collect();
    }
}
