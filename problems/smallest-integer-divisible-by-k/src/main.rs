fn main() {
    assert_eq!(Solution::smallest_repunit_div_by_k(1), 1);
    assert_eq!(Solution::smallest_repunit_div_by_k(2), -1);
    assert_eq!(Solution::smallest_repunit_div_by_k(3), 3);
}

struct Solution;
impl Solution {
    pub fn smallest_repunit_div_by_k(k: i32) -> i32 {
        let mut r = 0;
        for i in 1..=k {
            r = (r * 10 + 1) % k;
            if r == 0 {
                return i;
            }
        }
        return -1;
    }
}
