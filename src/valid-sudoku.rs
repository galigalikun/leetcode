fn main() {
    assert_eq!(
        Solution::is_valid_sudoku(vec![
            vec!['5', '3', '.', '.', '7', '.', '.', '.', '.'],
            vec!['6', '.', '.', '1', '9', '5', '.', '.', '.'],
            vec!['.', '9', '8', '.', '.', '.', '.', '6', '.'],
            vec!['8', '.', '.', '.', '6', '.', '.', '.', '3'],
            vec!['4', '.', '.', '8', '.', '3', '.', '.', '1'],
            vec!['7', '.', '.', '.', '2', '.', '.', '.', '6'],
            vec!['.', '6', '.', '.', '.', '.', '2', '8', '.'],
            vec!['.', '.', '.', '4', '1', '9', '.', '.', '5'],
            vec!['.', '.', '.', '.', '8', '.', '.', '7', '9']
        ]),
        true
    );
}

pub struct Solution {}
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut bb = Vec::new();
        bb.push(Vec::new());
        bb[0].push(5);
        bb[0].push(3);
        bb[0].push(1|2|4); // not 5,3,6,9,8,7
        bb[0].push(2|4|6|8); // not 5,3,7,1,9,5
        for x in 0..=9 {

            let mut num =
            for y in 0..=9 {

                let n = board[x][y] as i32 - 48;
                if n >0 && 10 > n {

                }
            }
        }
        return true;
    }
}
