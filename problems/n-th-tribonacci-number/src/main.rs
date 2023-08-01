fn main() {
    assert_eq!(Solution::tribonacci(4), 4);
    assert_eq!(Solution::tribonacci(25), 1389537);
}

struct Solution;
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut a = 0;
        let mut b = 1;
        let mut c = 1;
        if n == 0 {
            return a;
        }
        if n == 1 {
            return b;
        }
        if n == 2 {
            return c;
        }
        for _ in 3..=n {
            let d = a + b + c;
            a = b;
            b = c;
            c = d;
        }
        return c;
    }
}
