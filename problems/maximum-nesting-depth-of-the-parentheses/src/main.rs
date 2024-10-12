fn main() {
    assert_eq!(Solution::max_depth("(1+(2*3)+((8)/4))+1".to_string()), 3);
    assert_eq!(Solution::max_depth("(1)+((2))+(((3)))".to_string()), 3);
    assert_eq!(Solution::max_depth("()(())((()()))".to_string()), 3);
}

struct Solution;
impl Solution {
    pub fn max_depth(s: String) -> i32 {
        return s.chars().fold(0, |acc, c| {
            if c == '(' {
                acc + 1
            } else if c == ')' {
                acc - 1
            } else {
                acc
            }
        });
    }
}
