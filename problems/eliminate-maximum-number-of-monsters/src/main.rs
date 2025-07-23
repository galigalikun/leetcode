fn main() {
    assert_eq!(Solution::eliminate_maximum(vec![1, 3, 4], vec![1, 1, 1]), 3);
    assert_eq!(
        Solution::eliminate_maximum(vec![1, 1, 2, 3], vec![1, 1, 1, 1]),
        4
    );
    assert_eq!(Solution::eliminate_maximum(vec![3, 2, 4], vec![5, 3, 2]), 1);
}

struct Solution;
impl Solution {
    pub fn eliminate_maximum(dist: Vec<i32>, speed: Vec<i32>) -> i32 {
        return dist
            .iter()
            .zip(speed.iter())
            .map(|(&d, &s)| (d as f64 / s as f64).ceil() as i32)
            .enumerate()
            .filter(|&(i, t)| i < t as usize)
            .count() as i32;
    }
}
