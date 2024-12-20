fn main() {
    assert_eq!(
        Solution::get_sum_absolute_differences(vec![2, 3, 5]),
        vec![4, 3, 5]
    );
    assert_eq!(
        Solution::get_sum_absolute_differences(vec![1, 4, 6, 8, 10]),
        vec![24, 15, 13, 15, 21]
    );
}

struct Solution;
impl Solution {
    pub fn get_sum_absolute_differences(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut left_sum = vec![0; n];
        let mut right_sum = vec![0; n];
        let mut left = 0;
        let mut right = 0;
        for i in 0..n {
            left_sum[i] = left;
            left += nums[i];
            right_sum[n - i - 1] = right;
            right += nums[n - i - 1];
        }
        let mut res = vec![];
        for i in 0..n {
            res.push(
                (i as i32 * nums[i] - left_sum[i] + right_sum[i]
                    - (n as i32 - i as i32 - 1) * nums[i])
                    .abs(),
            );
        }
        res
    }
}
