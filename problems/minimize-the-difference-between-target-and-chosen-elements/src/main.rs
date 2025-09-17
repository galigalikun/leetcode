fn main() {
    assert_eq!(
        Solution::minimize_the_difference(vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]], 13),
        0
    );
    assert_eq!(
        Solution::minimize_the_difference(vec![vec![1], vec![2], vec![3]], 100),
        94
    );
    assert_eq!(
        Solution::minimize_the_difference(vec![vec![1, 2, 9, 8, 7]], 6),
        1
    );
}

struct Solution;
impl Solution {
    pub fn minimize_the_difference(mat: Vec<Vec<i32>>, target: i32) -> i32 {
        let m = mat.len();
        let mut dp = vec![false; 5001];
        for &v in &mat[0] {
            dp[v as usize] = true;
        }
        for i in 1..m {
            let mut ndp = vec![false; 5001];
            for j in 0..=5000 {
                if dp[j] {
                    for &v in &mat[i] {
                        let nj = j + v as usize;
                        if nj <= 5000 {
                            ndp[nj] = true;
                        }
                    }
                }
            }
            dp = ndp;
        }
        for i in 0..=5000 {
            if target as usize >= i && dp[target as usize - i]
                || target as usize + i <= 5000 && dp[target as usize + i]
            {
                return i as i32;
            }
        }
        return 0;
    }
}
