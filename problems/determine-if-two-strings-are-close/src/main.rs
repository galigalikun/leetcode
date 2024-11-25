fn main() {
    assert_eq!(
        Solution::close_strings("abc".to_string(), "bca".to_string()),
        true
    );
    assert_eq!(
        Solution::close_strings("a".to_string(), "aa".to_string()),
        false
    );
    assert_eq!(
        Solution::close_strings("cabbba".to_string(), "abbccc".to_string()),
        true
    );
    assert_eq!(
        Solution::close_strings("uau".to_string(), "ssx".to_string()),
        false
    );
}

struct Solution;
impl Solution {
    pub fn close_strings(word1: String, word2: String) -> bool {
        let mut freq1 = vec![0; 26];
        let mut freq2 = vec![0; 26];
        let mut chars1 = vec![0; 26];
        let mut chars2 = vec![0; 26];
        for c in word1.chars() {
            freq1[(c as u8 - 'a' as u8) as usize] += 1;
            chars1[(c as u8 - 'a' as u8) as usize] = 1;
        }
        for c in word2.chars() {
            freq2[(c as u8 - 'a' as u8) as usize] += 1;
            chars2[(c as u8 - 'a' as u8) as usize] = 1;
        }
        freq1.sort();
        freq2.sort();
        chars1.sort();
        chars2.sort();
        if freq1 == freq2 && chars1 == chars2 {
            return true;
        }
        return false;
    }
}
