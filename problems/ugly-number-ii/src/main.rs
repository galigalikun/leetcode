fn main() {
    assert_eq!(Solution::nth_ugly_number(10), 12);
    assert_eq!(Solution::nth_ugly_number(1), 1);
    assert_eq!(Solution::nth_ugly_number(1352), 402653184);
}

pub struct Solution {}
// https://programmerstart.com/article/30301428916/
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut result = vec![1];
        let mut i2 = 0;
        let mut i3 = 0;
        let mut i5 = 0;
        loop {
            let u2 = 2 * result[i2];
            let u3 = 3 * result[i3];
            let u5 = 5 * result[i5];
            let m = std::cmp::min(std::cmp::min(u2, u3), u5);
            if m == u2 {
                i2 += 1;
            }
            if m == u3 {
                i3 += 1;
            }
            if m == u5 {
                i5 += 1;
            }
            if result.len() >= n as usize {
                break;
            }
            result.push(m);
        }
        return *result.last().unwrap();
    }
}
