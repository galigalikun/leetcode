fn main() {
    assert_eq!(
        Solution::find_smallest_set_of_vertices(
            6,
            vec![vec![0, 1], vec![0, 2], vec![2, 5], vec![3, 4], vec![4, 2]]
        ),
        vec![0, 3]
    );
    assert_eq!(
        Solution::find_smallest_set_of_vertices(
            5,
            vec![vec![0, 1], vec![2, 1], vec![3, 1], vec![1, 4], vec![2, 4]]
        ),
        vec![0, 2, 3]
    );
}

struct Solution;
impl Solution {
    pub fn find_smallest_set_of_vertices(n: i32, edges: Vec<Vec<i32>>) -> Vec<i32> {
        return edges
            .iter()
            .fold((0..n).collect::<Vec<i32>>(), |mut acc, edge| {
                acc.retain(|&x| x != edge[1]);
                acc
            });
    }
}
