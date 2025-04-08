fn main() {
    assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 5, 10, 50]), 65);
    assert_eq!(Solution::max_ascending_sum(vec![10, 20, 30, 40, 50]), 150);
    assert_eq!(
        Solution::max_ascending_sum(vec![12, 17, 15, 13, 10, 11, 12]),
        33
    );
}

struct Solution;
impl Solution {
    pub fn max_ascending_sum(nums: Vec<i32>) -> i32 {
        let mut result = vec![];
        let mut sum = 0;
        for i in 0..nums.len() {
            if i == 0 || nums[i] > nums[i - 1] {
                sum += nums[i];
            } else {
                result.push(sum);
                sum = nums[i];
            }
        }
        result.push(sum);
        result.sort();
        result.reverse();

        if result.len() > 0 {
            return result[0];
        } else {
            return 0;
        }
    }
}
