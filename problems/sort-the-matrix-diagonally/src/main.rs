fn main() {
    assert_eq!(
        Solution::diagonal_sort(vec![vec![3, 3, 1, 1], vec![2, 2, 1, 2], vec![1, 1, 1, 2]]),
        vec![[1, 1, 1, 1], [1, 2, 2, 2], [1, 2, 3, 3]]
    );
    assert_eq!(
        Solution::diagonal_sort(vec![
            vec![11, 25, 66, 1, 69, 7],
            vec![23, 55, 17, 45, 15, 52],
            vec![75, 31, 36, 44, 58, 8],
            vec![22, 27, 33, 25, 68, 4],
            vec![84, 28, 14, 11, 5, 50]
        ]),
        vec![
            [5, 17, 4, 1, 52, 7],
            [11, 11, 25, 45, 8, 69],
            [14, 23, 25, 44, 58, 15],
            [22, 27, 31, 36, 50, 66],
            [84, 28, 75, 33, 55, 68]
        ]
    );
}

struct Solution;
impl Solution {
    pub fn diagonal_sort(mat: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut mat = mat;
        let mut diagonal = Vec::new();
        let row = mat.len();
        let col = mat[0].len();
        let mut i = 0;
        let mut j = 0;
        while i < row && j < col {
            diagonal.push(mat[i][j]);
            i += 1;
            j += 1;
        }
        diagonal.sort();
        i = 0;
        j = 0;
        let mut k = 0;
        while i < row && j < col {
            mat[i][j] = diagonal[k];
            i += 1;
            j += 1;
            k += 1;
        }
        return mat;
    }
}
