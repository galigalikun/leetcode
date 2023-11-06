fn main() {
    assert_eq!(
        Solution::tictactoe(vec![
            vec![0, 0],
            vec![2, 0],
            vec![1, 1],
            vec![2, 1],
            vec![2, 2]
        ]),
        "A"
    );
    assert_eq!(
        Solution::tictactoe(vec![
            vec![0, 0],
            vec![1, 1],
            vec![0, 1],
            vec![0, 2],
            vec![1, 0],
            vec![2, 0]
        ]),
        "B"
    );
    assert_eq!(
        Solution::tictactoe(vec![
            vec![0, 0],
            vec![1, 1],
            vec![2, 0],
            vec![1, 0],
            vec![1, 2],
            vec![2, 1],
            vec![0, 1],
            vec![0, 2],
            vec![2, 2]
        ]),
        "Draw"
    );
}

struct Solution;
impl Solution {
    pub fn tictactoe(moves: Vec<Vec<i32>>) -> String {
        let mut board = vec![vec![0; 3]; 3];
        for (i, v) in moves.iter().enumerate() {
            board[v[0] as usize][v[1] as usize] = if i % 2 == 0 { 1 } else { -1 };
        }
        let mut sum = 0;
        for i in 0..3 {
            sum += board[i][i];
        }
        if sum == 3 {
            return "A".to_string();
        } else if sum == -3 {
            return "B".to_string();
        }
        sum = 0;
        for i in 0..3 {
            sum += board[i][2 - i];
        }
        if sum == 3 {
            return "A".to_string();
        } else if sum == -3 {
            return "B".to_string();
        }
        for i in 0..3 {
            sum = 0;
            for j in 0..3 {
                sum += board[i][j];
            }
            if sum == 3 {
                return "A".to_string();
            } else if sum == -3 {
                return "B".to_string();
            }
        }
        for i in 0..3 {
            sum = 0;
            for j in 0..3 {
                sum += board[j][i];
            }
            if sum == 3 {
                return "A".to_string();
            } else if sum == -3 {
                return "B".to_string();
            }
        }
        if moves.len() == 9 {
            return "Draw".to_string();
        }
        return "Pending".to_string();
    }
}
