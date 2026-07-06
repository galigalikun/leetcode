fn main() {
    assert_eq!(Solution::is_ideal_permutation(vec![1, 0, 2]), true);
    assert_eq!(Solution::is_ideal_permutation(vec![1, 2, 0]), false);
}

struct Solution {}
impl Solution {
    pub fn is_ideal_permutation(nums: Vec<i32>) -> bool {
        // In an ideal permutation, every inversion must be local,
        // which is equivalent to requiring |nums[i] - i| <= 1 for all i.
        for (i, &value) in nums.iter().enumerate() {
            if (value - i as i32).abs() > 1 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_true() {
        assert!(Solution::is_ideal_permutation(vec![1, 0, 2]));
    }

    #[test]
    fn sample_false() {
        assert!(!Solution::is_ideal_permutation(vec![1, 2, 0]));
    }

    #[test]
    fn sorted_array_is_ideal() {
        assert!(Solution::is_ideal_permutation(vec![0, 1, 2, 3, 4]));
    }

    #[test]
    fn distant_inversion_is_not_ideal() {
        assert!(!Solution::is_ideal_permutation(vec![2, 0, 1]));
    }
}
