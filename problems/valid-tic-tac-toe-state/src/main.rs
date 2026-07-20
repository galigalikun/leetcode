fn main() {
    assert_eq!(Solution::valid_tic_tac_toe(vec!["O  ".to_string(),"   ".to_string(),"   ".to_string()]), false);
    assert_eq!(Solution::valid_tic_tac_toe(vec!["XOX".to_string()," X ".to_string(),"   ".to_string()]), false);
    assert_eq!(Solution::valid_tic_tac_toe(vec!["XOX".to_string(),"O O".to_string(),"XOX".to_string()]), true);
}

struct Solution{}
impl Solution {
    pub fn valid_tic_tac_toe(board: Vec<String>) -> bool {
        let grid: Vec<Vec<u8>> = board.iter().map(|row| row.as_bytes().to_vec()).collect();

        let mut x_count = 0;
        let mut o_count = 0;

        for row in &grid {
            for &cell in row {
                if cell == b'X' {
                    x_count += 1;
                } else if cell == b'O' {
                    o_count += 1;
                }
            }
        }

        if o_count > x_count || x_count > o_count + 1 {
            return false;
        }

        let is_winner = |player: u8| -> bool {
            for i in 0..3 {
                if grid[i][0] == player && grid[i][1] == player && grid[i][2] == player {
                    return true;
                }
                if grid[0][i] == player && grid[1][i] == player && grid[2][i] == player {
                    return true;
                }
            }

            (grid[0][0] == player && grid[1][1] == player && grid[2][2] == player)
                || (grid[0][2] == player && grid[1][1] == player && grid[2][0] == player)
        };

        let x_wins = is_winner(b'X');
        let o_wins = is_winner(b'O');

        if x_wins && o_wins {
            return false;
        }
        if x_wins {
            return x_count == o_count + 1;
        }
        if o_wins {
            return x_count == o_count;
        }

        true
    }
}
