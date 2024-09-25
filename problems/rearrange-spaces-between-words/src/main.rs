fn main() {
    assert_eq!(
        Solution::reorder_spaces("  this   is  a sentence ".to_string()),
        "this   is   a   sentence"
    );
    assert_eq!(
        Solution::reorder_spaces(" practice   makes   perfect".to_string()),
        "practice   makes   perfect "
    );
}

struct Solution;
impl Solution {
    pub fn reorder_spaces(text: String) -> String {
        let mut spaces = 0;
        let mut words = Vec::new();
        let mut word = String::new();
        for s in text.chars() {
            if s == ' ' {
                spaces += 1;
                if !word.is_empty() {
                    words.push(word.clone());
                    word.clear();
                }
            } else {
                word.push(s);
            }
        }
        if !word.is_empty() {
            words.push(word);
        }
        let mut result = String::new();
        if words.len() == 1 {
            result.push_str(&words[0]);
            for _ in 0..spaces {
                result.push(' ');
            }
            return result;
        }
        let space = spaces / (words.len() - 1);
        let remain = spaces % (words.len() - 1);
        for i in 0..words.len() {
            result.push_str(&words[i]);
            if i < words.len() - 1 {
                for _ in 0..space {
                    result.push(' ');
                }
            }
        }
        for _ in 0..remain {
            result.push(' ');
        }
        result
    }
}
