fn main() {
    assert_eq!(
        Solution::remove_covered_intervals(vec![vec![1, 4], vec![3, 6], vec![2, 8]]),
        2
    );
    assert_eq!(
        Solution::remove_covered_intervals(vec![vec![1, 4], vec![2, 3]]),
        1
    );
}

struct Solution;
impl Solution {
    pub fn remove_covered_intervals(intervals: Vec<Vec<i32>>) -> i32 {
        let mut intervals = intervals;
        for i in 0..intervals.len() {
            for j in 0..intervals.len() {
                if i == j {
                    continue;
                }
                if intervals[i][0] >= intervals[j][0] && intervals[i][1] <= intervals[j][1] {
                    intervals[i][0] = -1;
                    intervals[i][1] = -1;
                    break;
                }
            }
        }
        let mut count = 0;
        for i in 0..intervals.len() {
            if intervals[i][0] != -1 {
                count += 1;
            }
        }
        return count;
    }
}
