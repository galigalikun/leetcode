fn main() {
    assert_eq!(
        Solution::is_rectangle_overlap(vec![0, 0, 2, 2], vec![1, 1, 3, 3]),
        true
    );
    assert_eq!(
        Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![1, 0, 2, 1]),
        false
    );
    assert_eq!(
        Solution::is_rectangle_overlap(vec![0, 0, 1, 1], vec![2, 2, 3, 3]),
        false
    );
    assert_eq!(
        Solution::is_rectangle_overlap(vec![7, 8, 13, 15], vec![10, 8, 12, 20]),
        true
    );
    assert_eq!(
        Solution::is_rectangle_overlap(vec![2, 17, 6, 20], vec![2, 17, 6, 20]),
        true
    );
    assert_eq!(
        Solution::is_rectangle_overlap(vec![2, 17, 6, 20], vec![3, 8, 6, 20]),
        true
    );
}

struct Solution {}
impl Solution {
    pub fn is_rectangle_overlap(rec1: Vec<i32>, rec2: Vec<i32>) -> bool {
        let overlap_x = rec1[0].max(rec2[0]) < rec1[2].min(rec2[2]);
        let overlap_y = rec1[1].max(rec2[1]) < rec1[3].min(rec2[3]);

        overlap_x && overlap_y
    }
}
