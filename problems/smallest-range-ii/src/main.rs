fn main() {
    assert_eq!(Solution::smallest_range_ii(vec![1], 0), 0);
    assert_eq!(Solution::smallest_range_ii(vec![0,10], 2), 6);
    assert_eq!(Solution::smallest_range_ii(vec![1,3,6], 3), 3);
}

struct Solution;
impl Solution {
    pub fn smallest_range_ii(nums: Vec<i32>, k: i32) -> i32 {
        if nums.len() <= 1 {
            return 0;
        }

        let mut sorted = nums;
        sorted.sort_unstable();

        let n = sorted.len();
        let mut answer = sorted[n - 1] - sorted[0];
        let base_min = sorted[0] + k;
        let base_max = sorted[n - 1] - k;

        for i in 0..(n - 1) {
            let high = base_max.max(sorted[i] + k);
            let low = base_min.min(sorted[i + 1] - k);
            answer = answer.min(high - low);
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn example_cases() {
        assert_eq!(Solution::smallest_range_ii(vec![1], 0), 0);
        assert_eq!(Solution::smallest_range_ii(vec![0, 10], 2), 6);
        assert_eq!(Solution::smallest_range_ii(vec![1, 3, 6], 3), 3);
    }

    #[test]
    fn additional_cases() {
        assert_eq!(Solution::smallest_range_ii(vec![7, 8, 8], 5), 1);
        assert_eq!(Solution::smallest_range_ii(vec![2, 7, 2], 1), 3);
    }
}
