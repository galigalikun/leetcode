fn main() {
    assert_eq!(Solution::find_right_interval(vec![vec![1, 2]]), vec![-1]);
    assert_eq!(
        Solution::find_right_interval(vec![vec![3, 4], vec![2, 3], vec![1, 2]]),
        vec![-1, 0, 1]
    );
    assert_eq!(
        Solution::find_right_interval(vec![vec![1, 4], vec![2, 3], vec![3, 4]]),
        vec![-1, 2, -1]
    );
}

struct Solution {}
// https://just4once.gitbooks.io/leetcode-notes/content/leetcode/binary-search/436-find-right-interval.html
impl Solution {
    pub fn find_right_interval(intervals: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = vec![];
        for i in 0..intervals.len() {
            let mut min = std::i32::MAX;
            let mut min_idx = -1;
            for j in 0..intervals.len() {
                if intervals[j][0] >= intervals[i][1] && intervals[j][0] < min {
                    min = intervals[j][0];
                    min_idx = j as i32;
                }
            }
            result.push(min_idx);
        }
        return result;
    }
}
