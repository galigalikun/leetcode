fn main() {
    assert_eq!(Solution::bitwise_complement(5), 2);
    assert_eq!(Solution::bitwise_complement(7), 0);
    assert_eq!(Solution::bitwise_complement(10), 5);
}

struct Solution;
impl Solution {
    pub fn bitwise_complement(n: i32) -> i32 {
        let a = format!("{:b}", n);
        let mut b = String::new();
        for c in a.chars() {
            if c == '0' {
                b.push('1');
            } else {
                b.push('0');
            }
        }
        return i32::from_str_radix(&b, 2).unwrap();
    }
}
