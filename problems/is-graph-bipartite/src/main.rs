fn main() {
    assert_eq!(Solution::is_bipartite(vec![vec![1,2,3],vec![0,2],vec![0,1,3],vec![0,2]]), false);
    assert_eq!(Solution::is_bipartite(vec![vec![1,3],vec![0,2],vec![1,3],vec![0,2]]), true);
}

struct Solution{}
impl Solution {
    pub fn is_bipartite(graph: Vec<Vec<i32>>) -> bool {
        return false;
    }
}
