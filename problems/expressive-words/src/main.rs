fn main() {
    assert_eq!(Solution::expressive_words("heeellooo".to_string(), vec!["hello".to_string(), "hi".to_string(), "helo".to_string()]), 1);
    assert_eq!(Solution::expressive_words("zzzzzyyyyy".to_string(), vec!["zzyy".to_string(),"zy".to_string(),"zyy".to_string()]), 3);
}

struct Solution{}
impl Solution {
    pub fn expressive_words(s: String, words: Vec<String>) -> i32 {
        words
            .iter()
            .filter(|word| Self::is_stretchy(&s, word))
            .count() as i32
    }

    fn is_stretchy(s: &str, word: &str) -> bool {
        let s_bytes = s.as_bytes();
        let w_bytes = word.as_bytes();
        let mut i = 0;
        let mut j = 0;

        while i < s_bytes.len() && j < w_bytes.len() {
            if s_bytes[i] != w_bytes[j] {
                return false;
            }

            let s_char = s_bytes[i];
            let mut s_count = 0;
            while i < s_bytes.len() && s_bytes[i] == s_char {
                i += 1;
                s_count += 1;
            }

            let w_char = w_bytes[j];
            let mut w_count = 0;
            while j < w_bytes.len() && w_bytes[j] == w_char {
                j += 1;
                w_count += 1;
            }

            if s_count < w_count {
                return false;
            }
            if s_count != w_count && s_count < 3 {
                return false;
            }
        }

        i == s_bytes.len() && j == w_bytes.len()
    }
}
