fn main() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1,-2,3,-2]), 3);
    assert_eq!(Solution::max_subarray_sum_circular(vec![5,-3,5]), 10);
    assert_eq!(Solution::max_subarray_sum_circular(vec![-3,-2,-3]), -2);
}

struct Solution;
impl Solution {
    pub fn max_subarray_sum_circular(nums: Vec<i32>) -> i32 {
        let mut total_sum = 0;

        let mut current_max = 0;
        let mut max_sum = i32::MIN;

        let mut current_min = 0;
        let mut min_sum = i32::MAX;

        for num in nums {
            total_sum += num;

            current_max = (current_max + num).max(num);
            max_sum = max_sum.max(current_max);

            current_min = (current_min + num).min(num);
            min_sum = min_sum.min(current_min);
        }

        if max_sum < 0 {
            max_sum
        } else {
            max_sum.max(total_sum - min_sum)
        }
    }
}
