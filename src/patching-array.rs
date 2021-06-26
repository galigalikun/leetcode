fn main() {
    assert_eq!(Solution::min_patches(vec![1, 3], 6), 1);
    assert_eq!(Solution::min_patches(vec![1, 5, 10], 20), 2);
    assert_eq!(Solution::min_patches(vec![1, 2, 2], 4), 0);
    assert_eq!(Solution::min_patches(vec![1, 2, 31, 33], 2147483647), 28);
}

pub struct Solution {}
// https://dreamume.medium.com/leetcode-330-patching-array-2477a76f40a0
impl Solution {
    pub fn min_patches(nums: Vec<i32>, n: i32) -> i32 {
        let mut miss:i64 = 1;
        let mut i = 0;
        let mut count = 0;
        while miss <= n as i64 {
            if i < nums.len() && nums[i] as i64 <= miss {
                miss += nums[i] as i64;
                i += 1;
            } else {
                miss += miss;
                count += 1;
            }
        }
        return count;
    }
}
