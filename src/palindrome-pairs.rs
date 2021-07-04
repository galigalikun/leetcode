fn main() {
    assert_eq!(
        Solution::palindrome_pairs(vec![
            "abcd".to_string(),
            "dcba".to_string(),
            "lls".to_string(),
            "s".to_string(),
            "sssll".to_string()
        ]),
        vec![[0, 1], [1, 0], [3, 2], [2, 4]]
    );

    assert_eq!(
        Solution::palindrome_pairs(vec![
            "bat".to_string(),
            "tab".to_string(),
            "cat".to_string()
        ]),
        vec![[0, 1], [1, 0]]
    );

    assert_eq!(
        Solution::palindrome_pairs(vec!["a".to_string(), "".to_string()]),
        vec![[0, 1], [1, 0]]
    );
}

pub struct Solution {}
// https://medium.com/@harycane/palindrome-pairs-46c5b8511397
// https://evelynn.gitbooks.io/google-interview/content/palindrome-pairs.html
use std::collections::HashMap;
impl Solution {
    fn is_palindrome(s: &str) -> bool {
        if s.len() == 0 {
            return true;
        }
        let mut left = 0;
        let mut right = s.len() - 1;
        while left <= right {
            if s.chars().nth(left) != s.chars().nth(right) {
                return false;
            }
            if right == 0 {
                return true;
            }
            left += 1;
            right -= 1;
        }
        return true;
    }
    pub fn palindrome_pairs(words: Vec<String>) -> Vec<Vec<i32>> {
        let mut map = HashMap::new();
        let mut idx = 0;
        for w in words.clone() {
            map.insert(w, idx);
            idx += 1;
        }
        let mut result = vec![];
        for i in 0..words.len() {
            let mut left = 0;
            let mut right = 0;
            while left <= right {
                let s = &words[i][left..right];
                if let Some(&idx) = map.get(&s.chars().rev().collect::<String>()) {
                    if idx != i
                        && Solution::is_palindrome(
                            &words[i][if left == 0 {
                                right..words[i].len()
                            } else {
                                0..left
                            }],
                        )
                    {
                        result.push(if left == 0 {
                            vec![i as i32, idx as i32]
                        } else {
                            vec![idx as i32, i as i32]
                        });
                    }
                }
                if right < words[i].len() {
                    right += 1;
                } else {
                    left += 1;
                }
            }
        }
        return result;
    }
}
