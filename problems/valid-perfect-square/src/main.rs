fn main() {
    assert_eq!(Solution::is_perfect_square(16), true);
    assert_eq!(Solution::is_perfect_square(14), false);
    assert_eq!(Solution::is_perfect_square(9), true);
    assert_eq!(Solution::is_perfect_square(5), false);
}

pub struct Solution {}
impl Solution {
    pub fn is_perfect_square(num: i32) -> bool {
        return (num as f32).sqrt() % 1.0 == 0.0;
    }
}
