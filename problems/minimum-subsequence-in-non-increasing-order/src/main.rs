fn main() {
    assert_eq!(Solution::min_subsequence(vec![4, 3, 10, 9, 8]), vec![10, 9]);
    assert_eq!(
        Solution::min_subsequence(vec![4, 4, 7, 6, 7]),
        vec![7, 7, 6]
    );
    assert_eq!(Solution::min_subsequence(vec![6]), vec![6]);
}

struct Solution;
impl Solution {
    pub fn min_subsequence(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        nums.sort_by(|a, b| b.cmp(a));
        let mut ans = vec![];
        for i in 0..nums.len() {
            let mut sum = 0;
            for j in 0..i {
                sum += nums[j];
            }
            let mut sum2 = 0;
            for j in i..nums.len() {
                sum2 += nums[j];
            }
            if sum > sum2 {
                for j in 0..i {
                    ans.push(nums[j]);
                }
                break;
            }
        }
        return ans;
    }
}
