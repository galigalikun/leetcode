fn main() {
    assert_eq!(
        Solution::merge(vec![vec![1, 3], vec![2, 6], vec![8, 10], vec![15, 18]]),
        vec![[1, 6], [8, 10], [15, 18]]
    );
    assert_eq!(Solution::merge(vec![vec![1, 4], vec![4, 5]]), vec![[1, 5]]);
    assert_eq!(Solution::merge(vec![vec![1, 4], vec![1, 4]]), vec![[1, 4]]);
    assert_eq!(Solution::merge(vec![vec![1, 4], vec![1, 5]]), vec![[1, 5]]);
    assert_eq!(Solution::merge(vec![vec![1, 4], vec![0, 4]]), vec![[0, 4]]);
    assert_eq!(Solution::merge(vec![vec![1, 4], vec![0, 1]]), vec![[0, 4]]);
    assert_eq!(Solution::merge(vec![vec![1, 4], vec![2, 3]]), vec![[1, 4]]);
    assert_eq!(Solution::merge(vec![vec![1, 4], vec![0, 0]]), vec![[0, 4]]);
}

pub struct Solution {}
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for i in 0..intervals.len() {
            if i > 0 {
                if intervals[i - 1].last() < intervals[i].first() {
                    result.push(intervals[i].clone());
                } else if intervals[i].last() < intervals[i - 1].first() {
                    result.push(intervals[i].clone());
                } else {
                    let start = if intervals[i].first() < intervals[i - 1].first() {
                        intervals[i][0]
                    } else {
                        intervals[i - 1][0]
                    };
                    let end = if intervals[i].last() > intervals[i - 1].last() {
                        intervals[i][1]
                    } else {
                        intervals[i - 1][1]
                    };
                    result.pop();
                    result.push(vec![start, end]);
                }
            // if intervals[i - 1].first() <= intervals[i].first()
            //     && intervals[i].first() < intervals[i - 1].last()
            //     && intervals[i - 1].last() < intervals[i].last()
            // {
            //     result.pop();
            //     result.push(vec![intervals[i - 1][0], intervals[i][1]]);
            // } else if intervals[i - 1].first() <= intervals[i].first()
            //     && intervals[i].first() < intervals[i - 1].last()
            // {
            //     continue;
            // } else if intervals[i - 1].last() == intervals[i].first() {
            //     result.pop();
            //     result.push(vec![intervals[i - 1][0], intervals[i][1]]);
            // } else if intervals[i - 1].first() == intervals[i].first()
            //     && intervals[i - 1].last() == intervals[i].last()
            // {
            //     continue;
            // } else if intervals[i - 1].first() > intervals[i].first()
            //     && intervals[i - 1].last() > intervals[i].last()
            // {
            //     result.pop();
            //     result.push(vec![intervals[i][0], intervals[i - 1][1]]);
            // } else if intervals[i - 1].first() > intervals[i].first()
            //     && intervals[i - 1].last() <= intervals[i].last()
            // {
            //     result.pop();
            //     result.push(vec![intervals[i][0], intervals[i][1]]);
            // } else {
            //     result.push(intervals[i].clone());
            // }
            } else {
                result.push(intervals[i].clone());
            }
        }
        return result;
    }
}
