fn main() {
    assert_eq!(
        Solution::palindrome_pairs(vec![
            "abcd".to_string(),
            "dcba".to_string(),
            "lls".to_string(),
            "s".to_string(),
            "sssll".to_string()
        ]),
        vec![[1, 0], [0, 1], [3, 2], [2, 4]]
    );

    assert_eq!(
        Solution::palindrome_pairs(vec![
            "bat".to_string(),
            "tab".to_string(),
            "cat".to_string()
        ]),
        vec![[1, 0], [0, 1]]
    );

    assert_eq!(
        Solution::palindrome_pairs(vec!["a".to_string(), "".to_string()]),
        vec![[0, 1], [1, 0]]
    );
}

pub struct Solution {}
// https://medium.com/@harycane/palindrome-pairs-46c5b8511397
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
            for j in 0..=words[i].len() {
                let s1 = &words[i][0..j];
                let s2 = &words[i][j..];
                if Solution::is_palindrome(s1) {
                    if let Some(&idx) = map.get(&s2.chars().rev().collect::<String>()) {
                        if idx != i {
                            result.push(vec![idx as i32, i as i32]);
                        }
                    }
                }

                if s2.len() > 0 && Solution::is_palindrome(s2) {
                    if let Some(&idx) = map.get(&s1.chars().rev().collect::<String>()) {
                        if idx != i {
                            result.push(vec![i as i32, idx as i32]);
                        }
                    }
                }
            }
        }
        return result;
    }
}
