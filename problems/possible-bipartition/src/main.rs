fn main() {
    assert_eq!(
        Solution::possible_bipartition(4, vec![vec![1, 2], vec![1, 3], vec![2, 4]]),
        true
    );
    assert_eq!(
        Solution::possible_bipartition(3, vec![vec![1, 2], vec![1, 3], vec![2, 3]]),
        false
    );
    assert_eq!(
        Solution::possible_bipartition(
            5,
            vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![1, 5]]
        ),
        false
    );
}

struct Solution;
impl Solution {
    pub fn possible_bipartition(n: i32, dislikes: Vec<Vec<i32>>) -> bool {
        return false;
    }
}
