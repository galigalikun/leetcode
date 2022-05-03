fn main() {
    assert_eq!(Solution::knight_probability(3, 2, 0, 0), 0.06250);
    assert_eq!(Solution::knight_probability(1, 0, 0, 0), 1.00000);
}

struct Solution {}
impl Solution {
    pub fn knight_probability(n: i32, k: i32, row: i32, column: i32) -> f64 {
        let mut dp = vec![vec![0.0; n as usize]; n as usize];
        dp[row as usize][column as usize] = 1.0;
        for _ in 0..k {
            let mut tmp = vec![vec![0.0; n as usize]; n as usize];
            for i in 0..n {
                for j in 0..n {
                    for (x, y) in vec![
                        (i - 2, j - 1),
                        (i - 2, j + 1),
                        (i + 2, j - 1),
                        (i + 2, j + 1),
                        (i - 1, j - 2),
                        (i - 1, j + 2),
                        (i + 1, j - 2),
                        (i + 1, j + 2),
                    ] {
                        if x >= 0 && x < n && y >= 0 && y < n {
                            tmp[x as usize][y as usize] += dp[i as usize][j as usize] / 8.0;
                        }
                    }
                }
            }
            dp = tmp;
        }
        return dp
            .iter()
            .fold(0.0, |acc, x| acc + x.iter().fold(0.0, |acc, x| acc + *x));
    }
}
