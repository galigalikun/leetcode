fn main() {
    assert_eq!(Solution::min_increment_for_unique(vec![1,2,2]), 1);
    assert_eq!(Solution::min_increment_for_unique(vec![3,2,1,2,1,7]), 6);
}

struct Solution;
impl Solution {
    pub fn min_increment_for_unique(nums: Vec<i32>) -> i32 {
        let mut sorted_nums = nums;
        sorted_nums.sort_unstable();

        let mut moves = 0;
        let mut min_allowed = i32::MIN;

        for value in sorted_nums {
            let target = value.max(min_allowed);
            moves += target - value;
            min_allowed = target + 1;
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_zero_for_empty_input() {
        assert_eq!(Solution::min_increment_for_unique(vec![]), 0);
    }

    #[test]
    fn handles_sample_cases() {
        assert_eq!(Solution::min_increment_for_unique(vec![1, 2, 2]), 1);
        assert_eq!(Solution::min_increment_for_unique(vec![3, 2, 1, 2, 1, 7]), 6);
    }

    #[test]
    fn handles_all_duplicates() {
        assert_eq!(Solution::min_increment_for_unique(vec![0, 0, 0, 0]), 6);
    }
}
