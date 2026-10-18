fn main() {
    assert_eq!(Solution::clumsy(4), 7);
    assert_eq!(Solution::clumsy(10), 12);
}

struct Solution;
impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let mut stack = vec![n];
        let mut op = 0;

        for x in (1..n).rev() {
            match op % 4 {
                0 => {
                    let top = stack.pop().unwrap();
                    stack.push(top * x);
                }
                1 => {
                    let top = stack.pop().unwrap();
                    stack.push(top / x);
                }
                2 => stack.push(x),
                _ => stack.push(-x),
            }
            op += 1;
        }

        stack.into_iter().sum()
    }
}
