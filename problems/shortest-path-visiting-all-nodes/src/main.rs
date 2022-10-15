fn main() {
    assert_eq!(Solution::shortest_path_length(vec![vec![1,2,3],vec![0],vec![0],vec![0]]), 4);
    assert_eq!(Solution::shortest_path_length(vec![vec![1],vec![0,2,4],vec![1,3,4],vec![2],vec![1,2]]), 4);
}

struct Solution;
impl Solution {
    pub fn shortest_path_length(graph: Vec<Vec<i32>>) -> i32 {
        return 0;
    }
}
