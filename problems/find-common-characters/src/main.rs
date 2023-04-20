fn main() {
    assert_eq!(
        Solution::common_chars(vec![
            "bella".to_string(),
            "label".to_string(),
            "roller".to_string()
        ]),
        vec!["e", "l", "l"]
    );
    assert_eq!(
        Solution::common_chars(vec![
            "cool".to_string(),
            "lock".to_string(),
            "cook".to_string()
        ]),
        vec!["c", "o"]
    );
}

struct Solution;
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut result = vec![];
        let mut chars = vec![];
        for word in words {
            let mut char_count = vec![0; 26];
            for c in word.chars() {
                char_count[c as usize - 'a' as usize] += 1;
            }
            chars.push(char_count);
        }
        for i in 0..26 {
            let mut min = 100;
            for char_count in &chars {
                if char_count[i] < min {
                    min = char_count[i];
                }
            }
            for _ in 0..min {
                result.push((i as u8 + 'a' as u8) as char);
            }
        }
        return result.iter().map(|c| c.to_string()).collect();
    }
}
