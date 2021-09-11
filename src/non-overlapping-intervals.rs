fn main() {
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 3]]),
        1
    );
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![1, 2], vec![1, 2]]),
        2
    );
    assert_eq!(
        Solution::erase_overlap_intervals(vec![vec![1, 2], vec![2, 3]]),
        0
    );
    assert_eq!(
        Solution::erase_overlap_intervals(vec![
            vec![1, 100],
            vec![11, 22],
            vec![1, 11],
            vec![2, 12]
        ]),
        2
    );
}

pub struct Solution {}
// http://web.stanford.edu/class/archive/cs/cs161/cs161.1176/Sections/final_review-1.pdf
impl Solution {
    pub fn erase_overlap_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut result = intervals.len() as i32 - 1;
        let mut interval = intervals;
        interval.sort_by(|a, b| a[1].cmp(&b[1]));
        let mut prev = &interval[0];
        for i in 1..interval.len() {
            if interval[i][0] >= prev[1] {
                result -= 1;
                prev = &interval[i];
            }
        }
        return result;
    }
}
