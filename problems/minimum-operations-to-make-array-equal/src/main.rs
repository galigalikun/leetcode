fn main() {
    assert_eq!(Solution::min_operations(3), 2);
    assert_eq!(Solution::min_operations(6), 9);
}

struct Solution;
impl Solution {
    pub fn min_operations(n: i32) -> i32 {
        if n % 2 == 0 {
            return n * n / 4;
        }
        return (n / 2) * (n / 2 + 1);
    }
}
