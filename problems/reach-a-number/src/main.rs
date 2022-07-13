fn main() {
    assert_eq!(Solution::reach_number(2), 3);
    assert_eq!(Solution::reach_number(3), 2);
    assert_eq!(Solution::reach_number(-1), 1);
    assert_eq!(Solution::reach_number(-2), 3);
}

struct Solution {}
impl Solution {
    pub fn reach_number(target: i32) -> i32 {
        let mut ans = 0;
        let mut step = 1;
        loop {
            if target == ans + step {
                return step;
            } else if target == ans - step {
                return step;
            } else if target > ans + step {
                ans += step;
            } else {
                ans -= step;
            }
            step += 1;
        }
    }
}
