fn main() {
    assert_eq!(Solution::min_start_value(vec![-3, 2, -3, 4, 2]), 5);
    assert_eq!(Solution::min_start_value(vec![1, 2]), 1);
    assert_eq!(Solution::min_start_value(vec![1, -2, -3]), 5);
}

struct Solution;
impl Solution {
    pub fn min_start_value(nums: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut min = 0;
        for num in nums {
            sum += num;
            min = min.min(sum);
        }
        if min < 0 {
            return 1 - min;
        }
        return 1;
    }
}
