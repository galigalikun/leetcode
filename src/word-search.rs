fn main() {
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        ),
        true
    );
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "SEE".to_string()
        ),
        true
    );
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCB".to_string()
        ),
        false
    );
    assert_eq!(
        Solution::exist(
            vec![
                vec!['C', 'A', 'A'],
                vec!['A', 'A', 'A'],
                vec!['B', 'C', 'D']
            ],
            "AAB".to_string()
        ),
        true
    );
}

pub struct Solution {}
impl Solution {
    fn check(
        board: Vec<Vec<char>>,
        word: String,
        result: &mut Vec<(usize, usize)>,
        idx: usize,
        x: usize,
        y: usize,
    ) -> bool {
        if let Some(c) = word.as_str().chars().nth(idx) {
            let mut work = Vec::new();
            if board[y].len() > (x + 1)
                && None == result.iter().find(|(yy, xx)| yy == &y && xx == &(x + 1))
            {
                if c == board[y][x + 1] {
                    work.push((y, x + 1));
                }
            }
            if x > 0 && None == result.iter().find(|(yy, xx)| yy == &y && xx == &(x - 1)) {
                if c == board[y][x - 1] {
                    work.push((y, x - 1));
                }
            }
            if board.len() > (y + 1)
                && None == result.iter().find(|(yy, xx)| yy == &(y + 1) && xx == &(x))
            {
                if c == board[y + 1][x] {
                    work.push((y + 1, x));
                }
            }

            if y > 0 && None == result.iter().find(|(yy, xx)| yy == &(y - 1) && xx == &(x)) {
                if c == board[y - 1][x] {
                    work.push((y - 1, x));
                }
            }

            for (yy, xx) in work {
                result.push((yy, xx));
                if Solution::check(board.clone(), word.clone(), result, idx + 1, xx, yy) {
                    return true;
                }
                result.pop();
            }
        } else {
            return true;
        }

        return false;
    }
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        for y in 0..board.len() {
            for x in 0..board[y].len() {
                if let Some(c) = word.as_str().chars().nth(0) {
                    if c == board[y][x] {
                        if Solution::check(board.clone(), word.clone(), &mut vec![(y, x)], 1, x, y)
                        {
                            return true;
                        }
                    }
                }
            }
        }
        return false;
    }
}
