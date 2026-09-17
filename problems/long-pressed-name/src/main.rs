fn main() {
    assert_eq!(Solution::is_long_pressed_name("alex".to_string(), "aaleex".to_string()), true);
    assert_eq!(Solution::is_long_pressed_name("saeed".to_string(), "ssaaedd".to_string()), false);
}

struct Solution;
impl Solution {
    pub fn is_long_pressed_name(name: String, typed: String) -> bool {
        let name_bytes = name.as_bytes();
        let typed_bytes = typed.as_bytes();

        let mut i = 0;
        let mut j = 0;

        while j < typed_bytes.len() {
            if i < name_bytes.len() && name_bytes[i] == typed_bytes[j] {
                i += 1;
                j += 1;
            } else if j > 0 && typed_bytes[j] == typed_bytes[j - 1] {
                j += 1;
            } else {
                return false;
            }
        }

        i == name_bytes.len()
    }
}
