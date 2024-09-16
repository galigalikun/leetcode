fn main() {
    assert_eq!(Solution::min_cost_connect_points(vec![vec![0,0],vec![2,2],vec![3,10],vec![5,2],vec![7,0]]), 20);
    assert_eq!(Solution::min_cost_connect_points(vec![vec![3,12],vec![-2,5],vec![-4,1]]), 18);
}

struct Solution;
struct UnionFind {
    parent: Vec<usize>,
    rank: Vec<usize>,
}
impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            rank: vec![0; n],
        }
    }
    fn find(&mut self, x: usize) -> usize {
        return 0;
    }
    fn union(&mut self, x: usize, y: usize) -> bool {
        let x = self.find(x);
        let y = self.find(y);
        if x == y {
            return false;
        }

        return true;
    }
}
impl Solution {
    pub fn min_cost_connect_points(points: Vec<Vec<i32>>) -> i32 {
        let n = points.len();
        let mut edges = vec![];
        for i in 0..n {
            points.iter().enumerate().for_each(|(j, p)| {
                edges.push((i, j, (p[0] - points[i][0]).abs() + (p[1] - points[i][1]).abs()));
            });
        }
        edges.sort_by_key(|x| x.2);
        let mut uf = UnionFind::new(n);
        let mut ans = 0;
        for (x, y, d) in edges {
            if uf.union(x, y) {
                ans += d;
            }
        }
        ans
    }
}
