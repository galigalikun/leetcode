fn main() {
    assert_eq!(Solution::character_replacement("ABAB".to_string(), 2), 4);
    assert_eq!(Solution::character_replacement("AABABBA".to_string(), 1), 4);
}

struct Solution {}
// https://aaronice.gitbook.io/lintcode/two_pointers/longest-repeating-character-replacement
impl Solution {
    pub fn character_replacement(s: String, k: i32) -> i32 {
        let mut count = vec![0; 26];
        let mut max_count = 0;
        let mut max_len = 0;
        let mut left = 0;
        for right in 0..s.len() {
            let idx = s.chars().nth(right).unwrap() as usize - 65;
            count[idx] += 1;
            max_count = std::cmp::max(max_count, count[idx]);
            while right - left + 1 - max_count > k as usize {
                let l = s.chars().nth(left).unwrap() as usize - 65;
                count[l] -= 1;
                left += 1;
            }
            max_len = std::cmp::max(max_len, right as i32 - left as i32 + 1);
        }
        return max_len;
    }
}
