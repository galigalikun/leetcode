fn main() {
    assert_eq!(Solution::hit_bricks(vec![vec![1,0,0,0],vec![1,1,1,0]], vec![vec![1,0]]), vec![2]);
    assert_eq!(Solution::hit_bricks(vec![vec![1,0,0,0],vec![1,1,0,0]], vec![vec![1,1],vec![1,0]]), vec![0,0]);
}

struct Solution {}

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<i32>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect::<Vec<_>>();
        let size = vec![1; n];
        Self { parent, size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            let root = self.find(self.parent[x]);
            self.parent[x] = root;
        }
        self.parent[x]
    }

    fn union(&mut self, a: usize, b: usize) {
        let mut root_a = self.find(a);
        let mut root_b = self.find(b);
        if root_a == root_b {
            return;
        }
        if self.size[root_a] < self.size[root_b] {
            std::mem::swap(&mut root_a, &mut root_b);
        }
        self.parent[root_b] = root_a;
        self.size[root_a] += self.size[root_b];
    }

    fn component_size(&mut self, x: usize) -> i32 {
        let root = self.find(x);
        self.size[root]
    }
}

impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let rows = grid.len();
        let cols = grid[0].len();
        let roof = rows * cols;

        let mut state = grid.clone();
        for hit in &hits {
            let r = hit[0] as usize;
            let c = hit[1] as usize;
            if state[r][c] == 1 {
                state[r][c] = 0;
            } else {
                state[r][c] = -1;
            }
        }

        let mut uf = UnionFind::new(rows * cols + 1);
        for r in 0..rows {
            for c in 0..cols {
                if state[r][c] != 1 {
                    continue;
                }
                let id = r * cols + c;
                if r == 0 {
                    uf.union(id, roof);
                }
                if r > 0 && state[r - 1][c] == 1 {
                    uf.union(id, (r - 1) * cols + c);
                }
                if c > 0 && state[r][c - 1] == 1 {
                    uf.union(id, r * cols + (c - 1));
                }
            }
        }

        let mut answer = vec![0; hits.len()];
        let dirs = [(-1_i32, 0_i32), (1, 0), (0, -1), (0, 1)];

        for i in (0..hits.len()).rev() {
            let r = hits[i][0] as usize;
            let c = hits[i][1] as usize;

            if state[r][c] == -1 {
                state[r][c] = 0;
                answer[i] = 0;
                continue;
            }

            let before = uf.component_size(roof);
            state[r][c] = 1;
            let id = r * cols + c;

            if r == 0 {
                uf.union(id, roof);
            }

            for (dr, dc) in dirs {
                let nr = r as i32 + dr;
                let nc = c as i32 + dc;
                if nr < 0 || nr >= rows as i32 || nc < 0 || nc >= cols as i32 {
                    continue;
                }
                let nr_u = nr as usize;
                let nc_u = nc as usize;
                if state[nr_u][nc_u] == 1 {
                    uf.union(id, nr_u * cols + nc_u);
                }
            }

            let after = uf.component_size(roof);
            let fallen = (after - before - 1).max(0);
            answer[i] = fallen;
        }

        answer
    }
}

#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    fn sample_1() {
        let grid = vec![vec![1, 0, 0, 0], vec![1, 1, 1, 0]];
        let hits = vec![vec![1, 0]];
        assert_eq!(Solution::hit_bricks(grid, hits), vec![2]);
    }

    #[test]
    fn sample_2() {
        let grid = vec![vec![1, 0, 0, 0], vec![1, 1, 0, 0]];
        let hits = vec![vec![1, 1], vec![1, 0]];
        assert_eq!(Solution::hit_bricks(grid, hits), vec![0, 0]);
    }

    #[test]
    fn hit_empty_cell() {
        let grid = vec![vec![1, 0], vec![1, 1]];
        let hits = vec![vec![0, 1]];
        assert_eq!(Solution::hit_bricks(grid, hits), vec![0]);
    }

    #[test]
    fn duplicate_hit_same_cell() {
        let grid = vec![vec![1, 0], vec![1, 1]];
        let hits = vec![vec![1, 0], vec![1, 0]];
        assert_eq!(Solution::hit_bricks(grid, hits), vec![1, 0]);
    }
}
