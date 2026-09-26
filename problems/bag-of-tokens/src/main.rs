fn main() {
    assert_eq!(Solution::bag_of_tokens_score(vec![100], 50), 0);
    assert_eq!(Solution::bag_of_tokens_score(vec![100,200], 150), 1);
    assert_eq!(Solution::bag_of_tokens_score(vec![100,200,300,400], 200), 2);
}

struct Solution;
impl Solution {
    pub fn bag_of_tokens_score(mut tokens: Vec<i32>, power: i32) -> i32 {
        if tokens.is_empty() {
            return 0;
        }

        tokens.sort_unstable();

        let mut left = 0usize;
        let mut right = tokens.len().saturating_sub(1);
        let mut current_power = power;
        let mut score = 0;
        let mut best = 0;

        while left <= right && (current_power >= tokens[left] || score > 0) {
            while left <= right && current_power >= tokens[left] {
                current_power -= tokens[left];
                score += 1;
                best = best.max(score);
                left += 1;
            }

            if left <= right && score > 0 {
                current_power += tokens[right];
                score -= 1;
                right = right.saturating_sub(1);
            }
        }

        best
    }
}
