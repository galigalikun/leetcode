fn main() {
    assert_eq!(Solution::longest_prefix("level".to_string()), "l");
    assert_eq!(Solution::longest_prefix("ababab".to_string()), "abab");
}

struct Solution;
impl Solution {
    pub fn longest_prefix(s: String) -> String {
        return s
            .chars()
            .enumerate()
            .fold((0, 0), |(hash, len), (i, c)| {
                let hash = (hash * 26 + c as u32 - 'a' as u32) % 1_000_000_007;
                (hash, if hash == 0 { i + 1 } else { len })
            })
            .1
            .to_string();
    }
}
