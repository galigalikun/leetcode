fn main() {
    assert_eq!(
        Solution::largest_number(vec![4, 3, 2, 5, 6, 7, 2, 5, 5], 9),
        "7772"
    );
    assert_eq!(
        Solution::largest_number(vec![7, 6, 5, 5, 5, 6, 8, 7, 8], 12),
        "85"
    );
    assert_eq!(
        Solution::largest_number(vec![2, 4, 6, 2, 4, 6, 4, 4, 4], 5),
        "0"
    );
}

struct Solution;
impl Solution {
    pub fn largest_number(cost: Vec<i32>, target: i32) -> String {
        let mut dp = vec![0; target as usize + 1];
        for c in cost.iter() {
            for i in *c..=target {
                dp[i as usize] = dp[i as usize].max(dp[(i - c) as usize] + 1);
            }
        }
        if dp[target as usize] == 0 {
            return "0".to_string();
        }
        let mut ans = "".to_string();
        let mut i = target;
        for j in (0..9).rev() {
            while i >= cost[j as usize] && dp[i as usize] == dp[(i - cost[j as usize]) as usize] + 1
            {
                ans.push((j as u8 + 1 + b'0') as char);
                i -= cost[j as usize];
            }
        }
        return ans;
    }
}
