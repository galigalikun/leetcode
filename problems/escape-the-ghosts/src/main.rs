fn main() {
    assert_eq!(
        Solution::escape_ghosts(vec![vec![1, 0], vec![0, 3]], vec![0, 1]),
        true
    );
    assert_eq!(Solution::escape_ghosts(vec![vec![1, 0]], vec![2, 0]), false);
    assert_eq!(Solution::escape_ghosts(vec![vec![2, 0]], vec![1, 0]), false);
}

struct Solution {}

impl Solution {
    pub fn escape_ghosts(ghosts: Vec<Vec<i32>>, target: Vec<i32>) -> bool {
        let my_distance = target[0].abs() + target[1].abs();

        for ghost in ghosts {
            let ghost_distance = (ghost[0] - target[0]).abs() + (ghost[1] - target[1]).abs();
            if ghost_distance <= my_distance {
                return false;
            }
        }

        true
    }
}
