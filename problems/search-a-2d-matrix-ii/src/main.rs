fn main() {
    assert_eq!(
        Solution::search_matrix(
            vec![
                [1, 4, 7, 11, 15].to_vec(),
                [2, 5, 8, 12, 19].to_vec(),
                [3, 6, 9, 16, 22].to_vec(),
                [10, 13, 14, 17, 24].to_vec(),
                [18, 21, 23, 26, 30].to_vec()
            ],
            5
        ),
        true
    );

    assert_eq!(
        Solution::search_matrix(
            vec![
                [1, 4, 7, 11, 15].to_vec(),
                [2, 5, 8, 12, 19].to_vec(),
                [3, 6, 9, 16, 22].to_vec(),
                [10, 13, 14, 17, 24].to_vec(),
                [18, 21, 23, 26, 30].to_vec()
            ],
            20
        ),
        false
    );

    assert_eq!(
        Solution::search_matrix(vec![[-1].to_vec(), [-1].to_vec()], 0),
        false
    );

    assert_eq!(
        Solution::search_matrix(
            vec![
                [3, 6, 9, 12, 17, 22].to_vec(),
                [5, 11, 11, 16, 22, 26].to_vec(),
                [10, 11, 14, 16, 24, 31].to_vec(),
                [10, 15, 17, 17, 29, 31].to_vec(),
                [14, 17, 20, 23, 34, 37].to_vec(),
                [19, 21, 22, 28, 37, 40].to_vec(),
                [22, 22, 24, 32, 37, 43].to_vec()
            ],
            20
        ),
        true
    );

    assert_eq!(Solution::search_matrix(vec![[-5].to_vec()], -5), true);

    assert_eq!(Solution::search_matrix(vec![[-5].to_vec()], -10), false);
}

struct Solution {}
// https://dev.to/seanpgallivan/solution-search-a-2d-matrix-ii-4lc
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        let y = matrix.len();
        let mut i = 0;
        let mut j = matrix[0].len() - 1;
        while i < y && j >= 0 {
            if matrix[i][j] == target {
                return true;
            } else if matrix[i][j] > target {
                if j == 0 {
                    break;
                }
                j -= 1;
            } else {
                i += 1;
            }
        }
        return false;
    }
}
