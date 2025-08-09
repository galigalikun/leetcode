fn main() {
    assert_eq!(
        Solution::max_genetic_difference(
            vec![-1, 0, 1, 1],
            vec![vec![0, 2], vec![3, 2], vec![2, 5]]
        ),
        vec![2, 3, 7]
    );
    assert_eq!(
        Solution::max_genetic_difference(
            vec![3, 7, -1, 2, 0, 7, 0, 2],
            vec![vec![4, 6], vec![1, 15], vec![0, 5]]
        ),
        vec![6, 14, 7]
    );
}

struct Solution;
impl Solution {
    pub fn max_genetic_difference(parents: Vec<i32>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let mut result = Vec::new();
        for query in queries {
            let (u, v) = (query[0], query[1]);
            let mut max_diff = 0;
            for i in 0..parents.len() {
                if parents[i] == u || parents[i] == v {
                    max_diff = max_diff.max((u ^ v).count_ones());
                }
            }
            result.push(max_diff as i32);
        }
        result
    }
}
