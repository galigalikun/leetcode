fn main() {
    assert_eq!(
        Solution::is_isomorphic("egg".to_string(), "add".to_string()),
        true
    );

    assert_eq!(
        Solution::is_isomorphic("foo".to_string(), "bar".to_string()),
        false
    );

    assert_eq!(
        Solution::is_isomorphic("paper".to_string(), "title".to_string()),
        true
    );

    assert_eq!(
        Solution::is_isomorphic("badc".to_string(), "baba".to_string()),
        false
    );
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut i = 0;
        let mut map1: HashMap<char, char> = HashMap::new();
        let mut map2: HashMap<char, char> = HashMap::new();
        for c in s.as_str().chars() {
            if let Some(cc) = t.as_str().chars().nth(i) {
                if let Some(&v) = map1.get(&c) {
                    if v != cc {
                        return false;
                    }
                } else if let Some(&v) = map2.get(&cc) {
                    if v != c {
                        return false;
                    }
                } else {
                    map1.insert(c, cc);
                    map2.insert(cc, c);
                }
            }
            i += 1;
        }
        return true;
    }
}
