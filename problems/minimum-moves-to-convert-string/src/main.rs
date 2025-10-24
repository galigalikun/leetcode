fn main() {
    assert_eq!(Solution::minimum_moves("XXX".to_string()), 1);
    assert_eq!(Solution::minimum_moves("XXOX".to_string()), 2);
    assert_eq!(Solution::minimum_moves("OOOO".to_string()), 3);
}

struct Solution;
impl Solution {
    pub fn minimum_moves(s: String) -> i32 {
        let mut count = 0;
        let mut i = 0;
        while i < s.len() {
            if s[i..].starts_with("XXX") {
                count += 1;
                i += 3;
            } else if s[i..].starts_with("XXO") {
                count += 2;
                i += 3;
            } else if s[i..].starts_with("XOO") {
                count += 2;
                i += 3;
            } else if s[i..].starts_with("OOO") {
                count += 1;
                i += 3;
            } else {
                i += 1;
            }
        }
        count
    }
}
