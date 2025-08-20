fn main() {
    // assert_eq!(Solution::is_three(2), false);
    assert_eq!(Solution::is_three(4), true);
    assert_eq!(Solution::is_three(49), true);
}

struct Solution;
impl Solution {
    pub fn is_three(n: i32) -> bool {
        // Check if n is a perfect square and its square root is prime
        let sqrt = (n as f64).sqrt() as i32;
        for i in 2..=sqrt {
            if sqrt % i == 0 {
                return false;
            }
        }
        return n > 1 && (sqrt * sqrt == n) && (sqrt == 2 || sqrt == 3 || sqrt == 5);
    }
}
