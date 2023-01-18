fn main() {
    assert_eq!(Solution::is_long_pressed_name("alex".to_string(), "aaleex".to_string()), true);
    assert_eq!(Solution::is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()), false);
}

struct Solution;
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        if name == typed {
            return true;
        }
        return false;
    }
}
