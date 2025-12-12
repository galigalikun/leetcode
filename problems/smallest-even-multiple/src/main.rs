fn main() {
    assert_eq!(Solution::smallest_even_multiple(5), 10);
    assert_eq!(Solution::smallest_even_multiple(6), 6);
}

struct Solution;
impl Solution {
    pub fn smallest_even_multiple(n: i32) -> i32 {
        if n % 2 == 0 { n } else { n * 2 }
    }
}
