fn main() {
    assert_eq!(Solution::ways_to_partition(vec![2, -1, 2], 3), 1);
    assert_eq!(Solution::ways_to_partition(vec![0, 0, 0], 1), 2);
    assert_eq!(
        Solution::ways_to_partition(
            vec![22, 4, -25, -20, -15, 15, -16, 7, 19, -10, 0, -13, -14],
            -33
        ),
        4
    );
}

struct Solution;
impl Solution {
    pub fn ways_to_partition(nums: Vec<i32>, k: i32) -> i32 {
        let mut left_sum = 0;
        let mut right_sum = nums.iter().sum::<i32>();
        let mut result = 0;

        for i in 0..nums.len() {
            right_sum -= nums[i];
            if left_sum == right_sum {
                result += 1;
            }
            left_sum += nums[i];
        }

        left_sum = 0;
        right_sum = nums.iter().sum::<i32>();

        for i in 0..nums.len() {
            right_sum -= nums[i];
            if left_sum == right_sum {
                result += 1;
            }
            left_sum += nums[i];
        }

        result
    }
}
