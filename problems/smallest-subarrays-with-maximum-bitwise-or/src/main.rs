fn main() {
    assert_eq!(
        Solution::smallest_subarrays(vec![1, 0, 2, 1, 3]),
        vec![3, 3, 2, 2, 1]
    );
    assert_eq!(Solution::smallest_subarrays(vec![1, 2]), vec![2, 1]);
}

struct Solution;
impl Solution {
    pub fn smallest_subarrays(nums: Vec<i32>) -> Vec<i32> {
        let n = nums.len();
        let mut ans = vec![1; n];
        let mut last = vec![-1; 32];
        for i in (0..n).rev() {
            for j in 0..32 {
                if (nums[i] >> j) & 1 == 1 {
                    last[j] = i as i32;
                }
            }
            let mut max_index = i as i32;
            for &idx in &last {
                if idx != -1 {
                    max_index = max_index.max(idx);
                }
            }
            ans[i] = (max_index - i as i32 + 1) as i32;
        }
        ans
    }
}
