fn main() {
    assert_eq!(Solution::query_string("0110".to_string(), 3), true);
    assert_eq!(Solution::query_string("0110".to_string(), 4), false);
}

struct Solution;
impl Solution {
    pub fn query_string(s: String, n: i32) -> bool {
        for i in 1..=n {
            if !s.contains(&format!("{:b}", i)) {
                return false;
            }
        }
        return true;
    }
}
