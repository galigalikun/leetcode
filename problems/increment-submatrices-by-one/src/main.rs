fn main() {
    assert_eq!(
        Solution::range_add_queries(3, vec![vec![1, 1, 2, 2], vec![0, 0, 1, 1]]),
        vec![vec![1, 1, 0], vec![1, 2, 1], vec![0, 1, 1]]
    );
    assert_eq!(
        Solution::range_add_queries(2, vec![vec![0, 0, 1, 1]]),
        vec![vec![1, 1], vec![1, 1]]
    );
}

struct Solution;
impl Solution {
    pub fn range_add_queries(n: i32, queries: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let n = n as usize;
        let mut diff = vec![vec![0i32; n + 1]; n + 1];

        for query in queries {
            let r1 = query[0] as usize;
            let c1 = query[1] as usize;
            let r2 = query[2] as usize;
            let c2 = query[3] as usize;

            diff[r1][c1] += 1;
            if c2 + 1 <= n {
                diff[r1][c2 + 1] -= 1;
            }
            if r2 + 1 <= n {
                diff[r2 + 1][c1] -= 1;
            }
            if r2 + 1 <= n && c2 + 1 <= n {
                diff[r2 + 1][c2 + 1] += 1;
            }
        }

        let mut result = vec![vec![0i32; n]; n];
        for i in 0..n {
            for j in 0..n {
                let mut val = diff[i][j];
                if i > 0 {
                    val += result[i - 1][j];
                }
                if j > 0 {
                    val += result[i][j - 1];
                }
                if i > 0 && j > 0 {
                    val -= result[i - 1][j - 1];
                }
                result[i][j] = val;
            }
        }

        result
    }
}
