fn main() {
    assert_eq!(Solution::ways_to_split(vec![1, 1, 1]), 1);
    assert_eq!(Solution::ways_to_split(vec![1, 2, 2, 2, 5, 0]), 3);
    assert_eq!(Solution::ways_to_split(vec![3, 2, 1]), 0);
    assert_eq!(Solution::ways_to_split(vec![0, 3, 3]), 1);
}

struct Solution;
impl Solution {
    pub fn ways_to_split(nums: Vec<i32>) -> i32 {
        let mut prefix_sum = vec![0; nums.len()];
        prefix_sum[0] = nums[0];
        for i in 1..nums.len() {
            prefix_sum[i] = prefix_sum[i - 1] + nums[i];
        }
        let mut ans = 0;
        let mut left = 0;
        let mut right = 0;
        for i in 0..nums.len() {
            left += nums[i];
            if left > prefix_sum[nums.len() - 1] / 3 {
                break;
            }
            while right < nums.len() - 1 && prefix_sum[right] < 2 * left {
                right += 1;
            }
            if prefix_sum[nums.len() - 1] - prefix_sum[right] >= left {
                ans += right - i;
            }
        }
        (ans % 1_000_000_007) as i32
    }
}
