fn main() {
    assert_eq!(Solution::min_time_to_type("abc".to_string()), 5);
    assert_eq!(Solution::min_time_to_type("bza".to_string()), 7);
    assert_eq!(Solution::min_time_to_type("zjpc".to_string()), 34);
}

struct Solution;
impl Solution {
    pub fn min_time_to_type(word: String) -> i32 {
        return (word.len() - 1) as i32
        + word
                .as_bytes()
                .windows(2)
                .map(|w| {
                    let diff = (w[0] as i32 - w[1] as i32).abs();
                    diff.min(26 - diff)
                })
                .sum::<i32>()
            + 1;
    }
}
