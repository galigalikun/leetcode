fn main() {
    assert_eq!(
        Solution::find_words(vec![
            "Hello".to_string(),
            "Alaska".to_string(),
            "Dad".to_string(),
            "Peace".to_string()
        ]),
        vec!["Alaska", "Dad"]
    );
    // assert_eq!(Solution::find_words(vec!["omk".to_string()]), vec![]);
    assert_eq!(
        Solution::find_words(vec!["adsdf".to_string(), "sfd".to_string()]),
        vec!["adsdf", "sfd"]
    );
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn find_words(words: Vec<String>) -> Vec<String> {
        let mut map = "qwertyuiop"
            .chars()
            .map(|c| (c, 0_i32))
            .collect::<HashMap<_, _>>();
        map.extend(
            "asdfghjkl"
                .chars()
                .map(|c| (c, 1_i32))
                .collect::<HashMap<_, _>>(),
        );
        map.extend(
            "zxcvbnm"
                .chars()
                .map(|c| (c, 2_i32))
                .collect::<HashMap<_, _>>(),
        );
        let mut result = vec![];
        for w in words {
            let mut p: Option<&i32> = None;
            let mut b = true;
            for c in w.chars() {
                if p != None && p != map.get(&c.to_ascii_lowercase()) {
                    b = false;
                    break;
                }
                p = map.get(&c.to_ascii_lowercase());
            }
            if b {
                result.push(w);
            }
        }
        return result;
    }
}
