fn main() {
    assert_eq!(Solution::make_good("leEeetcode".to_string()), "leetcode");
    assert_eq!(Solution::make_good("abBAcC".to_string()), "");
    assert_eq!(Solution::make_good("s".to_string()), "s");
}

struct Solution;
impl Solution {
    pub fn make_good(s: String) -> String {
        return s.chars().fold("".to_string(), |mut acc, c| {
            if !acc.is_empty()
                && (acc.chars().last().unwrap().to_ascii_lowercase() == c.to_ascii_lowercase()
                    && acc.chars().last().unwrap() != c)
            {
                acc.pop();
            } else {
                acc.push(c);
            }
            acc
        });
    }
}
