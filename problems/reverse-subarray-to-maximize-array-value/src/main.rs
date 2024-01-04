fn main() {
    assert_eq!(Solution::max_value_after_reverse(vec![2, 3, 1, 5, 4]), 10);
    assert_eq!(
        Solution::max_value_after_reverse(vec![2, 4, 9, 24, 2, 1, 10]),
        68
    );
}

struct Solution;
impl Solution {
    fn reverse(nums: &Vec<i32>, i: usize, j: usize) -> Vec<i32> {
        let mut res = vec![];
        for k in 0..i {
            res.push(nums[k]);
        }
        for k in (i..j + 1).rev() {
            res.push(nums[k]);
        }
        for k in j + 1..nums.len() {
            res.push(nums[k]);
        }
        return res;
    }
    pub fn max_value_after_reverse(nums: Vec<i32>) -> i32 {
        let mut res = 0;
        for i in 0..nums.len() {
            for j in i..nums.len() {
                let a = Solution::reverse(&nums, i, j);
                let mut sum = 0;
                for k in 0..a.len() - 1 {
                    sum += (a[k] - a[k + 1]).abs();
                }
                res = std::cmp::max(res, sum);
            }
        }
        return res;
    }
}
