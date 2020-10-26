fn main() {
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 3),
        "PAHNAPLSIIGYIR".to_string()
    );
    assert_eq!(
        Solution::convert("PAYPALISHIRING".to_string(), 4),
        "PINALSIGYAHRPI".to_string()
    );
    assert_eq!(Solution::convert("A".to_string(), 1), "A".to_string());
}

pub struct Solution {}

impl Solution {
    pub fn convert(s: String, num_rows: i32) -> String {
        if num_rows == 1 {
            return s;
        }
        let mut swap = Vec::new();
        for row in 0..num_rows {
            swap.push(Vec::new());
            for _ in 0..s.len() {
                swap[row as usize].push('\0');
            }
        }
        let mut col = 0;
        let mut row = 0;
        let mut dep = 0;
        for c in s.as_str().chars() {
            swap[row][col] = c;
            if col % (num_rows - 1) as usize == 0 {
                row += 1;
                if row % (num_rows) as usize == 0 {
                    col += 1;
                    row = (num_rows - 2) as usize;
                    dep += 1;
                }
            } else {
                col += 1;
                row = (num_rows - 2 - dep) as usize;
                dep += 1;
                if row == 0 {
                    dep = 0;
                }
            }
        }
        return swap
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .iter()
            .filter(|&x| x != &'\0')
            .collect::<String>();
    }
}
