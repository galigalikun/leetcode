fn main() {
    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0']
        ]),
        1
    );

    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1']
        ]),
        3
    );

    assert_eq!(
        Solution::num_islands(vec![
            vec!['0', '1', '0'],
            vec!['1', '0', '1'],
            vec!['0', '1', '0']
        ]),
        4
    );

    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '1'],
            vec!['0', '1', '0'],
            vec!['1', '1', '1']
        ]),
        1
    );

    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1'],
            vec!['1', '1', '1', '0', '1']
        ]),
        1
    );

    assert_eq!(
        Solution::num_islands(vec![
            vec!['1', '1', '1', '1', '1', '0', '1', '1', '1', '1'],
            vec!['1', '0', '1', '0', '1', '1', '1', '1', '1', '1'],
            vec!['0', '1', '1', '1', '0', '1', '1', '1', '1', '1'],
            vec!['1', '1', '0', '1', '1', '0', '0', '0', '0', '1'],
            vec!['1', '0', '1', '0', '1', '0', '0', '1', '0', '1'],
            vec!['1', '0', '0', '1', '1', '1', '0', '1', '0', '0'],
            vec!['0', '0', '1', '0', '0', '1', '1', '1', '1', '0'],
            vec!['1', '0', '1', '1', '1', '0', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1', '1', '1', '1', '0', '1'],
            vec!['1', '0', '1', '1', '1', '1', '1', '1', '1', '0']
        ]),
        2
    );
}

pub struct Solution {}
// https://ts0818.hatenablog.com/entry/2020/10/21/230443
impl Solution {
    fn helper(target: &mut Vec<Vec<char>>, y: usize, x: usize) {
        let row = target.len();
        let col = target[0].len();

        if y >= row || x >= col || target[y][x] != '1' {
            return;
        }

        target[y][x] = '0';

        Solution::helper(target, y + 1, x);
        if y > 0 {
            Solution::helper(target, y - 1, x);
        }

        Solution::helper(target, y, x + 1);
        if x > 0 {
            Solution::helper(target, y, x - 1);
        }
    }
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        let mut c = 0;
        let row = grid.len();
        let col = grid[0].len();
        let mut target = grid.clone();
        for y in 0..row {
            for x in 0..col {
                if target[y][x] == '1' {
                    Solution::helper(&mut target, y, x);
                    c += 1;
                }
            }
        }

        return c;
    }
}
