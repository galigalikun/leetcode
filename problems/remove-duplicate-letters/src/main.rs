fn main() {
    assert_eq!(
        Solution::remove_duplicate_letters("bcabc".to_string()),
        "abc".to_string()
    );

    assert_eq!(
        Solution::remove_duplicate_letters("cbacdcbc".to_string()),
        "acdb".to_string()
    );
}

pub struct Solution {}
// https://ttzztt.gitbooks.io/lc/content/remove-duplicate-letters.html
use std::collections::HashMap;
impl Solution {
    pub fn remove_duplicate_letters(s: String) -> String {
        if s.len() == 0 {
            return "".to_string();
        }
        let mut map = HashMap::new();
        for c in s.chars() {
            if let Some(m) = map.get_mut(&c) {
                *m += 1;
            } else {
                map.insert(c, 1);
            }
        }

        let mut pos = 0;
        for i in 0..s.len() {
            if s.chars().nth(pos) > s.chars().nth(i) {
                pos = i;
            }
            if let Some(m) = map.get_mut(&s.chars().nth(i).unwrap()) {
                if *m == 1 {
                    break;
                }
                *m -= 1;
            }
        }

        let c = s.chars().nth(pos).unwrap();

        return format!(
            "{}{}",
            c,
            Solution::remove_duplicate_letters((&s[pos + 1..].replace(c, "")).to_string())
        );
    }
}
