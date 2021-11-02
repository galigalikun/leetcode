fn main() {
    assert_eq!(Solution::longest_substring("aaabb".to_string(), 3), 3);
    assert_eq!(Solution::longest_substring("ababbc".to_string(), 2), 5);
}

pub struct Solution {}
// https://baihuqian.github.io/2018-09-03-longest-substring-with-at-least-k-repeating-characters/
impl Solution {
    pub fn longest_substring(s: String, k: i32) -> i32 {
        if s.len() == 0 {
            return 0;
        }
        let mut chars = vec![0; 26]; // alphabet vec
        for i in 0..s.len() {
            let idx = (s.chars().nth(i).unwrap() as i32 - 97) as usize;
            chars[idx] += 1;
        }
        let unique = chars.iter().filter(|&x| x < &k && x > &0).count();
        if unique == 0 {
            return s.len() as i32;
        }

        let mut result = 0;
        let mut start = 0;
        let mut current = 0;
        while current < s.len() {
            let idx = (s.chars().nth(current).unwrap() as i32 - 97) as usize;
            if chars[idx] < k {
                result = std::cmp::max(
                    result,
                    Solution::longest_substring((&s[start..current]).to_string(), k),
                );
                start = current + 1;
            }
            current += 1;
        }

        result = std::cmp::max(
            result,
            Solution::longest_substring((&s[start..]).to_string(), k),
        );
        return result;
    }
}
