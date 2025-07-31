fn main() {
    assert_eq!(
        Solution::count_palindromic_subsequence("aabca".to_string()),
        3
    );
    assert_eq!(
        Solution::count_palindromic_subsequence("adc".to_string()),
        0
    );
    assert_eq!(
        Solution::count_palindromic_subsequence("bbcbaba".to_string()),
        4
    );
}

struct Solution;
impl Solution {
    pub fn count_palindromic_subsequence(s: String) -> i32 {
        return s
            .chars()
            .collect::<Vec<_>>()
            .windows(3)
            .filter(|w| w[0] == w[2])
            .map(|w| {
                let mut set = std::collections::HashSet::new();
                for c in s[1..s.len() - 1].chars() {
                    if c != w[0] {
                        set.insert(c);
                    }
                }
                set.len() as i32
            })
            .sum::<i32>() + s.chars().collect::<std::collections::HashSet<_>>().len() as i32;
    }
}
