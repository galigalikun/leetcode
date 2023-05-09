fn main() {
    assert_eq!(Solution::base_neg2(2), "110");
    assert_eq!(Solution::base_neg2(3), "111");
    assert_eq!(Solution::base_neg2(4), "100");
}

struct Solution;
impl Solution {
    pub fn base_neg2(n: i32) -> String {
        return format!("{:b}", n);
    }
}
