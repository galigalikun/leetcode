fn main() {
    assert_eq!(Solution::square_is_white("a1".to_string()), false);
    assert_eq!(Solution::square_is_white("h3".to_string()), true);
    assert_eq!(Solution::square_is_white("c7".to_string()), false);
}

struct Solution;
impl Solution {
    pub fn square_is_white(coordinates: String) -> bool {
        return coordinates
            .chars()
            .map(|c| c.to_digit(36).unwrap() as i32)
            .sum::<i32>()
            % 2
            == 0;
    }
}
