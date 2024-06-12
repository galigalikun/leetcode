fn main() {
    assert_eq!(Solution::min_distance(vec![1, 4, 8, 10, 20], 3), 5);
    assert_eq!(Solution::min_distance(vec![2, 3, 5, 12, 18], 2), 9);
}

struct Solution;
impl Solution {
    fn cost(houses: &Vec<i32>, i: usize, j: usize) -> i32 {
        let mut res = 0;
        let mut i = i;
        while i < j {
            res += (houses[j] - houses[i]) / 2;
            i += 1;
        }
        return res;
    }
    fn helper(houses: &Vec<i32>, i: usize, k: usize, dp: &mut Vec<Vec<i32>>) -> i32 {
        if k == 0 {
            return if i == 0 { 0 } else { i32::MAX };
        }
        if i == 0 {
            return 0;
        }
        let mut res = i32::MAX;
        for j in (0..i).rev() {
            let cost = Self::cost(&houses, j, i);
            if cost == i32::MAX {
                break;
            }
            res = res.min(cost + dp[k - 1][j]);
        }
        return res;
    }
    pub fn min_distance(houses: Vec<i32>, k: i32) -> i32 {
        let mut houses = houses;
        houses.sort();
        let n = houses.len();
        let mut dp = vec![vec![0; n]; k as usize];
        for i in 0..n {
            for j in 0..k as usize {
                dp[j][i] = Self::helper(&houses, i, j, &mut dp);
            }
        }
        return dp[k as usize - 1][n - 1];
    }
}
