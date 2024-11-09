fn main() {
    assert_eq!(
        Solution::furthest_building(vec![4, 2, 7, 6, 9, 14, 12], 5, 1),
        4
    );
    assert_eq!(
        Solution::furthest_building(vec![4, 12, 2, 7, 3, 18, 20, 3, 19], 10, 2),
        7
    );
    assert_eq!(Solution::furthest_building(vec![14, 3, 19, 3], 17, 0), 3);
}

struct Solution;
impl Solution {
    pub fn furthest_building(heights: Vec<i32>, bricks: i32, ladders: i32) -> i32 {
        let mut heap = std::collections::BinaryHeap::new();
        let mut bricks = bricks;
        for i in 0..heights.len() - 1 {
            let diff = heights[i + 1] - heights[i];
            if diff > 0 {
                heap.push(diff);
                if heap.len() > ladders as usize {
                    bricks -= heap.pop().unwrap();
                }
                if bricks < 0 {
                    return i as i32;
                }
            }
        }
        heights.len() as i32 - 1
    }
}
