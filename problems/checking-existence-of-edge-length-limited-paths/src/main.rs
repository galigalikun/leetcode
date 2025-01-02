fn main() {
    assert_eq!(
        Solution::distance_limited_paths_exist(
            3,
            vec![vec![0, 1, 2], vec![1, 2, 4], vec![2, 0, 8], vec![1, 0, 16]],
            vec![vec![0, 1, 2], vec![0, 2, 5]]
        ),
        vec![false, true]
    );
    assert_eq!(
        Solution::distance_limited_paths_exist(
            5,
            vec![vec![0, 1, 10], vec![1, 2, 5], vec![2, 3, 9], vec![3, 4, 13]],
            vec![vec![0, 4, 14], vec![1, 4, 13]]
        ),
        vec![true, false]
    );
}

struct Solution;
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }
    fn union(&mut self, i: usize, j: usize) {
        let (mut i, mut j) = (self.find(i), self.find(j));
        if i == j {
            return;
        }
        if self.size[i] < self.size[j] {
            std::mem::swap(&mut i, &mut j);
        }
        self.parent[j] = i;
        self.size[i] += self.size[j];
    }
    fn find(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find(self.parent[i]);
        }
        self.parent[i]
    }
}
impl Solution {
    pub fn distance_limited_paths_exist(
        n: i32,
        edge_list: Vec<Vec<i32>>,
        queries: Vec<Vec<i32>>,
    ) -> Vec<bool> {
        let mut edge_list = edge_list.iter().enumerate().collect::<Vec<_>>();
        edge_list.sort_by_key(|x| x.1[2]);
        let mut queries = queries.iter().enumerate().collect::<Vec<_>>();
        queries.sort_by_key(|x| x.1[2]);
        let mut uf = UnionFind::new(n as usize);
        let mut res = vec![];
        let mut i = 0;
        for (_j, query) in queries {
            while i < edge_list.len() && edge_list[i].1[2] < query[2] {
                uf.union(edge_list[i].1[0] as usize, edge_list[i].1[1] as usize);
                i += 1;
            }
            res.push(uf.find(query[0] as usize) == uf.find(query[1] as usize));
        }
        res
    }
}
