fn main() {
    assert_eq!(Solution::num_moves_stones(1, 2, 5), vec![1, 2]);
    assert_eq!(Solution::num_moves_stones(4, 3, 2), vec![0, 0]);
    assert_eq!(Solution::num_moves_stones(3, 5, 1), vec![1, 2]);
}

struct Solution;
impl Solution {
    pub fn num_moves_stones(a: i32, b: i32, c: i32) -> Vec<i32> {
        let mut v = vec![a, b, c];
        v.sort();
        let min = if v[2] - v[1] == 1 && v[1] - v[0] == 1 {
            0
        } else if v[2] - v[1] <= 2 || v[1] - v[0] <= 2 {
            1
        } else {
            2
        };
        let max = v[2] - v[0] - 2;
        return vec![min, max];
    }
}
