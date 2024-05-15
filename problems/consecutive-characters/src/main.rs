fn main() {
    assert_eq!(Solution::max_power("leetcode".to_string()), 2);
    assert_eq!(Solution::max_power("abbcccddddeeeeedcba".to_string()), 5);
}

struct Solution;
impl Solution {
    pub fn max_power(s: String) -> i32 {
        let mut max = 1;
        let mut count = 1;
        let mut prev = s.chars().nth(0).unwrap();
        for c in s.chars().skip(1) {
            if c == prev {
                count += 1;
                max = max.max(count);
            } else {
                count = 1;
                prev = c;
            }
        }
        return max;
    }
}
