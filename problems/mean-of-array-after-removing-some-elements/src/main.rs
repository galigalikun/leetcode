fn main() {
    assert_eq!(
        Solution::trim_mean(vec![
            1, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 3
        ]),
        2.0
    );
    assert_eq!(
        Solution::trim_mean(vec![
            6, 2, 7, 5, 1, 2, 0, 3, 10, 2, 5, 0, 5, 5, 0, 8, 7, 6, 8, 0
        ]),
        4.0
    );
    assert_eq!(
        Solution::trim_mean(vec![
            6, 0, 7, 0, 7, 5, 7, 8, 3, 4, 0, 7, 8, 1, 6, 8, 1, 1, 2, 4, 8, 1, 9, 5, 4, 3, 8, 5, 10,
            8, 6, 6, 1, 0, 6, 10, 8, 2, 3, 4
        ]),
        4.777777777777778
    );
}

struct Solution;
impl Solution {
    pub fn trim_mean(arr: Vec<i32>) -> f64 {
        let mut arr = arr;
        arr.sort();
        let n = arr.len();
        let five_percent = (n as f64 * 0.05) as usize;
        let sum: i32 = arr
            .iter()
            .skip(five_percent)
            .take(n - five_percent * 2)
            .sum();
        return sum as f64 / (n - five_percent * 2) as f64;
    }
}
