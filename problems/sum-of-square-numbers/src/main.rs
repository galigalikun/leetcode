fn main() {
    assert_eq!(Solution::judge_square_sum(61), true);
    assert_eq!(Solution::judge_square_sum(5), true);
    assert_eq!(Solution::judge_square_sum(3), false);
    assert_eq!(Solution::judge_square_sum(2), true);
    assert_eq!(Solution::judge_square_sum(5487023), false);
}

struct Solution {}
impl Solution {
    pub fn judge_square_sum(c: i32) -> bool {
        let n = (c as f64).sqrt();
        for a in (0..=n as i32).rev() {
            for b in 0..=n as i32 {
                if a.pow(2) + b.pow(2) == c {
                    return true;
                }
            }
        }

        return false;
    }
}
