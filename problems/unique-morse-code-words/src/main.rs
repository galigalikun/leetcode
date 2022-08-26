fn main() {
    assert_eq!(
        Solution::unique_morse_representations(vec![
            "gin".to_string(),
            "zen".to_string(),
            "gig".to_string(),
            "msg".to_string()
        ]),
        2
    );
    assert_eq!(
        Solution::unique_morse_representations(vec!["a".to_string()]),
        1
    );
    assert_eq!(
        Solution::unique_morse_representations(vec![
            "rwjje".to_string(),
            "aittjje".to_string(),
            "auyyn".to_string(),
            "lqtktn".to_string(),
            "lmjwn".to_string()
        ]),
        1
    );
}

struct Solution {}

use std::collections::HashMap;
impl Solution {
    fn helper(s: String) -> String {
        let map = [
            ".-", "-...", "-.-.", "-..", ".", "..-.", "--.", "....", "..", ".---", "-.-", ".-..",
            "--", "-.", "---", ".--.", "--.-", ".-.", "...", "-", "..-", "...-", ".--", "-..-",
            "-.--", "--..",
        ];
        let mut ret = String::new();
        for c in s.chars() {
            let idx = c as usize - 97;
            ret = format!("{}{}", ret, map[idx])
        }
        return ret;
    }
    pub fn unique_morse_representations(words: Vec<String>) -> i32 {
        let mut map = HashMap::new();
        for w in words {
            let s = Self::helper(w.clone());
            if let Some(m) = map.get_mut(&s) {
                *m += 1;
            } else {
                map.insert(s, 1);
            }
        }
        return map.len() as i32;
    }
}
