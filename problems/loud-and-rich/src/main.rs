fn main() {
    assert_eq!(
        Solution::loud_and_rich(
            vec![
                vec![1, 0],
                vec![2, 1],
                vec![3, 1],
                vec![3, 7],
                vec![4, 3],
                vec![5, 3],
                vec![6, 3]
            ],
            vec![3, 2, 5, 4, 6, 1, 7, 0]
        ),
        vec![5, 5, 2, 5, 4, 5, 6, 7]
    );
    assert_eq!(Solution::loud_and_rich(vec![], vec![0]), vec![0]);
}

struct Solution;
impl Solution {
    pub fn loud_and_rich(richer: Vec<Vec<i32>>, quiet: Vec<i32>) -> Vec<i32> {
        return vec![];
    }
}
