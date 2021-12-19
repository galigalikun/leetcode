fn main() {
    assert_eq!(Solution::remove_boxes(vec![1, 3, 2, 2, 2, 3, 4, 3, 1]), 23);
    assert_eq!(Solution::remove_boxes(vec![1, 1, 1]), 9);
    assert_eq!(Solution::remove_boxes(vec![1]), 1);
}

// https://github.com/Seanforfun/Algorithm-and-Leetcode/blob/master/leetcode/546.%20Remove%20Boxes.md
struct Solution {}
impl Solution {
    fn helper(
        dp: &mut Vec<Vec<Vec<usize>>>,
        boxes: Vec<i32>,
        i: usize,
        j: usize,
        k: usize,
    ) -> usize {
        if i > j {
            return 0;
        }
        if dp[i][j][k] > 0 {
            return dp[i][j][k];
        }
        dp[i][j][k] = if i == j && j == 0 {
            (k + 1) * (k + 1)
        } else {
            Solution::helper(dp, boxes.clone(), i, j - 1, 0) + (k + 1) * (k + 1)
        };
        for p in i..j {
            if boxes[p] == boxes[j] {
                dp[i][j][k] = std::cmp::max(
                    dp[i][j][k],
                    Solution::helper(dp, boxes.clone(), i, p, k + 1)
                        + Solution::helper(dp, boxes.clone(), p + 1, j - 1, 0),
                );
            }
        }

        return dp[i][j][k];
    }
    pub fn remove_boxes(boxes: Vec<i32>) -> i32 {
        let mut dp = vec![vec![vec![0; boxes.len()]; boxes.len()]; boxes.len()];
        return Solution::helper(&mut dp, boxes.clone(), 0, boxes.len() - 1, 0) as i32;
    }
}
