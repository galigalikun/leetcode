fn main() {
    assert_eq!(
        Solution::buddy_strings("ab".to_string(), "ba".to_string()),
        true
    );
    assert_eq!(
        Solution::buddy_strings("ab".to_string(), "ab".to_string()),
        false
    );
    assert_eq!(
        Solution::buddy_strings("aa".to_string(), "aa".to_string()),
        true
    );
    assert_eq!(
        Solution::buddy_strings("aaaaaaabc".to_string(), "aaaaaaacb".to_string()),
        true
    );
    assert_eq!(
        Solution::buddy_strings("abcd".to_string(), "badc".to_string()),
        false
    );
}

struct Solution;
impl Solution {
    pub fn buddy_strings(s: String, goal: String) -> bool {
        let mut map = vec![0; goal.len()];
        let mut ans = 0;
        for i in s.chars().enumerate() {
            for j in goal.chars().enumerate() {
                if i.0 != j.0 && i.1 == j.1 {
                    ans += 1;
                    if ans == 2 {
                        return true;
                    }
                    map[j.0] = 1;
                }
            }
        }
        return if ans == 2 { true } else { false };
    }
}
