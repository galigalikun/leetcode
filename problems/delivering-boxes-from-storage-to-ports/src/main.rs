fn main() {
    assert_eq!(Solution::box_delivering(vec![vec![1, 1], vec![2, 1], vec![1, 1]], 2, 3, 3), 4);
    assert_eq!(Solution::box_delivering(vec![vec![1,2],vec![3,3],vec![3,1],vec![3,1],vec![2,4]], 3, 3, 6), 6);
    assert_eq!(Solution::box_delivering(vec![vec![1,4],vec![1,2],vec![2,1],vec![2,1],vec![3,2],vec![3,4]], 3, 6, 7), 6);
}

struct Solution;
impl Solution {
    pub fn box_delivering(boxes: Vec<Vec<i32>>, ports_count: i32, max_boxes: i32, max_weight: i32) -> i32 {
        let n = boxes.len();
        let mut dp = vec![0; n+1];
        let mut weight = 0;
        let mut j = 0;
        let mut ans = 0;
        for i in 0..n {
            while j < n && j - i < max_boxes as usize && weight + boxes[j][1] <= max_weight {
                weight += boxes[j][1];
                if j == 0 || boxes[j][0] != boxes[j-1][0] {
                    dp[j+1] = dp[j] + 1;
                } else {
                    dp[j+1] = dp[j];
                }
                j += 1;
            }
            ans = std::cmp::max(ans, dp[j] + dp[i] + 1);
            weight -= boxes[i][1];
            if i == n-1 || boxes[i][0] != boxes[i+1][0] {
                dp[i+1] = dp[i] - 1;
            } else {
                dp[i+1] = dp[i];
            }
        }
        return ans;
    }
}
