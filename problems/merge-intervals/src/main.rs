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
    assert_eq!(
        Solution::merge(vec![vec![1, 4], vec![0, 0]]),
        vec![[0, 0], [1, 4]]
    );
    assert_eq!(
        Solution::merge(vec![vec![1, 4], vec![5, 6]]),
        vec![[1, 4], [5, 6]]
    );
    assert_eq!(
        Solution::merge(vec![vec![1, 4], vec![0, 2], vec![3, 5]]),
        vec![[0, 5]]
    );
    assert_eq!(
        Solution::merge(vec![
            vec![2, 3],
            vec![4, 5],
            vec![6, 7],
            vec![8, 9],
            vec![1, 10]
        ]),
        vec![[1, 10]]
    );
}

struct Solution {}
impl Solution {
    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut data = intervals;
        data.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut result: Vec<Vec<i32>> = Vec::new();
        for i in 0..data.len() {
            if i > 0 {
                let idx = result.len() - 1;
                if result[idx][1] < data[i][0] {
                    result.push(data[i].clone());
                } else if data[i][1] < result[idx][0] {
                    if let Some(p) = result.pop() {
                        result.push(data[i].clone());
                        result.push(p);
                    }
                } else {
                    let start = if data[i].first() < result[idx].first() {
                        data[i][0]
                    } else {
                        result[idx][0]
                    };
                    let end = if data[i].last() > result[idx].last() {
                        data[i][1]
                    } else {
                        result[idx][1]
                    };
                    result.pop();
                    result.push(vec![start, end]);
                }
            } else {
                result.push(data[i].clone());
            }
        }
        return result;
    }
}
