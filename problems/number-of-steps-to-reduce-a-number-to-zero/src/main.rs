fn main() {
    assert_eq!(Solution::number_of_steps(14), 6);
    assert_eq!(Solution::number_of_steps(8), 4);
    assert_eq!(Solution::number_of_steps(123), 12);
}

struct Solution;
impl Solution {
    pub fn number_of_steps(num: i32) -> i32 {
        let mut num = num;
        let mut count = 0;
        while num > 0 {
            if num % 2 == 0 {
                num /= 2;
            } else {
                num -= 1;
            }
            count += 1;
        }
        return count;
    }
}
