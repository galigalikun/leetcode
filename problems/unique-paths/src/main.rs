fn main() {
    assert_eq!(Solution::unique_paths(3, 7), 28);
    assert_eq!(Solution::unique_paths(1, 1), 1);
    assert_eq!(Solution::unique_paths(1, 2), 1);
    assert_eq!(Solution::unique_paths(2, 2), 2);
    assert_eq!(Solution::unique_paths(3, 2), 3);
    assert_eq!(Solution::unique_paths(3, 3), 6);
    assert_eq!(Solution::unique_paths(4, 4), 20);
    assert_eq!(Solution::unique_paths(5, 5), 70);
    assert_eq!(Solution::unique_paths(10, 10), 48620);
    assert_eq!(Solution::unique_paths(4, 6), 56);
    assert_eq!(Solution::unique_paths(3, 7), 28);
}

struct Solution {}
// https://www.youtube.com/watch?v=GO5QHC_BmvM
impl Solution {
    pub fn unique_paths(m: i32, n: i32) -> i32 {
        let mut result = Vec::new();
        for i in 0..m as usize {
            result.push(vec![]);
            for j in 0..n as usize {
                if i == 0 || j == 0 {
                    result[i].push(1);
                } else {
                    let diff = result[i][j - 1] + result[i - 1][j];
                    result[i].push(diff);
                }
            }
        }
        return result[m as usize - 1][n as usize - 1];
    }
}
