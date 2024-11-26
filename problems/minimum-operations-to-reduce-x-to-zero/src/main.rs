fn main() {
    assert_eq!(Solution::min_operations(vec![1, 1, 4, 2, 3], 5), 2);
    assert_eq!(Solution::min_operations(vec![5, 6, 7, 8, 9], 4), -1);
    assert_eq!(Solution::min_operations(vec![3, 2, 20, 1, 1, 3], 10), 5);
}

struct Solution;
impl Solution {
    pub fn min_operations(nums: Vec<i32>, x: i32) -> i32 {
        let mut sum = 0;
        for &num in &nums {
            sum += num;
        }
        let target = sum - x;
        let mut left = 0;
        let mut right = 0;
        sum = 0;
        let mut max_len = -1;
        while right < nums.len() {
            sum += nums[right];
            while sum > target && left <= right {
                sum -= nums[left];
                left += 1;
            }
            if sum == target {
                max_len = max_len.max(right as i32 - left as i32 + 1);
            }
            right += 1;
        }
        if max_len != -1 {
            return nums.len() as i32 - max_len;
        }
        return 0;
    }
}
