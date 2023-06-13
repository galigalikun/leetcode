fn main() {
    assert_eq!(Solution::last_stone_weight_ii(vec![2, 7, 4, 1, 8, 1]), 1);
    assert_eq!(Solution::last_stone_weight_ii(vec![31, 26, 33, 21, 40]), 5);
}

struct Solution;
impl Solution {
    pub fn last_stone_weight_ii(stones: Vec<i32>) -> i32 {
        let sum = stones.iter().sum::<i32>() as usize;
        let mut dp = vec![false; sum / 2 + 1];
        dp[0] = true;
        for stone in stones {
            for j in (stone as usize..=sum / 2).rev() {
                dp[j] |= dp[j - stone as usize];
            }
        }
        for j in (0..=sum / 2).rev() {
            if dp[j] {
                return (sum - j * 2) as i32;
            }
        }
        return 0;
    }
}
