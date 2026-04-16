fn main() {
    assert_eq!(
        Solution::minimum_total_distance(vec![0, 4, 6], vec![vec![2, 2], vec![6, 2]]),
        4
    );
    assert_eq!(
        Solution::minimum_total_distance(vec![1, -1], vec![vec![-2, 1], vec![2, 1]]),
        2
    );
}

struct Solution;
impl Solution {
    pub fn minimum_total_distance(robot: Vec<i32>, factory: Vec<Vec<i32>>) -> i64 {
        let mut robot = robot;
        robot.sort_unstable();
        let mut factory = factory;
        factory.sort_unstable_by_key(|f| f[0]);
        let mut ans = 0;
        let mut j = 0;
        for i in 0..robot.len() {
            while j < factory.len() && factory[j][1] == 0 {
                j += 1;
            }
            if j == factory.len() {
                break;
            }
            ans += (robot[i] - factory[j][0]).abs() as i64;
            factory[j][1] -= 1;
        }
        ans
    }
}
