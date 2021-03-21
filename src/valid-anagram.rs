fn main() {
    assert_eq!(
        Solution::is_anagram("anagram".to_string(), "nagaram".to_string()),
        true
    );

    assert_eq!(
        Solution::is_anagram("rat".to_string(), "car".to_string()),
        false
    );
}

pub struct Solution {}
impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        let mut tmp = t.as_str().chars().collect::<Vec<_>>();
        for c in s.as_str().chars() {
            if let Some(p) = tmp.iter().position(|&x| x == c) {
                tmp.remove(p);
            } else {
                return false;
            }
        }
        if tmp.len() == 0 {
            return true;
        }
        return false;
    }
}
