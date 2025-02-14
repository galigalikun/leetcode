fn main() {
    assert_eq!(Solution::max_absolute_sum(vec![1, -3, 2, 3, -4]), 5);
    assert_eq!(Solution::max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8);
}

struct Solution;
impl Solution {
    pub fn max_absolute_sum(nums: Vec<i32>) -> i32 {
        let mut max_sum = 0;
        let mut min_sum = 0;
        let mut max = 0;
        let mut min = 0;
        for num in nums {
            max = std::cmp::max(max + num, num);
            min = std::cmp::min(min + num, num);
            max_sum = std::cmp::max(max_sum, max);
            min_sum = std::cmp::min(min_sum, min);
        }
        if max_sum > -min_sum {
            return max_sum;
        }
        return -min_sum;
    }
}
