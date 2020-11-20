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
}

pub struct Solution {}
impl Solution {
    pub fn insert(intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        let mut result = Vec::new();
        for interval in intervals {
            if new_interval[0] < interval[1] && new_interval[1] > interval[1] {
                result.push(vec![interval[0], new_interval[1]]);
            } else {
                result.push(interval);
            }
        }
        return result;
    }
}
