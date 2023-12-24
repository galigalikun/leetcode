fn main() {
    assert_eq!(
        Solution::make_connected(4, vec![vec![0, 1], vec![0, 2], vec![1, 2]]),
        1
    );
    assert_eq!(
        Solution::make_connected(
            6,
            vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2], vec![1, 3]]
        ),
        2
    );
    assert_eq!(
        Solution::make_connected(6, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 2]]),
        -1
    );
}

struct Solution;
impl Solution {
    pub fn make_connected(n: i32, connections: Vec<Vec<i32>>) -> i32 {
        let mut uf = UnionFind::new(n as usize);
        let mut extra = 0;
        for c in connections {
            if !uf.union(c[0] as usize, c[1] as usize) {
                extra += 1;
            }
        }
        return if extra >= uf.count - 1 {
            uf.count - 1
        } else {
            -1
        };
    }
}
