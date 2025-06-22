fn main() {
    assert_eq!(
        Solution::find_rotation(vec![vec![0, 1], vec![1, 0]], vec![vec![1, 0], vec![0, 1]]),
        true
    );
    assert_eq!(
        Solution::find_rotation(vec![vec![0, 1], vec![1, 1]], vec![vec![1, 0], vec![0, 1]]),
        false
    );
    assert_eq!(
        Solution::find_rotation(
            vec![vec![0, 0, 0], vec![0, 1, 0], vec![1, 1, 1]],
            vec![vec![1, 1, 1], vec![0, 1, 0], vec![0, 0, 0]]
        ),
        true
    );
}

struct Solution;
impl Solution {
    pub fn find_rotation(mat: Vec<Vec<i32>>, target: Vec<Vec<i32>>) -> bool {
        let mut mat = mat;
        return (0..4).any(|_i| {
            let rotated = (0..mat.len())
                .map(|j| {
                    (0..mat[0].len())
                        .map(|k| mat[mat.len() - 1 - k][j])
                        .collect()
                })
                .collect::<Vec<Vec<i32>>>();
            if rotated == target {
                return true;
            }
            mat = rotated;
            false
        });
    }
}
