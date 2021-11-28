fn main() {
    assert_eq!(
        Solution::find_longest_word(
            "abpcplea".to_string(),
            vec![
                "ale".to_string(),
                "apple".to_string(),
                "monkey".to_string(),
                "plea".to_string()
            ]
        ),
        "apple"
    );
    assert_eq!(
        Solution::find_longest_word(
            "abpcplea".to_string(),
            vec!["a".to_string(), "b".to_string(), "c".to_string()]
        ),
        "a"
    );
    assert_eq!(
        Solution::find_longest_word(
            "abce".to_string(),
            vec!["abe".to_string(), "abc".to_string()]
        ),
        "abc"
    );
}

// https://www.geeksforgeeks.org/find-largest-word-dictionary-deleting-characters-given-string/
struct Solution {}
impl Solution {
    fn helper(w1: String, w2: String) -> bool {
        let m = w1.len();
        let n = w2.len();
        let (mut i, mut j) = (0, 0);
        loop {
            if !(i < n && j < m) {
                break;
            }
            if w1.chars().nth(j) == w2.chars().nth(i) {
                j += 1;
            }
            i += 1;
        }
        return j == m;
    }
    pub fn find_longest_word(s: String, dictionary: Vec<String>) -> String {
        let mut len = 0;
        let mut result = "".to_string();
        for word in dictionary {
            if word.len() >= len && Solution::helper(word.clone(), s.clone()) {
                result = if word.len() == len {
                    std::cmp::min(result, word.clone())
                } else {
                    word.clone()
                };
                len = word.len();
            }
        }
        return result;
    }
}
