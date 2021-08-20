fn main() {
    assert_eq!(Solution::find_nth_digit(3), 3);
    assert_eq!(Solution::find_nth_digit(11), 0);
    assert_eq!(Solution::find_nth_digit(12), 1);
    assert_eq!(Solution::find_nth_digit(15), 2);
    assert_eq!(Solution::find_nth_digit(1000000000), 1);
}

pub struct Solution {}
// https://evelynn.gitbooks.io/google-interview/content/nth_digit.html
impl Solution {
    pub fn find_nth_digit(n: i32) -> i32 {
        let mut len:i64 = 1;
        let mut count:i64 = 9;
        let mut start:i64 = 1;
        let mut num:i64 = n as i64;
        while num > len * count {
            num -= len * count;
            len += 1;
            count *= 10;
            start *= 10;
        }
        start += (num - 1) / len;
        return start
            .to_string()
            .chars()
            .nth(((num - 1) % len) as usize)
            .unwrap()
            .to_digit(10)
            .unwrap() as i32;
    }
}
