fn main() {
    assert_eq!(Solution::check_valid_string("())".to_string()), true);
    assert_eq!(Solution::check_valid_string("(*))".to_string()), true);
}

struct Solution{}
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        return false;
    }
}
