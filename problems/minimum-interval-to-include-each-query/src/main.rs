fn main() {
    assert_eq!(
        Solution::min_interval(
            vec![vec![1, 4], vec![2, 4], vec![3, 6], vec![4, 4]],
            vec![2, 3, 4, 5]
        ),
        vec![3, 3, 1, 4]
    );
    assert_eq!(
        Solution::min_interval(
            vec![vec![2, 3], vec![2, 5], vec![1, 8], vec![20, 25]],
            vec![2, 19, 5, 22]
        ),
        vec![2, -1, 4, 6]
    );
}

struct Solution;
impl Solution {
    pub fn min_interval(intervals: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        let mut intervals = intervals;
        for interval in &mut intervals {
            interval[0] -= 1;
            interval[1] -= 1;
        }
        let mut queries = queries;
        for query in &mut queries {
            *query -= 1;
        }
        let mut queries_with_index: Vec<(i32, usize)> =
            queries.iter().enumerate().map(|(i, &q)| (q, i)).collect();
        queries_with_index.sort_by(|a, b| a.0.cmp(&b.0));
        intervals.sort_by(|a, b| a[0].cmp(&b[0]));
        let mut min_heap = std::collections::BinaryHeap::new();
        let mut result = vec![-1; queries.len()];
        let mut i = 0;
        for (query, index) in queries_with_index {
            while i < intervals.len() && intervals[i][0] <= query {
                let interval = &intervals[i];
                let size = interval[1] - interval[0] + 1;
                min_heap.push((size, interval[1]));
                i += 1;
            }
            while !min_heap.is_empty() && min_heap.peek().unwrap().1 < query {
                min_heap.pop();
            }
            if !min_heap.is_empty() {
                result[index] = min_heap.peek().unwrap().0;
            }
        }
        for i in 0..result.len() {
            if result[i] != -1 {
                result[i] += 1;
            }
        }
        return result
            .into_iter()
            .map(|x| if x == -1 { -1 } else { x })
            .collect();
    }
}
