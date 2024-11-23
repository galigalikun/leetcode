fn main() {
    assert_eq!(Solution::can_distribute(vec![1, 2, 3, 4], vec![2]), false);
    assert_eq!(Solution::can_distribute(vec![1, 2, 3, 3], vec![2]), true);
    assert_eq!(Solution::can_distribute(vec![1, 1, 2, 2], vec![2, 2]), true);
}

struct Solution;
impl Solution {
    fn dfs(
        dp: &mut Vec<Vec<i32>>,
        freq: &Vec<i32>,
        quantity: &Vec<i32>,
        mask: usize,
        qmask: usize,
    ) -> bool {
        if mask == freq.len() {
            return qmask == (1 << quantity.len()) - 1;
        }
        if dp[mask][qmask] != -1 {
            return dp[mask][qmask] == 1;
        }
        let mut res = Self::dfs(dp, freq, quantity, mask + 1, qmask);
        for i in 0..quantity.len() {
            if (qmask >> i) & 1 == 0 && freq[mask] >= quantity[i] {
                res |= Self::dfs(dp, freq, quantity, mask + 1, qmask | (1 << i));
            }
        }
        dp[mask][qmask] = res as i32;
        res
    }
    pub fn can_distribute(nums: Vec<i32>, quantity: Vec<i32>) -> bool {
        let mut freq = vec![0; 1 << nums.len()];
        for &num in &nums {
            freq.iter_mut().for_each(|f| *f += num.count_ones() as i32);
        }
        let mut dp = vec![vec![-1; 1 << quantity.len()]; 1 << nums.len()];
        Self::dfs(&mut dp, &freq, &quantity, 0, 0)
    }
}
