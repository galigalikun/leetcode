fn main() {
    assert_eq!(Solution::moves_to_make_zigzag(vec![1, 2, 3]), 2);
    assert_eq!(Solution::moves_to_make_zigzag(vec![9, 6, 1, 6, 2]), 4);
    assert_eq!(Solution::moves_to_make_zigzag(vec![2, 7, 10, 9, 8, 9]), 4);
}

struct Solution;
impl Solution {
    pub fn moves_to_make_zigzag(nums: Vec<i32>) -> i32 {
        let mut res = [0, 0];
        let n = nums.len();
        for i in 0..n {
            let left = if i > 0 { nums[i - 1] } else { 1001 };
            let right = if i + 1 < n { nums[i + 1] } else { 1001 };
            res[i % 2] += (nums[i] - left + 1).max(0);
            res[i % 2] += (nums[i] - right + 1).max(0);
        }
        return res[0].min(res[1]);
    }
}
