fn main() {
    assert_eq!(Solution::max_diff(555), 888);
    assert_eq!(Solution::max_diff(9), 8);
}

struct Solution;
impl Solution {
    pub fn max_diff(num: i32) -> i32 {
        let mut num = num;
        let mut max = num;
        let mut min = num;
        let mut max_digit = -1;
        let mut min_digit = -1;
        let mut digit = 0;
        while num > 0 {
            let d = num % 10;
            if max_digit == -1 && d != 9 {
                max_digit = d;
            }
            if min_digit == -1 && d != 1 && d != 0 {
                min_digit = d;
            }
            max += (9 - d) * 10i32.pow(digit);
            min += (d - 1) * 10i32.pow(digit);
            num /= 10;
            digit += 1;
        }
        if max_digit != -1 && min_digit != -1 {
            return max - min;
        }
        return 0;
    }
}
