fn main() {
    assert_eq!(Solution::max_operations("1001101".to_string()), 4);
    assert_eq!(Solution::max_operations("00111".to_string()), 0);
}

struct Solution;
impl Solution {
    pub fn max_operations(s: String) -> i32 {
        let mut s = s;
        while s.contains("00") || s.contains("11") {
            if let Some(pos) = s.find("00") {
                s.replace_range(pos..pos + 2, "");
            } else if let Some(pos) = s.find("11") {
                s.replace_range(pos..pos + 2, "");
            }
        }
        return s.len() as i32;
    }
}
