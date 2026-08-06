fn main() {
    assert_eq!(Solution::unique_letter_string("ABC".to_string()), 10);
    assert_eq!(Solution::unique_letter_string("ABA".to_string()), 8);
    assert_eq!(Solution::unique_letter_string("LEETCODE".to_string()), 92);
}

struct Solution{}
impl Solution {
    pub fn unique_letter_string(s: String) -> i32 {
        let bytes = s.as_bytes();
        let n = bytes.len() as i32;

        let mut prev = vec![-1_i32; 26];
        let mut prev_prev = vec![-1_i32; 26];
        let mut total = 0_i32;

        for (i, &b) in bytes.iter().enumerate() {
            let idx = (b - b'A') as usize;
            let i = i as i32;

            total += (i - prev[idx]) * (prev[idx] - prev_prev[idx]);
            prev_prev[idx] = prev[idx];
            prev[idx] = i;
        }

        for idx in 0..26 {
            total += (n - prev[idx]) * (prev[idx] - prev_prev[idx]);
        }

        total
    }
}
