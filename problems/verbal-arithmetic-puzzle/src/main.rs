fn main() {
    assert_eq!(Solution::is_solvable(vec!["SEND".to_string(),"MORE".to_string()], "MONEY".to_string()), true);
    assert_eq!(Solution::is_solvable(vec!["SIX".to_string(),"SEVEN".to_string(),"SEVEN".to_string()], "TWENTY".to_string()), true);
    assert_eq!(Solution::is_solvable(vec!["LEET".to_string(),"CODE".to_string()], "POINT".to_string()), false);
}

struct Solution;
impl Solution {
    pub fn is_solvable(words: Vec<String>, result: String) -> bool {
        let mut chars = vec![false; 26];
        let mut first_chars = vec![false; 26];
        let mut first_char = vec![false; 26];
        let mut first_char_idx = vec![false; 26];
        for word in words.iter() {
            let mut first = true;
            for c in word.chars().rev() {
                let idx = (c as u8 - 'A' as u8) as usize;
                if first {
                    first = false;
                    first_char[idx] = true;
                }
                if first_chars[idx] {
                    first_char_idx[idx] = true;
                }
                first_chars[idx] = true;
                chars[idx] = true;
            }
        }
        let mut first = true;
        for c in result.chars().rev() {
            let idx = (c as u8 - 'A' as u8) as usize;
            if first {
                first = false;
                first_char[idx] = true;
            }
            if first_chars[idx] {
                first_char_idx[idx] = true;
            }
            first_chars[idx] = true;
            chars[idx] = true;
        }

        return false;
    }
}
