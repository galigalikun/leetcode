fn main() {
    assert_eq!(
        Solution::smallest_string_with_swaps("dcab".to_string(), vec![vec![0, 3], vec![1, 2]]),
        "bacd"
    );
    assert_eq!(
        Solution::smallest_string_with_swaps(
            "dcab".to_string(),
            vec![vec![0, 3], vec![1, 2], vec![0, 2]]
        ),
        "abcd"
    );
    assert_eq!(
        Solution::smallest_string_with_swaps("cba".to_string(), vec![vec![0, 1], vec![1, 2]]),
        "abc"
    );
}

struct Solution;
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = vec![0; n];
        for i in 0..n {
            parent[i] = i;
        }
        Self {
            parent,
            rank: vec![0; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        return self.parent[x];
    }
    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x == root_y {
            return;
        }
        if self.rank[root_x] > self.rank[root_y] {
            self.parent[root_y] = root_x;
        } else if self.rank[root_x] < self.rank[root_y] {
            self.parent[root_x] = root_y;
        } else {
            self.parent[root_y] = root_x;
            self.rank[root_x] += 1;
        }
    }
}
impl Solution {
    pub fn smallest_string_with_swaps(s: String, pairs: Vec<Vec<i32>>) -> String {
        let mut s = s.into_bytes();
        let mut uf = UnionFind::new(s.len());
        for pair in pairs {
            uf.union(pair[0] as usize, pair[1] as usize);
        }
        let mut map = std::collections::HashMap::new();
        for i in 0..s.len() {
            let root = uf.find(i);
            map.entry(root).or_insert(Vec::new()).push(i);
        }
        for (_, v) in map {
            let mut chars = v.iter().map(|&i| s[i]).collect::<Vec<u8>>();
            chars.sort();
            for i in 0..v.len() {
                s[v[i]] = chars[i];
            }
        }
        return String::from_utf8(s).unwrap();
    }
}
