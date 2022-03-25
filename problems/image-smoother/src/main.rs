fn main() {
    assert_eq!(
        Solution::image_smoother(vec![vec![1, 1, 1], vec![1, 0, 1], vec![1, 1, 1]]),
        vec![vec![0, 0, 0], vec![0, 0, 0], vec![0, 0, 0]]
    );
    assert_eq!(
        Solution::image_smoother(vec![
            vec![100, 200, 100],
            vec![200, 50, 200],
            vec![100, 200, 100]
        ]),
        vec![[137, 141, 137], [141, 138, 141], [137, 141, 137]]
    );
}

struct Solution {}
impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        return vec![];
    }
}
