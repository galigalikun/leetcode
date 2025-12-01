fn main() {
    assert_eq!(Solution::min_anagram_length("abba".to_string()), 2);
    assert_eq!(Solution::min_anagram_length("cdef".to_string()), 4);
    assert_eq!(
        Solution::min_anagram_length("abcbcacabbaccba".to_string()),
        3
    );
}

struct Solution;
impl Solution {
    pub fn min_anagram_length(s: String) -> i32 {
        let mut counts = [0; 26];
        for b in s.bytes() {
            counts[(b - b'a') as usize] += 1;
        }
        for &count in &counts {
            if count % 2 != 0 {
                return -1;
            }
        }
        counts.iter().filter(|&&c| c > 0).count() as i32
    }
}
