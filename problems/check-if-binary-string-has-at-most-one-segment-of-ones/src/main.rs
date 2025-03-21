fn main() {
    assert_eq!(Solution::check_ones_segment("1001".to_string()), false);
    assert_eq!(Solution::check_ones_segment("110".to_string()), true);
}

struct Solution;
impl Solution {
    pub fn check_ones_segment(s: String) -> bool {
        return !s.contains("01");
    }
}
