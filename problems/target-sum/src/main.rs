fn main() {
    assert_eq!(Solution::find_target_sum_ways(vec![1, 1, 1, 1, 1], 3), 5);
    assert_eq!(Solution::find_target_sum_ways(vec![1], 1), 1);
    assert_eq!(Solution::find_target_sum_ways(vec![1], 2), 0);
    assert_eq!(Solution::find_target_sum_ways(vec![47,25,32,10,1,1,36,19,46,41,11,32,21,41,44,8,33,5,47,34], 10), 5855);
}

struct Solution {}
// https://twchen.gitbook.io/leetcode/dynamic-programming/target-sum
impl Solution {
    fn helper(nums: Vec<i32>, i: usize, target: i32) -> i32 {
        if target < 0 {
            return 0;
        }
        if i == nums.len() {
            if target == 0 {
                return 1;
            } else {
                return 0;
            }
        }
        return Solution::helper(nums.clone(), i + 1, target)
            + Solution::helper(nums.clone(), i + 1, target - nums[i]);
    }
    pub fn find_target_sum_ways(nums: Vec<i32>, target: i32) -> i32 {
        let sum = nums.iter().fold(0, |sum, a| sum + a);
        if sum < target || (sum - target) % 2 == 1 {
            return 0;
        }
        return Solution::helper(nums, 0, (sum - target) / 2);
    }
}
