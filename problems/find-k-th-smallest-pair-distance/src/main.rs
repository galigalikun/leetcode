fn main() {
    assert_eq!(Solution::smallest_distance_pair(vec![1, 3, 1], 1), 0);
    assert_eq!(Solution::smallest_distance_pair(vec![1, 1, 1], 2), 0);
    assert_eq!(Solution::smallest_distance_pair(vec![1, 6, 1], 3), 5);
}

struct Solution {}
impl Solution {
    pub fn smallest_distance_pair(nums: Vec<i32>, k: i32) -> i32 {
        let mut nums = nums;
        nums.sort();
        let mut i = 0;
        let mut j = nums.len() - 1;
        let mut min = std::i32::MAX;
        while i < j {
            let sum = nums[i] + nums[j];
            if sum == k {
                min = std::cmp::min(min, nums[j] - nums[i]);
                i += 1;
                j -= 1;
            } else if sum < k {
                i += 1;
            } else {
                j -= 1;
            }
        }
        return if min == std::i32::MAX { 0 } else { min };
    }
}
