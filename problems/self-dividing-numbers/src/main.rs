fn main() {
    assert_eq!(
        Solution::self_dividing_numbers(1, 22),
        [1, 2, 3, 4, 5, 6, 7, 8, 9, 11, 12, 15, 22]
    );
    assert_eq!(Solution::self_dividing_numbers(47, 85), [48, 55, 66, 77]);
}

struct Solution {}
impl Solution {
    fn is_self_dividing(num: i32) -> bool {
        let mut n = num;
        while n > 0 {
            let digit = n % 10;
            if digit == 0 || num % digit != 0 {
                return false;
            }
            n /= 10;
        }
        return true;
    }
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res = vec![];
        for i in left..=right {
            if Solution::is_self_dividing(i) {
                res.push(i);
            }
        }
        return res;
    }
}
