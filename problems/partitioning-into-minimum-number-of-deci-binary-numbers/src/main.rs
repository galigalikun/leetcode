fn main() {
    assert_eq!(Solution::min_partitions("32".to_string()), 3);
    assert_eq!(Solution::min_partitions("82734".to_string()), 8);
    assert_eq!(
        Solution::min_partitions("27346209830709182346".to_string()),
        9
    );
}

struct Solution;
impl Solution {
    pub fn min_partitions(n: String) -> i32 {
        return n.chars().max().unwrap().to_digit(10).unwrap() as i32;
    }
}
