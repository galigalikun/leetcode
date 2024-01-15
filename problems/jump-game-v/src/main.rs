fn main() {
    assert_eq!(Solution::max_jumps(vec![6,4,14,6,8,13,9,7,10,6,12], 2), 4);
    assert_eq!(Solution::max_jumps(vec![3,3,3,3,3], 3), 1);
    assert_eq!(Solution::max_jumps(vec![7,6,5,4,3,2,1], 1), 7);
}

struct Solution;
impl Solution {
    pub fn max_jumps(arr: Vec<i32>, d: i32) -> i32 {
        let mut dp = vec![0; arr.len()];
        let mut stack = vec![];
        for i in 0..arr.len() {
            while let Some(&j) = stack.last() {
                if arr[j] < arr[i] {
                    let a:i32 = dp[j] + 1;
                    dp[j] = a.max(dp[i]);
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        let mut stack = vec![];
        for i in (0..arr.len()).rev() {
            while let Some(&j) = stack.last() {
                if arr[j] < arr[i] {
                    let a:i32 = dp[j] + 1;
                    dp[j] = a.max(dp[i]);
                    stack.pop();
                } else {
                    break;
                }
            }
            stack.push(i);
        }
        return dp.into_iter().max().unwrap() + 1;
    }
}
