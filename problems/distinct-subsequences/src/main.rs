fn main() {
    assert_eq!(
        Solution::num_distinct("rabbbit".to_string(), "rabbit".to_string()),
        3
    );

    assert_eq!(
        Solution::num_distinct("babgbag".to_string(), "bag".to_string()),
        5
    );
}

struct Solution {}
impl Solution {
    pub fn num_distinct(s: String, t: String) -> i32 {
        let m = t.len();
        let n = s.len();
        if m > n {
            return 0;
        }
        let mut mat = vec![vec![0; n + 1]; m + 1];

        for i in 0..=n {
            mat[0][i] = 1;
        }

        for i in 1..=m {
            for j in 1..=n {
                if s.chars().nth(j - 1) != t.chars().nth(i - 1) {
                    mat[i][j] = mat[i][j - 1];
                } else {
                    mat[i][j] = mat[i][j - 1] + mat[i - 1][j - 1];
                }
            }
        }
        return mat[m][n];
    }
}
