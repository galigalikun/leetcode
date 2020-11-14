fn main() {
    assert_eq!(
        Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
            3
        ),
        true
    );
    assert_eq!(
        Solution::search_matrix(
            vec![vec![1, 3, 5, 7], vec![10, 11, 16, 20], vec![23, 30, 34, 50]],
            13
        ),
        false
    );
    assert_eq!(Solution::search_matrix(vec![], 0), false);
}

pub struct Solution {}
impl Solution {
    pub fn search_matrix(matrix: Vec<Vec<i32>>, target: i32) -> bool {
        for m in matrix {
            if let Some(&l) = m.last() {
                if let Some(&f) = m.first() {
                    if f == target || l == target {
                        return true;
                    } else if f < target && target < l {
                        if let Some(_) = m.into_iter().position(|n| n == target) {
                            return true;
                        }
                        return false;
                    } else if target < f {
                        return false;
                    }
                }
            }
        }
        return false;
    }
}
