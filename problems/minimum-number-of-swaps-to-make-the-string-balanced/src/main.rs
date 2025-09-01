fn main() {
    assert_eq!(Solution::min_swaps("][][".to_string()), 1);
    assert_eq!(Solution::min_swaps("]]][[[".to_string()), 2);
    assert_eq!(Solution::min_swaps("[]".to_string()), 0);
}

struct Solution;
impl Solution {
    pub fn min_swaps(s: String) -> i32 {
        let mut left = 0;
        let mut right = 0;
        for c in s.chars() {
            if c == '[' {
                left += 1;
            } else {
                right += 1;
            }
        }
        let a: i32 = left - right;
        a.abs() / 2
    }
}
