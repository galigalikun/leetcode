fn main() {
    assert_eq!(
        Solution::find_critical_and_pseudo_critical_edges(
            5,
            vec![
                vec![0, 1, 1],
                vec![1, 2, 1],
                vec![2, 3, 2],
                vec![0, 3, 2],
                vec![0, 4, 3],
                vec![3, 4, 3],
                vec![1, 4, 6]
            ]
        ),
        vec![vec![0, 1], vec![2, 3, 4, 5]]
    );
    assert_eq!(
        Solution::find_critical_and_pseudo_critical_edges(
            4,
            vec![vec![0, 1, 1], vec![1, 2, 1], vec![2, 3, 1], vec![0, 3, 1]]
        ),
        vec![vec![], vec![0, 1, 2, 3]]
    );
}

struct Solution;
struct UnionFind {
    parent: Vec<usize>,
    set_count: usize,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect::<Vec<usize>>(),
            set_count: n,
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        return self.parent[x];
    }
    fn union(&mut self, edge1: usize, edge2: usize) -> bool {
        let root1 = self.find(edge1);
        let root2 = self.find(edge2);
        if root1 == root2 {
            return false;
        }
        self.parent[root1] = root2;
        self.set_count -= 1;
        return true;
    }
}
impl Solution {
    pub fn find_critical_and_pseudo_critical_edges(n: i32, edges: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut edges = edges
            .iter()
            .enumerate()
            .map(|(i, v)| vec![i as i32, v[0], v[1], v[2]])
            .collect::<Vec<Vec<i32>>>();
        edges.sort_by(|a, b| a[3].cmp(&b[3]));
        let mut uf = UnionFind::new(n as usize);
        let mut min_cost = 0;
        for i in 0..edges.len() {
            if uf.union(edges[i][1] as usize, edges[i][2] as usize) {
                min_cost += edges[i][3];
            }
        }
        let mut res = vec![vec![], vec![]];
        for i in 0..edges.len() {
            let mut uf = UnionFind::new(n as usize);
            let mut cost: i32 = 0;
            for j in 0..edges.len() {
                if i != j && uf.union(edges[j][1] as usize, edges[j][2] as usize) {
                    cost += edges[j][3];
                }
            }
            if uf.set_count != 1 || (uf.set_count == 1 && cost > min_cost) {
                res[0].push(edges[i][0]);
            } else {
                let mut uf = UnionFind::new(n as usize);
                uf.union(edges[i][1] as usize, edges[i][2] as usize);
                cost = edges[i][3];
                for j in 0..edges.len() {
                    if i != j && uf.union(edges[j][1] as usize, edges[j][2] as usize) {
                        cost += edges[j][3];
                    }
                }
                if cost == min_cost {
                    res[1].push(edges[i][0]);
                }
            }
        }
        return res;
    }
}
