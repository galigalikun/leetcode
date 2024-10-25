use std::vec;

fn main() {
    assert_eq!(
        Solution::are_connected(6, 2, vec![vec![1, 4], vec![2, 5], vec![3, 6]]),
        vec![true, true, true]
    );
    assert_eq!(
        Solution::are_connected(6, 0, vec![vec![4, 5], vec![3, 4], vec![3, 2]]),
        vec![true, true, true]
    );
    assert_eq!(
        Solution::are_connected(5, 1, vec![vec![4, 5], vec![4, 5], vec![3, 2]]),
        vec![false, false, false]
    );
}

struct Solution;
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        UnionFind {
            parent,
            size: vec![0; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    fn union(&mut self, i: usize, j: usize) {
        let root_i = self.find(i);
        let root_j = self.find(j);
        if root_i == root_j {
            return;
        }
        if self.size[root_i] < self.size[root_j] {
            self.parent[root_i] = root_j;
            self.size[root_j] += self.size[root_i];
        } else {
            self.parent[root_j] = root_i;
            self.size[root_i] += self.size[root_j];
        }
    }
    fn connected(&mut self, i: usize, j: usize) -> bool {
        self.find(i) == self.find(j)
    }
}
impl Solution {
    pub fn are_connected(n: i32, threshold: i32, queries: Vec<Vec<i32>>) -> Vec<bool> {
        let mut uf = UnionFind::new(n as usize);
        for i in (threshold + 1)..=n {
            let mut j = 2;
            while i * j <= n {
                uf.union((i * j - 1) as usize, (i - 1) as usize);
                j += 1;
            }
        }
        queries
            .iter()
            .map(|q| uf.connected((q[0] - 1) as usize, (q[1] - 1) as usize))
            .collect()
    }
}
