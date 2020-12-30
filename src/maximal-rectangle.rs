fn main() {
    assert_eq!(
        Solution::maximal_rectangle(vec![
            vec!['1', '0', '1', '0', '0'],
            vec!['1', '0', '1', '1', '1'],
            vec!['1', '1', '1', '1', '1'],
            vec!['1', '0', '0', '1', '0']
        ]),
        6
    );
    assert_eq!(Solution::maximal_rectangle(vec![]), 0);
    assert_eq!(Solution::maximal_rectangle(vec![vec!['0']]), 0);
    assert_eq!(Solution::maximal_rectangle(vec![vec!['1']]), 1);
    assert_eq!(Solution::maximal_rectangle(vec![vec!['0', '0']]), 0);
}

pub struct Solution {}
// https://www.geeksforgeeks.org/maximum-size-rectangle-binary-sub-matrix-1s/
impl Solution {
    fn max_hit(r: usize, c: usize, row: &Vec<i32>) -> i32 {
        let mut result = Vec::new();
        let mut top_val = 0;
        let mut max_area = 0;
        let mut area = 0;
        let mut i = 0;
        while i < c {
            if result.is_empty()
                || if let Some(&top) = result.last() {
                    row[top] < row[i]
                } else {
                    false
                }
            {
                result.push(i);
                i += 1;
            } else if let Some(&top) = result.last() {
                top_val = row[top];
                result.pop();
                area = top_val * i as i32;
                if let Some(&top) = result.last() {
                    area = top_val * (i as i32 - top as i32 - 1);
                }

                max_area = std::cmp::max(area, max_area);
            }
        }

        while !result.is_empty() {
            if let Some(&top) = result.last() {
                top_val = row[top];
                result.pop();
                area = top_val * i as i32;
                if let Some(&top) = result.last() {
                    area = top_val * (i as i32 - top as i32 - 1);
                }
                max_area = std::cmp::max(area, max_area);
            }
        }

        return max_area;
    }
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let r = matrix.len();
        if r == 0 {
            return 0;
        }
        let c = matrix[0].len();

        let mut a = Vec::new();
        for i in 0..r {
            a.push(vec![]);
            for j in 0..c {
                a[i].push(matrix[i][j].to_digit(10).unwrap() as i32)
            }
        }

        let mut result = Solution::max_hit(r, c, &a[0]);
        for i in 1..r {
            for j in 0..c {
                if a[i][j] == 1 {
                    a[i][j] += a[i - 1][j];
                }
            }

            result = std::cmp::max(result, Solution::max_hit(r, c, &a[i]));
        }
        return result;
    }
}
