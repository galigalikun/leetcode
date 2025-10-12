fn main() {
    assert_eq!(Solution::sum_of_beauties(vec![1, 2, 3]), 2);
    assert_eq!(Solution::sum_of_beauties(vec![2, 4, 6, 4]), 1);
    assert_eq!(Solution::sum_of_beauties(vec![3, 2, 1]), 0);
}

struct Solution;
impl Solution {
    pub fn sum_of_beauties(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        if n < 3 {
            return 0;
        }

        let mut left_max = vec![0; n];
        let mut right_min = vec![0; n];

        left_max[0] = nums[0];
        for i in 1..n {
            left_max[i] = left_max[i - 1].max(nums[i]);
        }
        right_min[n - 1] = nums[n - 1];
        for i in (0..n - 1).rev() {
            right_min[i] = right_min[i + 1].min(nums[i]);
        }

        let mut beauty_sum = 0;
        for i in 1..n - 1 {
            if left_max[i - 1] < nums[i] && nums[i] < right_min[i + 1] {
                beauty_sum += 1;
            }
        }

        beauty_sum
    }
}
