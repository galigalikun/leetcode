fn main() {
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(vec![1, 2, 3, 2]),
        vec![1, 2, 3, 3]
    );
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(vec![2, 2, 1]),
        vec![1, 2, 1]
    );
    assert_eq!(
        Solution::longest_obstacle_course_at_each_position(vec![3, 1, 5, 6, 4, 2]),
        vec![1, 1, 2, 3, 2, 2]
    );
}

struct Solution;
impl Solution {
    pub fn longest_obstacle_course_at_each_position(obstacles: Vec<i32>) -> Vec<i32> {
        let mut dp = vec![0; obstacles.len()];
        let mut lis = vec![];

        for (i, &obstacle) in obstacles.iter().enumerate() {
            if lis.is_empty() || obstacle >= *lis.last().unwrap() {
                lis.push(obstacle);
                dp[i] = lis.len() as i32;
            } else {
                let pos = lis.binary_search(&obstacle).unwrap_or_default();
                lis[pos] = obstacle;
                dp[i] = pos as i32 + 1;
            }
        }

        dp
    }
}
