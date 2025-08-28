fn main() {
    assert_eq!(Solution::min_space_wasted_k_resizing(vec![10, 20], 0), 10);
    assert_eq!(
        Solution::min_space_wasted_k_resizing(vec![10, 20, 30], 1),
        10
    );
    assert_eq!(
        Solution::min_space_wasted_k_resizing(vec![10, 20, 15, 30, 20], 2),
        15
    );
}

struct Solution;
impl Solution {
    pub fn min_space_wasted_k_resizing(nums: Vec<i32>, k: i32) -> i32 {
        let n = nums.len();
        for i in 0..n {
            for j in i..n {
                let sum: i32 = nums[i..=j].iter().sum();
                let max_val = *nums[i..=j].iter().max().unwrap();
                let wasted = (j - i + 1) as i32 * max_val - sum;
                if wasted <= k {
                    return wasted;
                }
            }
        }
        return 0;
    }
}
