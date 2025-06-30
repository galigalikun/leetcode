fn main() {
    assert_eq!(
        Solution::make_equal(vec![
            "abc".to_string(),
            "aabc".to_string(),
            "bc".to_string()
        ]),
        true
    );
    assert_eq!(
        Solution::make_equal(vec!["ab".to_string(), "a".to_string()]),
        false
    );
}

struct Solution;
impl Solution {
    pub fn make_equal(words: Vec<String>) -> bool {
        let mut count = [0; 26];
        let n = words.len() as i32;

        for word in words {
            for c in word.chars() {
                count[(c as u8 - b'a') as usize] += 1;
            }
        }

        for &c in &count {
            if c % n != 0 {
                return false;
            }
        }

        true
    }
}
