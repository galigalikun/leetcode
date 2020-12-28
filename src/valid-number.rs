fn main() {
    assert_eq!(Solution::is_number("0".to_string()), true);
    assert_eq!(Solution::is_number("0.1".to_string()), true);
    assert_eq!(Solution::is_number("abc". to_string()), false);
    assert_eq!(Solution::is_number("1 a".to_string()), false);
    assert_eq!(Solution::is_number("2e10".to_string()), true);
    assert_eq!(Solution::is_number("1 ".to_string()), true);
}

pub struct Solution {}
impl Solution {
    pub fn is_number(s: String) -> bool {
        let ok = s.trim().parse::<f64>();

        return ok.is_ok();
    }
}
