fn main() {
    assert_eq!(
        Solution::rotate_the_box(vec![vec!['#', '.', '#'],]),
        vec![vec!['.', '#', '#'],]
    );
    assert_eq!(
        Solution::rotate_the_box(vec![vec!['#', '.', '*', '.'], vec!['#', '#', '#', '.'],]),
        vec![
            vec!['.', '#'],
            vec!['#', '#'],
            vec!['*', '*'],
            vec!['.', '.'],
        ]
    );
}

struct Solution;
impl Solution {
    pub fn rotate_the_box(box_grid: Vec<Vec<char>>) -> Vec<Vec<char>> {
        return box_grid
            .iter()
            .rev()
            .map(|row| {
                let mut new_row = vec!['.'; row.len()];
                let mut j = 0;
                for &c in row.iter() {
                    if c == '*' {
                        new_row[j] = '*';
                        j += 1;
                    } else if c == '#' {
                        new_row[j] = '#';
                        j += 1;
                    }
                }
                new_row
            })
            .collect();
    }
}
