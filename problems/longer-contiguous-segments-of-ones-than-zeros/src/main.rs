fn main() {
    assert_eq!(Solution::check_zero_ones("1101".to_string()), true);
    assert_eq!(Solution::check_zero_ones("111000".to_string()), false);
    assert_eq!(Solution::check_zero_ones("110100010".to_string()), false);
}

struct Solution;
impl Solution {
    pub fn check_zero_ones(s: String) -> bool {
        return s.split('0').map(|x| x.len()).max().unwrap_or(0)
            > s.split('1').map(|x| x.len()).max().unwrap_or(0);
    }
}
