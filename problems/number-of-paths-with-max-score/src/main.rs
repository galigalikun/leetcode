fn main() {
    assert_eq!(
        Solution::paths_with_max_score(vec![
            "E23".to_string(),
            "2X2".to_string(),
            "12S".to_string()
        ]),
        vec![7, 1]
    );
    assert_eq!(
        Solution::paths_with_max_score(vec![
            "E12".to_string(),
            "1X1".to_string(),
            "21S".to_string()
        ]),
        vec![4, 2]
    );
    assert_eq!(
        Solution::paths_with_max_score(vec![
            "E11".to_string(),
            "XXX".to_string(),
            "11S".to_string()
        ]),
        vec![0, 0]
    );
}

struct Solution;
impl Solution {
    pub fn paths_with_max_score(board: Vec<String>) -> Vec<i32> {
        let mut board = board;
        let mut dp = vec![vec![vec![0; 2]; board[0].len()]; board.len()];
        let mut mod_num = 1000000007;
        let mut dirs = vec![(0, -1), (-1, 0), (-1, -1)];
        for i in (0..board.len()).rev() {
            for j in (0..board[0].len()).rev() {
                if board[i].chars().nth(j).unwrap() == 'X' {
                    continue;
                }
                for (dx, dy) in &dirs {
                    let x = (i as i32 + dx) as usize;
                    let y = (j as i32 + dy) as usize;
                    if x >= board.len()
                        || y >= board[0].len()
                        || board[x].chars().nth(y).unwrap() == 'X'
                    {
                        continue;
                    }
                    let mut score = dp[x][y][0];
                    if i == board.len() - 1 && j == board[0].len() - 1 {
                        score = 0;
                    }
                    let mut sum = dp[i][j][0] + score;
                    if board[i].chars().nth(j).unwrap() != 'S' {
                        sum += board[i].chars().nth(j).unwrap() as i32 - '0' as i32;
                    }
                    if sum > dp[i][j][0] {
                        dp[i][j][0] = sum;
                        dp[i][j][1] = dp[x][y][1];
                    } else if sum == dp[i][j][0] {
                        dp[i][j][1] = (dp[i][j][1] + dp[x][y][1]) % mod_num;
                    }
                }
            }
        }
        if dp[0][0][1] > 0 {
            return dp[0][0].to_vec();
        }
        return vec![];
    }
}
