fn main() {
    assert_eq!(Solution::sum_of_distances_in_tree(6, vec![vec![0,1],vec![0,2],vec![2,3],vec![2,4],vec![2,5]]), vec![8,12,6,10,10,10]);
    assert_eq!(Solution::sum_of_distances_in_tree(1, vec![]), vec![0]);
    assert_eq!(Solution::sum_of_distances_in_tree(2, vec![vec![1,0]]), vec![1,1]);
}

struct Solution{}
impl Solution {
    pub fn sum_of_distances_in_tree(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        return vec![];
    }
}
