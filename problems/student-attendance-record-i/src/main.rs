fn main() {
    assert_eq!(Solution::check_record("PPALLP".to_string()), true);
    assert_eq!(Solution::check_record("PPALLL".to_string()), false);
}

struct Solution {}
use std::collections::HashMap;
impl Solution {
    pub fn check_record(s: String) -> bool {
        let mut map = HashMap::new();
        for c in s.chars() {
            if c != 'L' {
                map.remove(&'L');
            }
            if let Some(m) = map.get_mut(&c) {
                *m += 1;
                if c == 'L' {
                    if *m > 2 {
                        return false;
                    }
                } else {
                    if c == 'A' && *m > 1 {
                        return false;
                    }
                }
            } else {
                map.insert(c, 1);
            }
        }
        return true;
    }
}
