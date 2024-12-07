fn main() {
    assert_eq!(Solution::minimum_mountain_removals(vec![1, 3, 1]), 0);
    assert_eq!(
        Solution::minimum_mountain_removals(vec![2, 1, 1, 5, 6, 2, 3, 1]),
        3
    );
}

struct Solution;
impl Solution {
    pub fn minimum_mountain_removals(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut lis = vec![1; n];
        let mut lds = vec![1; n];
        for i in 0..n {
            for j in 0..i {
                if nums[i] > nums[j] {
                    lis[i] = lis[i].max(lis[j] + 1);
                }
            }
        }
        for i in (0..n).rev() {
            for j in (i + 1..n).rev() {
                if nums[i] > nums[j] {
                    lds[i] = lds[i].max(lds[j] + 1);
                }
            }
        }
        let mut ans = 0;
        for i in 0..n {
            if lis[i] > 1 && lds[i] > 1 {
                ans = ans.max(lis[i] + lds[i] - 1);
            }
        }
        (n - ans) as i32
    }
}
