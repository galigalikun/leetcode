fn main() {
    assert_eq!(Solution::num_moves_stones_ii(vec![7,4,9]), vec![1,2]);
    assert_eq!(Solution::num_moves_stones_ii(vec![6,5,4,3,10]), vec![2,3]);
    assert_eq!(Solution::num_moves_stones_ii(vec![100,101,104,102,103]), vec![0,0]);
}

struct Solution;
impl Solution {
    pub fn num_moves_stones_ii(mut stones: Vec<i32>) -> Vec<i32> {
        stones.sort_unstable();
        let n = stones.len();

        // Maximum moves:
        // Keep one endpoint fixed each time, and move the other endpoint inward.
        let max_left_fixed = stones[n - 1] - stones[1] - n as i32 + 2;
        let max_right_fixed = stones[n - 2] - stones[0] - n as i32 + 2;
        let max_moves = max_left_fixed.max(max_right_fixed);

        // Minimum moves with sliding window of width n.
        let mut min_moves = n as i32;
        let mut left = 0usize;
        for right in 0..n {
            while stones[right] - stones[left] + 1 > n as i32 {
                left += 1;
            }

            let already_in_window = right - left + 1;
            if already_in_window == n - 1 && stones[right] - stones[left] + 1 == n as i32 - 1 {
                // Special case: one stone is isolated by exactly one gap on an edge.
                min_moves = min_moves.min(2);
            } else {
                min_moves = min_moves.min((n - already_in_window) as i32);
            }
        }

        vec![min_moves, max_moves]
    }
}
