fn main() {
    assert_eq!(
        Solution::falling_squares(vec![vec![1, 2], vec![2, 3], vec![6, 1]]),
        vec![2, 5, 5]
    );
    assert_eq!(
        Solution::falling_squares(vec![vec![100, 100], vec![200, 100]]),
        vec![100, 100]
    );
}

struct Solution {}
impl Solution {
    pub fn falling_squares(positions: Vec<Vec<i32>>) -> Vec<i32> {
        let mut heights = vec![0i32; positions.len()];
        let mut result = vec![0i32; positions.len()];
        let mut global_max = 0;

        for (i, position) in positions.iter().enumerate() {
            let left = position[0];
            let size = position[1];
            let right = left + size;
            let mut base = 0;

            for j in 0..i {
                let other_left = positions[j][0];
                let other_right = other_left + positions[j][1];
                // Overlap requires strictly intersecting intervals (touching edges don't count)
                if left < other_right && right > other_left {
                    base = base.max(heights[j]);
                }
            }

            heights[i] = base + size;
            global_max = global_max.max(heights[i]);
            result[i] = global_max;
        }

        result
    }
}
