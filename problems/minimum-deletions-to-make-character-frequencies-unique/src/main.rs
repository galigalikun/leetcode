fn main() {
    assert_eq!(Solution::min_deletions("aab".to_string()), 0);
    assert_eq!(Solution::min_deletions("aaabbbcc".to_string()), 2);
    assert_eq!(Solution::min_deletions("ceabaacb".to_string()), 2);
    assert_eq!(Solution::min_deletions("bbcebab".to_string()), 2);
}

struct Solution;
impl Solution {
    pub fn min_deletions(s: String) -> i32 {
        let mut freq = vec![0; 26];
        for c in s.chars() {
            freq[c as usize - 'a' as usize] += 1;
        }
        freq.sort_unstable();
        let mut res = 0;
        for i in (0..25).rev() {
            if freq[i] == 0 {
                break;
            }
            if freq[i] >= freq[i + 1] {
                res += freq[i] - freq[i + 1] + 1;
                freq[i] = freq[i + 1] - 1;
            }
        }
        res
    }
}
