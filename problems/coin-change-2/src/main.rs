fn main() {
    assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
    assert_eq!(Solution::change(3, vec![2]), 0);
    assert_eq!(Solution::change(10, vec![10]), 1);
}

struct Solution {}
// https://aaronice.gitbook.io/lintcode/knapsack_problems/coin-change-ii
impl Solution {
    pub fn change(amount: i32, coins: Vec<i32>) -> i32 {
        let n = coins.len();
        let mut f = vec![vec![0; amount as usize + 1]; n + 1];
        f[0][0] = 1;
        for i in 1..=n {
            for j in 0..=amount {
                let mut k = 0;
                loop {
                    let s = k * coins[i - 1];
                    if s > j {
                        break;
                    }
                    f[i][j as usize] += f[i - 1][(j - s) as usize];
                    k += 1;
                }
            }
        }
        return f[n as usize][amount as usize];
    }
}
