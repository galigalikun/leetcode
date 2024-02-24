fn main() {
    assert_eq!(Solution::generate_the_string(4), "aaab");
    assert_eq!(Solution::generate_the_string(2), "ab");
    assert_eq!(Solution::generate_the_string(7), "aaaaaaa");
}

struct Solution;
impl Solution {
    pub fn generate_the_string(n: i32) -> String {
        return if n % 2 == 0 {
            "a".repeat((n - 1) as usize) + "b"
        } else {
            "a".repeat(n as usize)
        };
    }
}
