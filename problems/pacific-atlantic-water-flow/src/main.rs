fn main() {
    assert_eq!(
        Solution::pacific_atlantic(vec![
            vec![1, 2, 2, 3, 5],
            vec![3, 2, 3, 4, 4],
            vec![2, 4, 5, 3, 1],
            vec![6, 7, 1, 4, 5],
            vec![5, 1, 1, 2, 4]
        ]),
        vec![[0, 4], [1, 4], [1, 3], [2, 2], [4, 0], [3, 0], [3, 1]]
    );
    assert_eq!(
        Solution::pacific_atlantic(vec![vec![2, 1], vec![1, 2]]),
        vec![[0, 0], [1, 1], [1, 0], [0, 1]]
    );
}

struct Solution {}
impl Solution {
    fn helper(
        result: &mut Vec<Vec<i32>>,
        dp: &mut Vec<i32>,
        heights: Vec<Vec<i32>>,
        i: usize,
        j: usize,
        w: i32,
        h: i32,
    ) {
        let y = heights.len();
        let x = heights[0].len();
        let ij = i * x + j;
        if (dp[ij] & w) > 0 || heights[i][j] < h {
            return;
        }
        dp[ij] += w;
        if dp[ij] == 3 {
            result.push(vec![i as i32, j as i32]);
        }
        if i + 1 < y {
            Solution::helper(result, dp, heights.clone(), i + 1, j, w, heights[i][j]);
        }
        if i > 0 {
            Solution::helper(result, dp, heights.clone(), i - 1, j, w, heights[i][j]);
        }
        if j + 1 < x {
            Solution::helper(result, dp, heights.clone(), i, j + 1, w, heights[i][j]);
        }
        if j > 0 {
            Solution::helper(result, dp, heights.clone(), i, j - 1, w, heights[i][j]);
        }
    }
    pub fn pacific_atlantic(heights: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let y = heights.len();
        let x = heights[0].len();
        let mut result = vec![];
        let mut dp = vec![0; x * y];
        for i in 0..y {
            Solution::helper(
                &mut result,
                &mut dp,
                heights.clone(),
                i,
                0,
                1,
                heights[i][0],
            );
            Solution::helper(
                &mut result,
                &mut dp,
                heights.clone(),
                i,
                x - 1,
                2,
                heights[i][x - 1],
            );
        }
        for j in 0..x {
            Solution::helper(
                &mut result,
                &mut dp,
                heights.clone(),
                0,
                j,
                1,
                heights[0][j],
            );
            Solution::helper(
                &mut result,
                &mut dp,
                heights.clone(),
                y - 1,
                j,
                2,
                heights[y - 1][j],
            );
        }
        return result;
    }
}
