fn main() {
    assert_eq!(
        Solution::minimum_hamming_distance(
            vec![1, 2, 3, 4],
            vec![2, 1, 4, 5],
            vec![vec![0, 1], vec![2, 3]]
        ),
        1
    );
    assert_eq!(
        Solution::minimum_hamming_distance(vec![1, 2, 3, 4], vec![1, 3, 2, 4], vec![]),
        2
    );
    assert_eq!(
        Solution::minimum_hamming_distance(
            vec![5, 1, 2, 4, 3],
            vec![1, 5, 4, 2, 3],
            vec![vec![0, 4], vec![4, 2], vec![1, 3], vec![1, 4]]
        ),
        0
    );
}

struct Solution;
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let rank = vec![0; n];
        UnionFind { parent, rank }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    fn union(&mut self, mut x: usize, mut y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        if self.rank[root_x] < self.rank[root_y] {
            std::mem::swap(&mut x, &mut y);
        }
        self.parent[root_y] = root_x;
        if self.rank[root_x] == self.rank[root_y] {
            self.rank[root_x] += 1;
        }
    }
}
impl Solution {
    pub fn minimum_hamming_distance(
        source: Vec<i32>,
        target: Vec<i32>,
        allowed_swaps: Vec<Vec<i32>>,
    ) -> i32 {
        let n = source.len();
        let mut uf = UnionFind::new(n);
        for pair in allowed_swaps {
            uf.union(pair[0] as usize, pair[1] as usize);
        }
        let mut groups = vec![vec![]; n];
        for i in 0..n {
            let root = uf.find(i);
            groups[root].push(i);
        }
        let mut res = 0;
        for group in groups {
            let mut count = std::collections::HashMap::new();
            for &i in &group {
                *count.entry(source[i]).or_insert(0) += 1;
            }
            for &i in &group {
                *count.entry(target[i]).or_insert(0) -= 1;
            }
            res += count.values().map(|&x| x).sum::<i32>() / 2;
        }
        res
    }
}
