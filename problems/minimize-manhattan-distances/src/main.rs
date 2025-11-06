fn main() {
    assert_eq!(
        Solution::minimum_distance(vec![vec![3, 10], vec![5, 15], vec![10, 2], vec![4, 4]]),
        12
    );
    assert_eq!(
        Solution::minimum_distance(vec![vec![1, 1], vec![1, 1], vec![1, 1]]),
        0
    );
}

struct Solution;
impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        return points
            .windows(2)
            .map(|w| {
                let dx = w[0][0] - w[1][0];
                let dy = w[0][1] - w[1][1];
                dx.abs() + dy.abs()
            })
            .sum();
    }
}
