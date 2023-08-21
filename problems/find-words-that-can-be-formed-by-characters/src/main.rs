fn main() {
    assert_eq!(
        Solution::count_characters(
            vec![
                "cat".to_string(),
                "bt".to_string(),
                "hat".to_string(),
                "tree".to_string()
            ],
            "atach".to_string()
        ),
        6
    );
    assert_eq!(
        Solution::count_characters(
            vec![
                "hello".to_string(),
                "world".to_string(),
                "leetcode".to_string()
            ],
            "welldonehoneyr".to_string()
        ),
        10
    );
}

struct Solution;
impl Solution {
    fn is_good(chars_count: &[i32], word_count: &[i32]) -> bool {
        for i in 0..26 {
            if chars_count[i] < word_count[i] {
                return false;
            }
        }
        return true;
    }
    pub fn count_characters(words: Vec<String>, chars: String) -> i32 {
        let mut chars_count = [0; 26];
        for c in chars.chars() {
            chars_count[(c as u8 - 'a' as u8) as usize] += 1;
        }
        let mut result = 0;
        for word in words {
            let mut word_count = [0; 26];
            for c in word.chars() {
                word_count[(c as u8 - 'a' as u8) as usize] += 1;
            }
            if Solution::is_good(&chars_count, &word_count) {
                result += word.len();
            }
        }
        return result as i32;
    }
}
