fn main() {
    assert_eq!(
        Solution::check_if_can_break("abc".to_string(), "xya".to_string()),
        true
    );
    assert_eq!(
        Solution::check_if_can_break("abe".to_string(), "acd".to_string()),
        false
    );
    assert_eq!(
        Solution::check_if_can_break("leetcodee".to_string(), "interview".to_string()),
        true
    );
    assert_eq!(
        Solution::check_if_can_break("szy".to_string(), "cid".to_string()),
        true
    );
}

struct Solution;
impl Solution {
    pub fn check_if_can_break(s1: String, s2: String) -> bool {
        let mut s1 = s1.chars().collect::<Vec<char>>();
        let mut s2 = s2.chars().collect::<Vec<char>>();
        s1.sort();
        for i in 0..s1.len() {
            if s1[i] > s2[i] {
                s1.swap(i, i);
            }
        }
        s2.sort();
        for i in 0..s2.len() {
            if s2[i] > s1[i] {
                s2.swap(i, i);
            }
        }
        for i in 0..s1.len() {
            if s1[i] > s2[i] {
                return false;
            }
        }
        return true;
    }
}
