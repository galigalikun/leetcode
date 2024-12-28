fn main() {
    assert_eq!(
        Solution::max_height(vec![vec![50, 45, 20], vec![95, 37, 53], vec![45, 23, 12]]),
        190
    );
    assert_eq!(
        Solution::max_height(vec![vec![38, 25, 45], vec![76, 35, 3]]),
        76
    );
    assert_eq!(
        Solution::max_height(vec![
            vec![7, 11, 17],
            vec![7, 17, 11],
            vec![11, 7, 17],
            vec![11, 17, 7],
            vec![17, 7, 11],
            vec![17, 11, 7]
        ]),
        102
    );
}

struct Solution;
impl Solution {
    pub fn max_height(cuboids: Vec<Vec<i32>>) -> i32 {
        let mut cuboids = cuboids;
        cuboids.iter_mut().for_each(|cuboid| cuboid.sort_unstable());
        cuboids.sort_unstable();
        let n = cuboids.len();
        let mut dp = vec![0; n];
        dp[0] = cuboids[0][2];
        for i in cuboids.iter().skip(1).enumerate() {
            let (i, cuboid) = i;
            dp[i] = cuboid[2];
            for j in 0..i {
                if cuboid[0] >= cuboids[j][0]
                    && cuboid[1] >= cuboids[j][1]
                    && cuboid[2] >= cuboids[j][2]
                {
                    dp[i] = dp[i].max(dp[j] + cuboid[2]);
                }
            }
        }
        dp.into_iter().max().unwrap_or(0)
    }
}
