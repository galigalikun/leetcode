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
}

struct Solution;
impl Solution {
    pub fn find_ocurrences(text: String, first: String, second: String) -> Vec<String> {
        let mut result = vec![];
        let mut words = text.split_whitespace();
        let mut first_word = words.next();
        let mut second_word = words.next();
        while let (Some(f), Some(s)) = (first_word, second_word) {
            if f == first && s == second {
                if let Some(t) = words.next() {
                    result.push(t.to_string());
                }
            }
            first_word = second_word;
            second_word = words.next();
        }
        return result;
    }
}
