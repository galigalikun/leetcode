fn main() {
    assert_eq!(Solution::maximum_score(2, 4, 6), 6);
    assert_eq!(Solution::maximum_score(4, 4, 6), 7);
    assert_eq!(Solution::maximum_score(1, 8, 8), 8);
}

struct Solution;
impl Solution {
    pub fn maximum_score(a: i32, b: i32, c: i32) -> i32 {
        let mut v = vec![a, b, c];
        v.sort();
        if v[0] + v[1] <= v[2] {
            return v[0] + v[1];
        }
        return (v[0] + v[1] + v[2]) / 2;
    }
}
