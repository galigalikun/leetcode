fn main() {
    assert_eq!(
        Solution::backspace_compare("ab#c".to_string(), "ad#c".to_string()),
        true
    );
    assert_eq!(
        Solution::backspace_compare("ab##".to_string(), "c#d#".to_string()),
        true
    );
    assert_eq!(
        Solution::backspace_compare("a#c".to_string(), "b".to_string()),
        false
    );
    assert_eq!(
        Solution::backspace_compare("a##c".to_string(), "#a#c".to_string()),
        true
    );
}

struct Solution;
impl Solution {
    pub fn backspace_compare(s: String, t: String) -> bool {
        let mut s1 = String::new();
        let mut t1 = String::new();
        for i in 0..std::cmp::max(s.len(), t.len()) {
            if let Some(c) = s.chars().nth(i) {
                if c == '#' {
                    if s1.len() > 0 {
                        s1 = format!("{}", &s1[..s1.len() - 1]);
                    }
                } else {
                    s1 = format!("{}{}", s1, c);
                }
            }
            if let Some(c) = t.chars().nth(i) {
                if c == '#' {
                    if t1.len() > 0 {
                        t1 = format!("{}", &t1[..t1.len() - 1]);
                    }
                } else {
                    t1 = format!("{}{}", t1, c);
                }
            }
        }
        return if s1 == t1 { true } else { false };
    }
}
