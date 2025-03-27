fn main() {
    assert_eq!(
        Solution::find_center(vec![vec![1, 2], vec![2, 3], vec![4, 2]]),
        2
    );
    assert_eq!(
        Solution::find_center(vec![vec![1, 2], vec![5, 1], vec![1, 3], vec![1, 4]]),
        1
    );
    assert_eq!(
        Solution::find_center(vec![
            vec![1, 18],
            vec![18, 2],
            vec![3, 18],
            vec![18, 4],
            vec![18, 5],
            vec![6, 18],
            vec![18, 7],
            vec![18, 8],
            vec![18, 9],
            vec![18, 10],
            vec![18, 11],
            vec![12, 18],
            vec![18, 13],
            vec![18, 14],
            vec![15, 18],
            vec![16, 18],
            vec![17, 18]
        ]),
        18
    );
}

struct Solution;
impl Solution {
    pub fn find_center(edges: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;
        let mut count = HashMap::new();
        for edge in edges.clone() {
            *count.entry(edge[0]).or_insert(0) += 1;
            *count.entry(edge[1]).or_insert(0) += 1;
        }
        let mut max = 0;
        let mut idx = 0;
        for (i, v) in count.iter() {
            if *v > max {
                max = *v;
                idx = *i;
            }
        }
        return idx as i32;
    }
}
