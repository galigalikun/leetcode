fn main() {
    assert_eq!(
        Solution::print_vertically("HOW ARE YOU".to_string()),
        vec!["HAY", "ORO", "WEU"]
    );
    assert_eq!(
        Solution::print_vertically("TO BE OR NOT TO BE".to_string()),
        vec!["TBONTB", "OEROOE", "   T"]
    );
    assert_eq!(
        Solution::print_vertically("CONTEST IS COMING".to_string()),
        vec!["CIC", "OSO", "N M", "T I", "E N", "S G", "T"]
    );
}

struct Solution;
impl Solution {
    pub fn print_vertically(s: String) -> Vec<String> {
        let words: Vec<&str> = s.split_whitespace().collect();
        let mut max_len = 0;
        for word in &words {
            max_len = std::cmp::max(max_len, word.len());
        }
        let mut result = vec![];
        for i in 0..max_len {
            let mut word = String::new();
            for j in 0..words.len() {
                if i < words[j].len() {
                    word.push(words[j].chars().nth(i).unwrap());
                } else {
                    word.push(' ');
                }
            }
            result.push(word.trim_end().to_string());
        }
        return result;
    }
}
