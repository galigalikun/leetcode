fn main() {
    assert_eq!(Solution::clumsy(4), 7);
    assert_eq!(Solution::clumsy(10), 12);
}

struct Solution;
impl Solution {
    pub fn clumsy(n: i32) -> i32 {
        let mut result = 0;
        let mut n = n;
        let mut sign = 1;
        while n > 0 {
            let mut temp = n;
            n -= 1;
            if n > 0 {
                temp *= n;
                n -= 1;
            }
            if n > 0 {
                temp /= n;
                n -= 1;
            }
            if n > 0 {
                temp += n;
                n -= 1;
            }
            result += sign * temp;
            sign = -1;
        }
        return result;
    }
}
