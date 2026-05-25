fn main() {
    let mut got1 = Solution::outer_trees(vec![
        vec![1, 1],
        vec![2, 2],
        vec![2, 0],
        vec![2, 4],
        vec![3, 3],
        vec![4, 2],
    ]);
    let mut want1 = vec![vec![1, 1], vec![2, 0], vec![3, 3], vec![2, 4], vec![4, 2]];
    got1.sort();
    want1.sort();
    assert_eq!(got1, want1);

    let mut got2 = Solution::outer_trees(vec![vec![1, 2], vec![2, 2], vec![4, 2]]);
    let mut want2 = vec![vec![4, 2], vec![2, 2], vec![1, 2]];
    got2.sort();
    want2.sort();
    assert_eq!(got2, want2);
}

use std::collections::BTreeSet;

struct Solution {}

impl Solution {
    fn cross(p: &[i32], q: &[i32], r: &[i32]) -> i32 {
        (q[0] - p[0]) * (r[1] - p[1]) - (q[1] - p[1]) * (r[0] - p[0])
    }

    pub fn outer_trees(trees: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        if trees.len() <= 3 {
            return trees;
        }

        let mut points = trees;
        points.sort();

        let mut lower: Vec<Vec<i32>> = Vec::new();
        for p in &points {
            while lower.len() >= 2
                && Solution::cross(&lower[lower.len() - 2], &lower[lower.len() - 1], p) < 0
            {
                lower.pop();
            }
            lower.push(p.clone());
        }

        let mut upper: Vec<Vec<i32>> = Vec::new();
        for p in points.iter().rev() {
            while upper.len() >= 2
                && Solution::cross(&upper[upper.len() - 2], &upper[upper.len() - 1], p) < 0
            {
                upper.pop();
            }
            upper.push(p.clone());
        }

        let mut uniq: BTreeSet<(i32, i32)> = BTreeSet::new();
        for p in lower.into_iter().chain(upper.into_iter()) {
            uniq.insert((p[0], p[1]));
        }

        uniq.into_iter().map(|(x, y)| vec![x, y]).collect()
    }
}
