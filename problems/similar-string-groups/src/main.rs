fn main() {
    assert_eq!(
        Solution::num_similar_groups(vec![
            "tars".to_string(),
            "rats".to_string(),
            "arts".to_string(),
            "star".to_string()
        ]),
        2
    );
    assert_eq!(
        Solution::num_similar_groups(vec!["omv".to_string(), "ovm".to_string()]),
        1
    );
}

struct Solution {}
impl Solution {
    pub fn num_similar_groups(strs: Vec<String>) -> i32 {
        let n = strs.len();
        if n == 0 {
            return 0;
        }

        let mut uf = UnionFind::new(n);
        for i in 0..n {
            for j in (i + 1)..n {
                if is_similar(&strs[i], &strs[j]) {
                    uf.union(i, j);
                }
            }
        }

        uf.group_count() as i32
    }
}

struct UnionFind {
    parent: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
        }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] == x {
            return x;
        }

        let root = self.find(self.parent[x]);
        self.parent[x] = root;
        root
    }

    fn union(&mut self, a: usize, b: usize) {
        let root_a = self.find(a);
        let root_b = self.find(b);
        if root_a != root_b {
            self.parent[root_a] = root_b;
        }
    }

    fn group_count(&mut self) -> usize {
        let n = self.parent.len();
        (0..n)
            .filter(|&i| self.find(i) == i)
            .count()
    }
}

fn is_similar(a: &str, b: &str) -> bool {
    let mut diff_count = 0;
    let mut first = None;
    let mut second = None;

    for (ca, cb) in a.as_bytes().iter().zip(b.as_bytes().iter()) {
        if ca != cb {
            diff_count += 1;
            if diff_count == 1 {
                first = Some((*ca, *cb));
            } else if diff_count == 2 {
                second = Some((*ca, *cb));
            } else {
                return false;
            }
        }
    }

    if diff_count == 0 {
        return true;
    }

    if diff_count != 2 {
        return false;
    }

    let (a1, b1) = first.expect("first mismatch must exist when diff_count is 2");
    let (a2, b2) = second.expect("second mismatch must exist when diff_count is 2");
    a1 == b2 && b1 == a2
}
