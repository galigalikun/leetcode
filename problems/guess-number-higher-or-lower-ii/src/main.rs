fn main() {
    assert_eq!(Solution::get_money_amount(10), 16);
    assert_eq!(Solution::get_money_amount(1), 0);
    assert_eq!(Solution::get_money_amount(2), 1);
}

struct Solution {}
// https://hbisheng.gitbooks.io/leetcode-google/content/375-guess-number-higher-or-lower-ii.html
impl Solution {
    fn get_money(start: usize, end: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if start >= end {
            return 0;
        }
        if end - start == 1 {
            return start as i32;
        }
        if dp[start][end] != 0 {
            return dp[start][end];
        }
        let mut min = std::i32::MAX;
        for i in start..=end {
            let mut cost = i as i32;
            cost += std::cmp::max(
                Solution::get_money(start, i - 1, dp),
                Solution::get_money(i + 1, end, dp),
            );
            min = std::cmp::min(cost, min);
        }
        dp[start][end] = min;
        return min;
    }
    pub fn get_money_amount(n: i32) -> i32 {
        if n <= 1 {
            return 0;
        }
        let mut dp = vec![vec![0 as i32; n as usize + 1]; n as usize + 1];
        return Solution::get_money(1, n as usize, &mut dp);
    }
}
