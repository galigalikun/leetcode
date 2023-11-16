fn main() {
    assert_eq!(Solution::min_flips(vec![vec![0, 0], vec![0, 1]]), 3);
    assert_eq!(Solution::min_flips(vec![vec![0]]), 0);
    assert_eq!(Solution::min_flips(vec![vec![1, 0, 0], vec![1, 0, 0]]), -1);
}

struct Solution;
impl Solution {
    pub fn min_flips(mat: Vec<Vec<i32>>) -> i32 {
        let mat = mat;
        let mut queue = std::collections::VecDeque::new();
        let mut visited = std::collections::HashSet::new();
        let mut step = 0;
        let m = mat.len();
        let n = mat[0].len();
        let mut target = vec![vec![0; n]; m];
        for i in 0..m {
            for j in 0..n {
                target[i][j] = 1;
            }
        }
        queue.push_back(mat.clone());
        visited.insert(mat.clone());
        while !queue.is_empty() {
            let size = queue.len();
            for _ in 0..size {
                let cur = queue.pop_front().unwrap();
                if cur == target {
                    return step;
                }
                for i in 0..m {
                    for j in 0..n {
                        let mut next = cur.clone();
                        next[i][j] = 1 - next[i][j];
                        if i > 0 {
                            next[i - 1][j] = 1 - next[i - 1][j];
                        }
                        if i < m - 1 {
                            next[i + 1][j] = 1 - next[i + 1][j];
                        }
                        if j > 0 {
                            next[i][j - 1] = 1 - next[i][j - 1];
                        }
                        if j < n - 1 {
                            next[i][j + 1] = 1 - next[i][j + 1];
                        }
                        if !visited.contains(&next) {
                            queue.push_back(next.clone());
                            visited.insert(next);
                        }
                    }
                }
            }
            step += 1;
        }

        return step;
    }
}
