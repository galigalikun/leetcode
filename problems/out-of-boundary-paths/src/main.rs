fn main() {
    assert_eq!(Solution::find_paths(2, 2, 2, 0, 0), 6);
    assert_eq!(Solution::find_paths(1, 3, 3, 0, 1), 12);
    assert_eq!(Solution::find_paths(8, 50, 23, 5, 26), 914783380);
}

struct Solution {}
impl Solution {
    pub fn find_paths(m: i32, n: i32, max_move: i32, start_row: i32, start_column: i32) -> i32 {
        if max_move == 0 {
            return 0;
        }
        let mut dp_c = vec![vec![0 as i64; n as usize + 2]; m as usize + 2];
        let mut dp_l = vec![vec![0 as i64; n as usize + 2]; m as usize + 2];
        for i in 1..=m as usize {
            dp_c[i][1] += 1;
            dp_c[i][n as usize] += 1;
        }
        for i in 1..=n as usize {
            dp_c[1][i] += 1;
            dp_c[m as usize][i] += 1;
        }
        let mut ans = dp_c[start_row as usize + 1][start_column as usize + 1];
        for _d in 1..max_move {
            let s = dp_c;
            dp_c = dp_l;
            dp_l = s;
            for i in 1..=m as usize {
                for j in 1..=n as usize {
                    dp_c[i][j] =
                        (dp_l[i - 1][j] + dp_l[i + 1][j] + dp_l[i][j - 1] + dp_l[i][j + 1])
                            % 1000000007;
                }
            }
            ans = (ans + dp_c[start_row as usize + 1][start_column as usize + 1]) % 1000000007;
        }
        return ans as i32;
    }
}
