fn main() {
    assert_eq!(
        Solution::ways_to_fill_array(vec![vec![2, 6], vec![5, 1], vec![73, 660]]),
        vec![4, 1, 50734910]
    );
    assert_eq!(
        Solution::ways_to_fill_array(vec![
            vec![1, 1],
            vec![2, 2],
            vec![3, 3],
            vec![4, 4],
            vec![5, 5]
        ]),
        vec![1, 2, 3, 10, 5]
    );
}

struct Solution;
impl Solution {
    pub fn ways_to_fill_array(queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut dp: Vec<Vec<i128>> = vec![vec![0; 1001]; 1001];
        for i in 1..1001 {
            dp[i][0] = 1;
            dp[i][i] = 1;
        }
        for i in 2..1001 {
            for j in 1..i {
                dp[i][j] = (dp[i - 1][j - 1] + dp[i - 1][j]) % 1_000_000_007;
            }
        }
        queries
            .iter()
            .map(|query| {
                let (n, k) = (query[0] as usize, query[1] as usize);
                (1..=k).fold(1, |acc, i| acc * dp[k + n - 1][i] % 1_000_000_007)
            })
            .map(|x| x as i32)
            .collect()
    }
}
