fn main() {
    assert_eq!(
        Solution::group_anagrams(vec![
            "eat".to_string(),
            "tea".to_string(),
            "tan".to_string(),
            "ate".to_string(),
            "nat".to_string(),
            "bat".to_string()
        ]),
        vec![
            vec!["eat".to_string(), "tea".to_string(), "ate".to_string()],
            vec!["tan".to_string(), "nat".to_string()],
            vec!["bat".to_string()]
        ]
    );
    assert_eq!(
        Solution::group_anagrams(vec!["".to_string()]),
        vec![vec!["".to_string()]]
    );
    assert_eq!(
        Solution::group_anagrams(vec!["a".to_string()]),
        vec![vec!["a".to_string()]]
    );
    assert_eq!(
        Solution::group_anagrams(vec!["ddddddddddg".to_string(), "dgggggggggg".to_string()]),
        vec![
            vec!["ddddddddddg".to_string()],
            vec!["dgggggggggg".to_string()]
        ]
    );
}

pub struct Solution {}
use std::collections::HashMap;
impl Solution {
    fn is_word(s1: String, s2: String) -> Option<String> {
        if s1 == s2 {
            return Some(s2);
        }
        if s1.len() != s2.len() {
            return None;
        }
        let mut ss = s1;
        for c2 in s2.as_str().chars() {
            let sss = ss.replacen(c2, "", 1);
            if ss.len() > sss.len() {
                ss = sss;
            } else {
                return None;
            }
        }
        if ss.len() == 0 {
            return Some(s2);
        }
        return None;
    }
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = Vec::new();
        let mut map: HashMap<usize, Vec<(usize, String)>> = HashMap::new();
        let mut idx = 0;
        for str in strs {
            let mut is_push = false;
            if let Some(m) = map.get(&str.len()) {
                for (i, check) in m {
                    if let Some(s) = Solution::is_word(check.to_string(), str.clone()) {
                        result[*i].push(s);
                        is_push = true;
                        break;
                    }
                }
            }
            if !is_push {
                let ss = str.clone();
                result.push(vec![str]);
                if let Some(m) = map.get_mut(&ss.len()) {
                    m.push((idx, ss));
                } else {
                    map.insert(ss.len(), vec![(idx, ss)]);
                }
                idx += 1;
            }
        }
        return result;
    }
}
