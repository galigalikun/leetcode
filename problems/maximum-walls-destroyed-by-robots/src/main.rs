fn main() {
    assert_eq!(Solution::max_walls(vec![4], vec![3], vec![1, 10]), 1);
    assert_eq!(
        Solution::max_walls(vec![10, 2], vec![5, 1], vec![5, 2, 7]),
        3
    );
    assert_eq!(Solution::max_walls(vec![1, 2], vec![100, 1], vec![10]), 0);
}

struct Solution;
impl Solution {
    pub fn max_walls(robots: Vec<i32>, distance: Vec<i32>, walls: Vec<i32>) -> i32 {
        let mut ans = 0;
        for i in 0..robots.len() {
            if robots[i] + distance[i] >= walls[i] {
                ans += 1;
            }
        }
        ans
    }
}
