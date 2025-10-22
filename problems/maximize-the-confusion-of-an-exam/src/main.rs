fn main() {
    assert_eq!(Solution::max_consecutive_answers("TTFF".to_string(), 2), 4);
    assert_eq!(Solution::max_consecutive_answers("TFFT".to_string(), 1), 3);
    assert_eq!(
        Solution::max_consecutive_answers("TTFTTFTT".to_string(), 1),
        5
    );
}

struct Solution;
impl Solution {
    pub fn max_consecutive_answers(answer_key: String, k: i32) -> i32 {
        let mut max_len = 0;
        let mut left = 0;
        let mut count = 0;

        for (right, c) in answer_key.chars().enumerate() {
            if c == 'T' {
                count += 1;
            }
            while count > k {
                if answer_key.chars().nth(left).unwrap() == 'T' {
                    count -= 1;
                }
                left += 1;
            }
            max_len = max_len.max(right - left + 1);
        }

        max_len as i32
    }
}
