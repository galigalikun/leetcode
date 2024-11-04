fn main() {
    assert_eq!(
        Solution::max_width_of_vertical_area(vec![vec![8, 7], vec![9, 9], vec![7, 4], vec![9, 7]]),
        1
    );
    assert_eq!(
        Solution::max_width_of_vertical_area(vec![
            vec![3, 1],
            vec![9, 0],
            vec![1, 0],
            vec![1, 4],
            vec![5, 3],
            vec![8, 8]
        ]),
        3
    );
}

struct Solution;
impl Solution {
    pub fn max_width_of_vertical_area(points: Vec<Vec<i32>>) -> i32 {
        let mut x = points.iter().map(|p| p[0]).collect::<Vec<i32>>();
        x.sort();
        let mut max = 0;
        for i in 1..x.len() {
            max = max.max(x[i] - x[i - 1]);
        }
        max
    }
}
