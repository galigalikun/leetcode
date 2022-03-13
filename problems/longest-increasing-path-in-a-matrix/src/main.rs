fn main() {
    assert_eq!(
        Solution::longest_increasing_path(vec![vec![9, 9, 4], vec![6, 6, 8], vec![2, 1, 1]]),
        4
    );

    assert_eq!(
        Solution::longest_increasing_path(vec![vec![3, 4, 5], vec![3, 2, 6], vec![2, 2, 1]]),
        4
    );

    assert_eq!(Solution::longest_increasing_path(vec![vec![1]]), 1);
}

struct Solution {}
// https://dev.to/seanpgallivan/solution-longest-increasing-path-in-a-matrix-4o5f
impl Solution {
    fn helper(
        dp: &mut Vec<Vec<i32>>,
        matrix: Vec<Vec<i32>>,
        n: i32,
        m: i32,
        x: i32,
        y: i32,
    ) -> i32 {
        if dp[x as usize][y as usize] > 0 {
            return dp[x as usize][y as usize];
        }
        dp[x as usize][y as usize] = 1 + std::cmp::max(
            std::cmp::max(
                std::cmp::max(
                    if n - 1 > x
                        && matrix[x as usize][y as usize] > matrix[x as usize + 1][y as usize]
                    {
                        Solution::helper(dp, matrix.clone(), n, m, x + 1, y)
                    } else {
                        0
                    },
                    if x > 0 && matrix[x as usize][y as usize] > matrix[x as usize - 1][y as usize]
                    {
                        Solution::helper(dp, matrix.clone(), n, m, x - 1, y)
                    } else {
                        0
                    },
                ),
                if m - 1 > y && matrix[x as usize][y as usize] > matrix[x as usize][y as usize + 1]
                {
                    Solution::helper(dp, matrix.clone(), n, m, x, y + 1)
                } else {
                    0
                },
            ),
            if y > 0 && matrix[x as usize][y as usize] > matrix[x as usize][y as usize - 1] {
                Solution::helper(dp, matrix.clone(), n, m, x, y - 1)
            } else {
                0
            },
        );

        return dp[x as usize][y as usize];
    }
    pub fn longest_increasing_path(matrix: Vec<Vec<i32>>) -> i32 {
        let mut result = 0;
        let mut dp = vec![vec![0; matrix[0].len()]; matrix.len()];
        for i in 0..matrix.len() {
            for j in 0..matrix[i].len() {
                result = std::cmp::max(
                    result,
                    Solution::helper(
                        &mut dp,
                        matrix.clone(),
                        matrix.len() as i32,
                        matrix[0].len() as i32,
                        i as i32,
                        j as i32,
                    ),
                );
            }
        }

        return result;
    }
}
