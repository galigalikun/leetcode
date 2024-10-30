use std::vec;

fn main() {
    assert_eq!(
        Solution::matrix_rank_transform(vec![vec![1, 2], vec![3, 4]]),
        vec![[1, 2], [2, 3]]
    );
    assert_eq!(
        Solution::matrix_rank_transform(vec![vec![7, 7], vec![7, 7]]),
        vec![[1, 1], [1, 1]]
    );
    assert_eq!(
        Solution::matrix_rank_transform(vec![
            vec![20, -21, 14],
            vec![-19, 4, 19],
            vec![22, -47, 24],
            vec![-19, 4, 19]
        ]),
        vec![[4, 2, 3], [1, 3, 4], [5, 1, 6], [1, 3, 4]]
    );
}

struct Solution;
impl Solution {
    fn find(map: &mut std::collections::HashMap<i32, i32>, x: &i32) -> i32 {
        if let Some(&y) = map.get(x) {
            if y != *x {
                map.insert(*x, Solution::find(map, &y));
            }
            map[x]
        } else {
            map.insert(*x, *x);
            *x
        }
    }
    pub fn matrix_rank_transform(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut res = vec![vec![0; matrix[0].len()]; matrix.len()];
        let mut rank = vec![0; matrix.len() + matrix[0].len()];
        let mut map = std::collections::HashMap::new();
        let mut matrix = matrix
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .map(|(j, &v)| (v, i, j))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<Vec<_>>();
        matrix.sort_unstable();
        for (v, i, j) in &matrix {
            let mut rank = rank.clone();
            let mut map = map.clone();
            let (i, j) = (*i as i32, *j as i32);
            let (mut x, mut y) = (
                Solution::find(&mut map, &i),
                Solution::find(&mut map, &(j + matrix.len() as i32)),
            );
            if x != y {
                map.insert(x, y);
            }
            rank[x as usize] = rank[x as usize].max(rank[y as usize]);
            for (v, i, j) in matrix.iter().filter(|(v, _, _)| *v == *v) {
                let (i, j) = (*i as i32, *j as i32);
                let (x, y) = (
                    Solution::find(&mut map, &i),
                    Solution::find(&mut map, &(j + matrix.len() as i32)),
                );
                rank[x as usize] = rank[x as usize].max(rank[y as usize]);
            }
            rank[x as usize] += 1;
            res[i as usize][j as usize] = rank[x as usize];
        }
        res
    }
}
