fn main() {
    assert_eq!(Solution::partition_labels("ababcbacadefegdehijhklij".to_string()), vec![9,7,8]);
    assert_eq!(Solution::partition_labels("eccbbbbdec".to_string()), vec![10]);
}

struct Solution{}
impl Solution {
    pub fn partition_labels(s: String) -> Vec<i32> {
        let bytes = s.as_bytes();

        // Record the last index at which each letter appears.
        let mut last = [0usize; 26];
        for (i, &b) in bytes.iter().enumerate() {
            last[(b - b'a') as usize] = i;
        }

        // Greedily extend each partition to the farthest last-occurrence
        // of any letter seen so far, then cut when the window closes.
        let mut result = Vec::new();
        let mut start = 0;
        let mut end = 0;
        for (i, &b) in bytes.iter().enumerate() {
            end = end.max(last[(b - b'a') as usize]);
            if i == end {
                result.push((end - start + 1) as i32);
                start = i + 1;
            }
        }
        result
    }
}
