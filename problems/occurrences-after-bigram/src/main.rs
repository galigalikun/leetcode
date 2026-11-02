fn main() {
    assert_eq!(
        Solution::find_ocurrences(
            "alice is a good girl she is a good student".to_string(),
            "a".to_string(),
            "good".to_string()
        ),
        vec!["girl", "student"]
    );
    assert_eq!(
        Solution::find_ocurrences(
            "we will we will rock you".to_string(),
            "we".to_string(),
            "will".to_string()
        ),
        vec!["we", "rock"]
    );
    assert_eq!(
        Solution::find_ocurrences(
            "a a a a".to_string(),
            "a".to_string(),
            "a".to_string()
        ),
        vec!["a", "a"]
    );
}

struct Solution;
impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let words: Vec<&str> = text.split_whitespace().collect();
        let mut result = Vec::new();

        for window in words.windows(3) {
            if window[0] == first && window[1] == second {
                result.push(window[2].to_string());
            }
        }

        result
    }
}
