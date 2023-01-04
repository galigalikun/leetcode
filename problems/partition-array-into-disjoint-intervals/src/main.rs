fn main() {
    assert_eq!(Solution::partition_disjoint(vec![5, 0, 3, 8, 6]), 3);
    assert_eq!(Solution::partition_disjoint(vec![1, 1, 1, 0, 6, 12]), 4);
    assert_eq!(Solution::partition_disjoint(vec![1, 1]), 1);
}

struct Solution;
impl Solution {
    pub fn partition_disjoint(nums: Vec<i32>) -> i32 {
        let mut m = nums[0];
        let mut ans = vec![nums[0]];
        for i in 1..nums.len() {
            if nums[i] > m {
                break;
            }
            m = std::cmp::max(m, nums[i]);
            ans.push(nums[i]);
        }
        return ans.len() as i32;
    }
}
