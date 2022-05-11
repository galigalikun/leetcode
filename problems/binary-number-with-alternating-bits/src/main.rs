fn main() {
    assert_eq!(Solution::has_alternating_bits(5), true);
    assert_eq!(Solution::has_alternating_bits(7), false);
    assert_eq!(Solution::has_alternating_bits(11), false);
}

struct Solution {}
impl Solution {
    pub fn has_alternating_bits(n: i32) -> bool {
        let s = format!("{0:b}", n);
        for i in 0..s.len() - 1 {
            if s.chars().nth(i).unwrap() == s.chars().nth(i + 1).unwrap() {
                return false;
            }
        }
        return true;
    }
}
