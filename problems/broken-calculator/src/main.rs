fn main() {
    assert_eq!(Solution::broken_calc(2, 3), 2);
    assert_eq!(Solution::broken_calc(5, 8), 2);
    assert_eq!(Solution::broken_calc(3, 10), 3);
}

struct Solution;
impl Solution {
    pub fn broken_calc(start_value: i32, target: i32) -> i32 {
        let mut target = target;
        let mut count = 0;
        while target > start_value {
            if target % 2 == 0 {
                target /= 2;
            } else {
                target += 1;
            }
            count += 1;
        }
        return count + start_value - target;
    }
}
