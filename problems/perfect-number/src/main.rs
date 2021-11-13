fn main() {
    assert_eq!(Solution::check_perfect_number(28), true);
    assert_eq!(Solution::check_perfect_number(6), true);
    assert_eq!(Solution::check_perfect_number(496), true);
    assert_eq!(Solution::check_perfect_number(8128), true);
    assert_eq!(Solution::check_perfect_number(2), false);
}

struct Solution {}
impl Solution {
    pub fn check_perfect_number(num: i32) -> bool {
        return match num {
            6 | 28 | 496 | 8128 | 33550336 => true,
            _ => false,
        };
    }
}
