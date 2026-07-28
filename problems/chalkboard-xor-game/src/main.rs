fn main() {
    assert_eq!(Solution::xor_game(vec![1, 1, 2]), false);
    assert_eq!(Solution::xor_game(vec![0, 1]), true);
    assert_eq!(Solution::xor_game(vec![1, 2, 3]), true);
}

struct Solution {}

impl Solution {
    pub fn xor_game(nums: Vec<i32>) -> bool {
        if nums.len() % 2 == 0 {
            return true;
        }

        nums.iter().fold(0, |acc, &value| acc ^ value) == 0
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn odd_length_and_non_zero_xor_loses_for_alice() {
        assert!(!Solution::xor_game(vec![1, 1, 2]));
    }

    #[test]
    fn even_length_always_wins_for_alice() {
        assert!(Solution::xor_game(vec![2, 4]));
        assert!(Solution::xor_game(vec![5, 5, 7, 9]));
    }

    #[test]
    fn zero_xor_at_start_wins_for_current_player() {
        assert!(Solution::xor_game(vec![1, 2, 3]));
        assert!(Solution::xor_game(vec![0, 0, 0]));
    }
}
