fn main() {
    assert_eq!(Solution::check_valid_string("())".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*)".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*))".to_string()), true);
}

struct Solution {}
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        for i in 0..s.len() {
            if s.chars().nth(i) == Some('*') {
                let mut s = s.clone();
                s.remove(i);
                if Solution::check_valid_string(s) {
                    return true;
                }
            }
        }
        return false;
    }
}
