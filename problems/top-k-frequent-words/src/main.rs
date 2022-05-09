fn main() {
    assert_eq!(
        Solution::top_k_frequent(
            vec![
                "i".to_string(),
                "love".to_string(),
                "leetcode".to_string(),
                "i".to_string(),
                "love".to_string(),
                "coding".to_string()
            ],
            2
        ),
        vec!["i", "love"]
    );
    assert_eq!(
        Solution::top_k_frequent(
            vec![
                "the".to_string(),
                "day".to_string(),
                "is".to_string(),
                "sunny".to_string(),
                "the".to_string(),
                "the".to_string(),
                "the".to_string(),
                "sunny".to_string(),
                "is".to_string(),
                "is".to_string()
            ],
            4
        ),
        vec!["the", "is", "sunny", "day"]
    );
}

struct Solution {}
impl Solution {
    pub fn top_k_frequent(words: Vec<String>, k: i32) -> Vec<String> {
        return vec![];
    }
}
