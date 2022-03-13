fn main() {
    assert_eq!(
        Solution::solve_n_queens(4),
        vec![
            ["..Q.", "Q...", "...Q", ".Q.."],
            [".Q..", "...Q", "Q...", "..Q."]
        ]
    );

    assert_eq!(Solution::solve_n_queens(1), vec![["Q"]]);

    assert_eq!(
        Solution::solve_n_queens(5),
        vec![
            ["....Q", "..Q..", "Q....", "...Q.", ".Q..."],
            ["....Q", ".Q...", "...Q.", "Q....", "..Q.."],
            ["...Q.", ".Q...", "....Q", "..Q..", "Q...."],
            ["...Q.", "Q....", "..Q..", "....Q", ".Q..."],
            ["..Q..", "....Q", ".Q...", "...Q.", "Q...."],
            ["..Q..", "Q....", "...Q.", ".Q...", "....Q"],
            [".Q...", "....Q", "..Q..", "Q....", "...Q."],
            [".Q...", "...Q.", "Q....", "..Q..", "....Q"],
            ["Q....", "...Q.", ".Q...", "....Q", "..Q.."],
            ["Q....", "..Q..", "....Q", ".Q...", "...Q."]
        ]
    );
}

struct Solution {}
// http://www.ic-net.or.jp/home/takaken/nt/queen/
use std::collections::HashMap;
impl Solution {
    fn helper(
        n: i32,
        result: &mut Vec<Vec<String>>,
        work: &mut HashMap<i32, String>,
        y: i32,
        left: i32,
        down: i32,
        right: i32,
    ) {
        if y == n {
            let mut list = vec![];
            for i in 0..n {
                list.push(work[&i].clone());
            }
            result.push(list);
        } else {
            let mut bitmap = ((1 << n) - 1) & !(left | down | right);
            while bitmap != 0 {
                let bit = -bitmap & bitmap;
                work.insert(
                    y,
                    format!("{:0>1$b}", bit, n as usize)
                        .chars()
                        .map(|x| if x == '0' { '.' } else { 'Q' })
                        .collect::<String>(),
                );
                bitmap ^= bit;
                Solution::helper(
                    n,
                    result,
                    work,
                    y + 1,
                    (left | bit) << 1,
                    down | bit,
                    (right | bit) >> 1,
                );
            }
        }
    }
    pub fn solve_n_queens(n: i32) -> Vec<Vec<String>> {
        let mut result: Vec<Vec<String>> = vec![];
        Solution::helper(n, &mut result, &mut HashMap::new(), 0, 0, 0, 0);

        return result;
    }
}
