fn main() {
    assert_eq!(
        Solution::find_lu_slength(vec![
            "aba".to_string(),
            "cdc".to_string(),
            "eae".to_string()
        ]),
        3
    );
    assert_eq!(
        Solution::find_lu_slength(vec!["aaa".to_string(), "aaa".to_string(), "aa".to_string()]),
        -1
    );
    assert_eq!(
        Solution::find_lu_slength(vec![[
            "aabbcc".to_string(),
            "aabbcc".to_string(),
            "cb".to_string()
        ]]),
        2
    );
}

struct Solution {}

use std::collections::HashMap;
impl Solution {
    pub fn find_lu_slength(strs: Vec<String>) -> i32 {
        let mut map = HashMap::new();
        let mut ans = 0;
        for s in strs {
            if let Some(m) = map.get_mut(&s) {
                *m += 1;
                return -1;
            } else {
                map.insert(s.clone(), 1);
                ans = std::cmp::max(s.len(), ans);
            }
        }
        return ans as i32;
    }
}
