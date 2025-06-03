fn main() {
    assert_eq!(Solution::rearrange_sticks(3, 2), 3);
    assert_eq!(Solution::rearrange_sticks(5, 5), 1);
    assert_eq!(Solution::rearrange_sticks(20, 11), 647427950);
}

struct Solution;
impl Solution {
    fn rearrange_sticks_helper(n: usize, k: usize, memo: &mut Vec<Vec<i32>>) -> i32 {
        if k > n {
            return 0;
        }
        if k == 0 || n == 0 {
            return if k == 0 { 1 } else { 0 };
        }
        if memo[n][k] != -1 {
            return memo[n][k];
        }

        let mut result = 0;
        // Case where the nth stick is not placed
        result += Self::rearrange_sticks_helper(n - 1, k, memo);
        // Case where the nth stick is placed
        result += (n as i32) * Self::rearrange_sticks_helper(n - 1, k - 1, memo);

        memo[n][k] = result;
        result
    }
    pub fn rearrange_sticks(n: i32, k: i32) -> i32 {
        return Self::rearrange_sticks_helper(
            n as usize,
            k as usize,
            &mut vec![vec![-1; k as usize + 1]; n as usize + 1],
        );
    }
}
