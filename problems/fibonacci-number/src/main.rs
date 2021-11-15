fn main() {
    assert_eq!(Solution::fib(2), 1);
    assert_eq!(Solution::fib(3), 2);
    assert_eq!(Solution::fib(4), 3);
}

struct Solution {}
impl Solution {
    pub fn fib(n: i32) -> i32 {
        if n == 0 {
            return 0;
        } else if n == 1 {
            return 1;
        } else {
            return Solution::fib(n - 1) + Solution::fib(n - 2);
        }
    }
}
