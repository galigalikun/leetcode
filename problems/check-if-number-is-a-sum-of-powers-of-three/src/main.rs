fn main() {
    assert_eq!(Solution::check_powers_of_three(12), true);
    assert_eq!(Solution::check_powers_of_three(91), true);
    assert_eq!(Solution::check_powers_of_three(21), false);
}

struct Solution;
impl Solution {
    fn dfs(n: i128, i: i32) -> bool {
        if n == 0 {
            return true;
        }
        if n < 0 {
            return false;
        }
        return Self::dfs(n - 3_i128.pow(i as u32), i + 1) || Self::dfs(n, i + 1);
    }
    pub fn check_powers_of_three(n: i32) -> bool {
        return Self::dfs(n as i128, 0);
    }
}
