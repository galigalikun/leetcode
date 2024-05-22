fn main() {
    assert_eq!(Solution::arrange_words("Leetcode is cool".to_string()), "Is cool leetcode");
    assert_eq!(Solution::arrange_words("Keep calm and code on".to_string()), "On and keep calm code");
    assert_eq!(Solution::arrange_words("To be or not to be".to_string()), "To be or to be not");
}

struct Solution;
impl Solution {
    pub fn arrange_words(text: String) -> String {
        return text
            .split_whitespace()
            .enumerate()
            .map(|(i, word)| (i, word.to_lowercase()))
            .collect::<Vec<(usize, String)>>()
            .sort_by(|a, b| a.1.len().cmp(&b.1.len()))
            .map(|(i, word)| {
                if i == 0 {
                    format!("{}{}", word.get(0..1).unwrap().to_uppercase(), word.get(1..).unwrap())
                } else {
                    word.to_string()
                }
            })
            .collect::<Vec<String>>()
            .join(" ");
    }
}
