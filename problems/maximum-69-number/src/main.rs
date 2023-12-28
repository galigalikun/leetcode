fn main() {
    assert_eq!(Solution::maximum69_number(9669), 9969);
    assert_eq!(Solution::maximum69_number(9996), 9999);
    assert_eq!(Solution::maximum69_number(9999), 9999);
}

struct Solution;
impl Solution {
    pub fn maximum69_number(num: i32) -> i32 {
        let mut num = num;
        let mut digits = Vec::new();
        while num > 0 {
            digits.push(num % 10);
            num /= 10;
        }
        for i in (0..digits.len()).rev() {
            if digits[i] == 6 {
                digits[i] = 9;
                break;
            }
        }
        let mut num = 0;
        for i in (0..digits.len()).rev() {
            num = num * 10 + digits[i];
        }
        return num;
    }
}
