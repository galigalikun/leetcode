fn main() {
    assert_eq!(Solution::longest_balanced("abbac".to_string()), 4);
    assert_eq!(Solution::longest_balanced("aabcc".to_string()), 3);
    assert_eq!(Solution::longest_balanced("aba".to_string()), 2);
}

struct Solution;
impl Solution {
    pub fn longest_balanced(s: String) -> i32 {
        let mut max_length = 0;
        let mut count_a = 0;
        let mut count_b = 0;
        for c in s.chars() {
            if c == 'a' {
                count_a += 1;
            } else if c == 'b' {
                count_b += 1;
            }
            if count_a == count_b {
                max_length = max_length.max(count_a + count_b);
            }
        }
        max_length
    }
}
