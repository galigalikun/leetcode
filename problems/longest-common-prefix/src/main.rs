fn main() {
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "flower".to_string(),
            "flow".to_string(),
            "flight".to_string()
        ]),
        "fl".to_string()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec![
            "dog".to_string(),
            "racecar".to_string(),
            "car".to_string()
        ]),
        "".to_string()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec!["a".to_string()]),
        "a".to_string()
    );
    assert_eq!(
        Solution::longest_common_prefix(vec!["ab".to_string(), "a".to_string()]),
        "a".to_string()
    );
}

pub struct Solution {}

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut v = strs.iter();
        let mut prefix: Option<String> = None;
        if strs.len() == 1 {
            if let Some(s) = strs.first() {
                return s.to_string();
            }
        }
        if let Some(f) = v.next() {
            while let Some(s) = v.next() {
                if f.len() > s.len() {
                    let mut hit = false;
                    for i in (1..=s.len()).rev() {
                        if f[0..i] == s[0..i] {
                            hit = true;
                            if let Some(ref p) = prefix {
                                if p.len() > i {
                                    prefix = Some(s[0..i].to_string());
                                }
                            } else {
                                prefix = Some(s[0..i].to_string());
                            }
                            break;
                        }
                    }
                    if !hit {
                        prefix = Some("".to_string());
                    }
                } else {
                    let mut hit = false;
                    for i in (1..=f.len()).rev() {
                        if f[0..i] == s[0..i] {
                            hit = true;
                            if let Some(ref p) = prefix {
                                if p.len() > i {
                                    prefix = Some(s[0..i].to_string());
                                }
                            } else {
                                prefix = Some(s[0..i].to_string());
                            }
                            break;
                        }
                    }
                    if !hit {
                        prefix = Some("".to_string());
                    }
                }
            }
        }
        if let Some(p) = prefix {
            return p;
        }
        return "".to_string();
    }
}
