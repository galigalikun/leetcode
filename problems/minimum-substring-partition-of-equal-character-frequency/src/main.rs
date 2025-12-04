fn main() {
    assert_eq!(
        Solution::minimum_substrings_in_partition("fabccddg".to_string()),
        3
    );
    assert_eq!(
        Solution::minimum_substrings_in_partition("abababaccddb".to_string()),
        2
    );
}

struct Solution;
impl Solution {
    pub fn minimum_substrings_in_partition(s: String) -> i32 {
        let mut last_seen = [0; 26];
        let mut count = 0;
        let mut start = 0;
        for (i, ch) in s.chars().enumerate() {
            let idx = (ch as u8 - b'a') as usize;
            if last_seen[idx] >= start {
                count += 1;
                start = i;
            }
            last_seen[idx] = i;
        }

        return count + 1;
    }
}
