fn main() {
    assert_eq!(
        Solution::process_queries(
            5,
            vec![[1, 2], [2, 3], [3, 4], [4, 5]]
                .iter()
                .map(|&x| x.to_vec())
                .collect(),
            vec![[1, 3], [2, 1], [1, 1], [2, 2], [1, 2]]
                .iter()
                .map(|&x| x.to_vec())
                .collect()
        ),
        vec![3, 2, 3]
    );
    assert_eq!(
        Solution::process_queries(
            3,
            vec![],
            vec![[1, 1], [2, 1], [1, 1]]
                .iter()
                .map(|&x| x.to_vec())
                .collect()
        ),
        vec![1, -1]
    );
}

struct Solution;
use std::collections::BTreeSet;
impl Solution {
    pub fn process_queries(c: i32, connections: Vec<Vec<i32>>, queries: Vec<Vec<i32>>) -> Vec<i32> {
        let n = c as usize;
        let mut dsu = DisjointSetUnion::new(n);
        for edge in connections {
            let u = (edge[0] - 1) as usize;
            let v = (edge[1] - 1) as usize;
            dsu.union(u, v);
        }

        let mut online_sets = vec![BTreeSet::<i32>::new(); n];
        for node in 0..n {
            let root = dsu.find(node);
            online_sets[root].insert((node + 1) as i32);
        }

        let mut online = vec![true; n];
        let mut answers = Vec::new();

        for query in queries {
            if query.len() != 2 {
                continue;
            }
            let q_type = query[0];
            let x = (query[1] - 1) as usize;
            match q_type {
                1 => {
                    if online[x] {
                        answers.push((x + 1) as i32);
                        continue;
                    }
                    let root = dsu.find(x);
                    if let Some(&station) = online_sets[root].iter().next() {
                        answers.push(station);
                    } else {
                        answers.push(-1);
                    }
                }
                2 => {
                    if online[x] {
                        online[x] = false;
                        let root = dsu.find(x);
                        online_sets[root].remove(&((x + 1) as i32));
                    }
                }
                _ => {}
            }
        }

        answers
    }
}

struct DisjointSetUnion {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DisjointSetUnion {
    fn new(n: usize) -> Self {
        DisjointSetUnion {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
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
}
