fn main() {
    assert_eq!(Solution::divisor_game(2), true);
    assert_eq!(Solution::divisor_game(3), false);
}

struct Solution;
impl Solution {
    pub fn divisor_game(n: i32) -> bool {
        return if n % 2 == 0 {
            true
        } else {
            false
        };
    }
}
