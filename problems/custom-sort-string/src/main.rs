fn main() {
    assert_eq!(Solution::custom_sort_string("cba".to_string(), "abcd".to_string()), "cbad");
    assert_eq!(Solution::custom_sort_string("cbafg".to_string(), "abcd".to_string()), "cbad");
}

struct Solution {}

impl Solution {
    pub fn custom_sort_string(order: String, s: String) -> String {
        let mut counts = [0_usize; 26];

        for &b in s.as_bytes() {
            let idx = (b - b'a') as usize;
            counts[idx] += 1;
        }

        let mut result = String::with_capacity(s.len());

        for &b in order.as_bytes() {
            let idx = (b - b'a') as usize;
            let count = counts[idx];

            for _ in 0..count {
                result.push(b as char);
            }

            counts[idx] = 0;
        }

        for (idx, &count) in counts.iter().enumerate() {
            for _ in 0..count {
                result.push((b'a' + idx as u8) as char);
            }
        }

        result
    }
}
