fn main() {
    assert_eq!(Solution::base_neg2(0), "0");
    assert_eq!(Solution::base_neg2(2), "110");
    assert_eq!(Solution::base_neg2(3), "111");
    assert_eq!(Solution::base_neg2(4), "100");
    assert_eq!(Solution::base_neg2(5), "101");
}

struct Solution;
impl Solution {
    pub fn base_neg2(n: i32) -> String {
        if n == 0 {
            return "0".to_string();
        }

        let mut value = n;
        let mut digits = Vec::new();

        while value != 0 {
            if value % 2 == 0 {
                digits.push('0');
                value /= -2;
            } else {
                digits.push('1');
                value = (value - 1) / -2;
            }
        }

        digits.reverse();
        digits.into_iter().collect()
    }
}
