fn main() {
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![1, 3], vec![2, 3]]), vec![2, 3]);
    assert_eq!(Solution::find_redundant_connection(vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![1, 4], vec![1, 5]]), vec![1, 4]);
}

struct Solution{}
impl Solution {
    pub fn find_redundant_connection(edges: Vec<Vec<i32>>) -> Vec<i32> {
        return vec![];
    }
}
