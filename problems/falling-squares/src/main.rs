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
        let mut max_height = 0;
        let mut max_area = 0;
        let mut heights = vec![0; positions.len()];
        let mut areas = vec![0; positions.len()];
        let mut result = vec![0; positions.len()];
        for (i, position) in positions.iter().enumerate() {
            let x = position[0];
            let y = position[1];
            let mut height = heights[i];
            let mut area = areas[i];
            for j in 0..i {
                let other_x = positions[j][0];
                let other_y = positions[j][1];
                if x >= other_x && x <= other_x + other_y {
                    height = std::cmp::max(height, heights[j]);
                    area = std::cmp::max(area, areas[j]);
                }
            }
            heights[i] = height + y;
            areas[i] = std::cmp::max(area, height + y);
            max_height = std::cmp::max(max_height, heights[i]);
            max_area = std::cmp::max(max_area, areas[i]);
            result[i] = max_area;
        }
        println!("debug {:?}", result);
        return result;
    }
}
