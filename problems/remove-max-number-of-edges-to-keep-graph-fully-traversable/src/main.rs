fn main() {
    assert_eq!(
        Solution::max_num_edges_to_remove(
            4,
            vec![
                vec![3, 1, 2],
                vec![3, 2, 3],
                vec![1, 1, 3],
                vec![1, 2, 4],
                vec![1, 1, 2],
                vec![2, 3, 4]
            ]
        ),
        2
    );
    assert_eq!(
        Solution::max_num_edges_to_remove(
            4,
            vec![vec![3, 1, 2], vec![3, 2, 3], vec![1, 1, 4], vec![2, 1, 4]]
        ),
        0
    );
    assert_eq!(
        Solution::max_num_edges_to_remove(4, vec![vec![3, 2, 3], vec![1, 1, 2], vec![2, 3, 4]]),
        -1
    );
}

struct Solution;
struct UnionFind {
    parent: Vec<usize>,
    set_count: usize,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        UnionFind {
            parent,
            set_count: n,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }
    fn union(&mut self, u: usize, v: usize) -> bool {
        let u = self.find(u);
        let v = self.find(v);
        if u == v {
            return false;
        }
        self.parent[u] = v;
        self.set_count -= 1;
        true
    }
}
impl Solution {
    pub fn max_num_edges_to_remove(n: i32, edges: Vec<Vec<i32>>) -> i32 {
        let n = n as usize;
        let mut alice = UnionFind::new(n);
        let mut bob = UnionFind::new(n);
        let mut ans = 0;
        for edge in edges.iter() {
            let t = edge[0];
            let u = (edge[1] - 1) as usize;
            let v = (edge[2] - 1) as usize;
            if t == 3 {
                if !alice.union(u, v) {
                    ans += 1;
                } else {
                    bob.union(u, v);
                }
            }
        }
        for edge in edges.iter() {
            let t = edge[0];
            let u = (edge[1] - 1) as usize;
            let v = (edge[2] - 1) as usize;
            if t == 1 {
                if !alice.union(u, v) {
                    ans += 1;
                }
            } else if t == 2 {
                if !bob.union(u, v) {
                    ans += 1;
                }
            }
        }
        if alice.set_count == 1 && bob.set_count == 1 {
            ans
        } else {
            -1
        }
    }
}
