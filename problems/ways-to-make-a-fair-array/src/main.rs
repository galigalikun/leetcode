fn main() {
    assert_eq!(Solution::ways_to_make_fair(vec![2, 1, 6, 4]), 1);
    assert_eq!(Solution::ways_to_make_fair(vec![1, 1, 1]), 3);
    assert_eq!(Solution::ways_to_make_fair(vec![1, 2, 3]), 0);
}

struct Solution;
impl Solution {
    pub fn ways_to_make_fair(nums: Vec<i32>) -> i32 {
        let mut even_sum_left = 0;
        let mut odd_sum_left = 0;
        let mut even_sum_right = 0;
        let mut odd_sum_right = 0;
        let mut result = 0;
        for i in 0..nums.len() {
            if i % 2 == 0 {
                if even_sum_left + odd_sum_right == odd_sum_left + even_sum_right {
                    result += 1;
                }
                even_sum_left += nums[i];
            } else {
                if even_sum_left + odd_sum_right == odd_sum_left + even_sum_right {
                    result += 1;
                }
                odd_sum_left += nums[i];
            }
            if i % 2 == 0 {
                even_sum_right += nums[i];
            } else {
                odd_sum_right += nums[i];
            }
        }
        return result;
    }
}
