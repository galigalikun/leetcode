fn main() {
    assert_eq!(
        Solution::insert(vec![vec![1, 3], vec![6, 9]], vec![2, 5]),
        vec![[1, 5], [6, 9]]
    );
    assert_eq!(
        Solution::insert(
            vec![
                vec![1, 2],
                vec![3, 5],
                vec![6, 7],
                vec![8, 10],
                vec![12, 16]
            ],
            vec![4, 8]
        ),
        vec![[1, 2], [3, 10], [12, 16]]
    );
    assert_eq!(Solution::insert(vec![], vec![5, 7]), vec![[5, 7]]);
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![2, 3]), vec![[1, 5]]);
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![2, 7]), vec![[1, 7]]);
    assert_eq!(Solution::insert(vec![vec![1, 5]], vec![5, 7]), vec![[1, 7]]);
    assert_eq!(
        Solution::insert(vec![vec![1, 5]], vec![6, 8]),
        vec![[1, 5], [6, 8]]
    );
}

pub struct Solution {}
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        if intervals.len() == 0 {
            return vec![new_interval];
        }
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..intervals.len() {
            if i > 0 {
                let idx = result.len() - 1;
                if result[idx][1] < intervals[i][0] {
                    if new_interval[0] <= intervals[i][1] && new_interval[1] > intervals[i][1] {
                        result.push(vec![intervals[i][0], new_interval[1]]);
                    } else {
                        result.push(intervals[i].clone());
                    }
                } else if intervals[i][1] < result[idx][0] {
                    if let Some(p) = result.pop() {
                        if new_interval[0] <= intervals[i][1] && new_interval[1] > intervals[i][1] {
                            result.push(vec![intervals[i][0], new_interval[1]]);
                        } else {
                            result.push(intervals[i].clone());
                        }
                        result.push(p);
                    }
                } else {
                    let start = if intervals[i].first() < result[idx].first() {
                        intervals[i][0]
                    } else {
                        result[idx][0]
                    };
                    let end = if intervals[i].last() > result[idx].last() {
                        intervals[i][1]
                    } else {
                        result[idx][1]
                    };
                    result.pop();
                    result.push(vec![start, end]);
                }
            } else {
                if new_interval[0] <= intervals[i][1] && new_interval[1] > intervals[i][1] {
                    result.push(vec![intervals[i][0], new_interval[1]]);
                } else {
                    result.push(intervals[i].clone());
                }
            }
        }
        return result;
    }
}
