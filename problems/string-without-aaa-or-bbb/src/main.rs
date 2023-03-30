fn main() {
    assert_eq!(Solution::str_without3a3b(1, 2), "abb");
    assert_eq!(Solution::str_without3a3b(4, 1), "aabaa");
}

struct Solution;
impl Solution {
    pub fn str_without3a3b(a: i32, b: i32) -> String {
        return format!("{}{}", "a".repeat(a as usize), "b".repeat(b as usize));
    }
}
